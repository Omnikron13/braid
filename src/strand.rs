// Copyright 2023 Joey Sabey <joey.sabey@gmx.com>
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.

// This file contains the core implementation of the rope data structure.
// It is declared here as 'Strand', with the intention of being a relatively 'pure' implementation
// of the data structure, agnostic of details such as where it's consituent strings are buffered,
// whether it is a subset of a larger rope, etc.
use std::fmt;
use std::iter;
use std::rc::Rc;

type BoxedLeafIterator<'a> = Box<dyn Iterator<Item = Rc<LeafNode<'a>>> + 'a>;

#[derive(Clone)]
pub enum Strand<'a> {
    Branch(Rc<BranchNode<'a>>),
    Leaf(Rc<LeafNode<'a>>),
}

#[derive(Debug)]
pub struct BranchNode<'a> {
    length: usize,
    left: Strand<'a>,
    right: Strand<'a>,
}

#[derive(Debug)]
pub struct LeafNode<'a> {
    length: usize,
    value: &'a str,
}

impl<'a> Strand<'a> {
   pub fn new_leaf(s: &'a str) -> Strand<'a> {
      Strand::Leaf(Rc::new(LeafNode{length: s.chars().count(), value: s}))
   }

   fn new_branch(left: Strand<'a>, right: Strand<'a>) -> Strand<'a> {
      Strand::Branch(Rc::new(BranchNode { length: left.length() + right.length(), left, right }))
   }

   // Passthrough to the shared (char) length of the node
   pub fn length(&self) -> usize {
      match self {
         Strand::Branch(b) => b.length,
         Strand::Leaf(l) => l.length,
      }
   }


   // Insert string at given (char) index
   pub fn insert(&self, s: &'a str, i: usize) -> Strand<'a> {
      // Short circuit for the edge case of an 'empty' leaf
      if self.length() == 0 {
         return Strand::new_leaf(s);
      }

      let (left, right) = match self {
         Strand::Branch(branch) => {
            if i < branch.left.length() {(
               branch.left.insert(s, i),
               branch.right.clone(),
            )} else {(
               branch.left.clone(),
               branch.right.insert(s, i - branch.left.length()),
            )}
         },
         Strand::Leaf(leaf) => {
            if i == 0 {(
               Strand::new_leaf(s),
               Strand::Leaf(leaf.clone()),
            )} else if i == leaf.length {(
               Strand::Leaf(leaf.clone()),
               Strand::new_leaf(s),
            )} else {(
               Strand::new_leaf(unsafe {leaf.value.get_unchecked(0..i)}),
               Strand::new_branch(
                  Strand::new_leaf(s),
                  Strand::new_leaf(unsafe {leaf.value.get_unchecked(i..leaf.length)}),
               ),
            )}
         },
      };

      Strand::new_branch(left, right)
   }


   // Remove substring of length n starting at (char) index i
   pub fn remove(&self, i: usize, n: usize) -> Strand<'a> {
      assert!(i < self.length(), "Index out of bounds");
      assert!(i + n <= self.length(), "Strand doesn't have enough characters");

      // Short circuit for clearing entire strand
      if n == self.length() {
         return Strand::new_leaf("");
      }

      match self {
         // Trim branch
         Strand::Branch(branch) => {
            // Trim head
            if i == 0 {
               // Discard left entirely
               if n == branch.left.length() {
                  return branch.right.clone();
               }
               // ...and trim the head of right
               if n > branch.left.length() {
                  return branch.right.remove(0, n - branch.left.length());
               }

               // Trim head of left (right unchanged)
               return Strand::new_branch(
                  branch.left.remove(0, n),
                  branch.right.clone(),
               );
            }

            // Trim tail/inner trim
            if i >= branch.left.length() {
               // Drop right
               if n == branch.right.length() {
                  return branch.left.clone()
               }

               // Trim/split tail
               return Strand::new_branch(
                  branch.left.clone(),
                  branch.right.remove(i - branch.left.length(), n),
               );
            }

            // Full split
            return Strand::new_branch(
               branch.left.remove(i, branch.left.length() - i),
               branch.right.remove(0, n - (branch.left.length() - i)),
            );
         },

         // Head/Tail/Split
         Strand::Leaf(leaf) => {
            if i == 0 {
               return Strand::new_leaf(unsafe {leaf.value.get_unchecked(n..leaf.length)});
            }
            if i + n == leaf.length {
               return Strand::new_leaf(unsafe {leaf.value.get_unchecked(0..i)});
            }
            return Strand::new_branch(
               Strand::new_leaf(unsafe {leaf.value.get_unchecked(0..i)}),
               Strand::new_leaf(unsafe {leaf.value.get_unchecked(i + n..leaf.length)}),
            );
         },
      }
   }


   // Return an iterator over the leaf nodes
   fn leaf_iter(&'a self) -> BoxedLeafIterator {
      match self {
         Strand::Branch(branch) => {
            Box::new(branch.left.leaf_iter().chain(branch.right.leaf_iter()))
         },
         Strand::Leaf(leaf) => {
            Box::new(iter::once(leaf.clone()))
         },
      }
   }


   /// Return an iterator over the given [char] range
   fn char_iter(&'a self, mut index: usize, mut length: usize) -> impl Iterator<Item=char> + 'a {
      return self.leaf_iter().filter_map(move |x| {
         // Short circuit out strands past the strand containing the end index
         if length == 0 { return None };

         // If the current strand does not contain the start index, filter it out and adjust the index
         if index >= x.length {
            index -= x.length;
            return None;
         }

         // TODO: improve readability...
         let i = index;
         index = 0;
         let n = std::cmp::min(x.length, length + i + 1);
         length -= std::cmp::min(length, x.length);
         return Some(unsafe { x.value.get_unchecked(i..n) });
      })
      // TODO: inline?
      .map(|x| x.chars())
      .flatten();
   }


   // Return at iterator over the index of all occurences of a given char in the given range
   fn findchar_iter(&'a self, needle: char, from: usize, to: usize) -> impl Iterator<Item=usize> + 'a {
      // Note that currently char_iter takes start + length, and this takes start + end...
      return self.char_iter(from, to - from).enumerate().filter_map(move |(i, x)| {
         if x == needle {
            return Some(i);
         }
         return None;
      });
   }


   // Return an iterator over all the lines in the strand
   // TODO: try to do this with e.g. take_while() (may need an actual iter class..?)
   fn line_iter(&'a self) -> impl Iterator<Item = Box<impl Iterator<Item = char> + 'a>> + 'a {
      return self.findchar_iter('\n', 0, self.length())
         .chain(iter::once(self.length()))
         .scan(0, |prev, x| {
            let boxed = Box::new(self.char_iter(*prev, x - *prev).filter(|&c| c != '\n'));
            *prev = x;
            return Some(boxed);
         });
   }


   // TODO: rename, or remove, or rework into a more general iterator?
   // Return an iterator over all leaf nodes which overlap a given char range
   fn skip_iter(&'a self, mut y: usize, mut z: usize) -> BoxedLeafIterator {
      // Short-circuit flag if the end index has already been filtered
      let mut end = false;

      Box::new(self.leaf_iter().filter(move |x| {
         // Short circuit out strands past the strand containing the end index
         if end { return false };

         // If the current strand does not contain the start index, filter it out abd adjust the index
         if y >= x.length {
            y -= x.length;
            return false;
         }

         // If the current strand does not contain the end index, filter it out and adjust the index
         if z + y >= x.length {
            z -= x.length;
            return true;
         }

         // If the logic reaches this point, the end index is within the current strand.
         // The next iteration will read the new end flag and filter out the rest of the strands.
         end = true;
         return true;
      }))
   }
}


// 'Ropes' are basically just a particular string implementation, thus should simply display
// their contiguous character representation as default output/formatting.
impl fmt::Display for Strand<'_> {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match self {
         Strand::Branch(branch) => {
            write!(f, "{}{}", branch.left, branch.right)
         },
         Strand::Leaf(leaf) => {
            write!(f, "{}", leaf.value)
         },
      }
   }
}


// Condensed debug output format, which should still be pretty easy to pick apart when needed.
impl fmt::Debug for Strand<'_> {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match self {
         Strand::Branch(branch) => {
            write!(f, "[ {:?} : {:?} ]", branch.left, branch.right)
         },
         Strand::Leaf(leaf) => {
            write!(f, "'{}'", leaf.value)
         },
      }
   }
}


// Some lovely tests to try and catch regressions, etc.
#[cfg(test)]
mod tests {
   use super::*;
   use test::Bencher;
   use rand::{distributions::Alphanumeric, SeedableRng, Rng}; // 0.8
   use rand_xoshiro::Xoshiro256Plus;

   // Generate a random string of length n characters
   fn rand_string(n: usize) -> String {
      Xoshiro256Plus::seed_from_u64(13)
         .sample_iter(&Alphanumeric)
         .take(n)
         .map(char::from)
         .collect()
   }


   #[test]
   fn test_to_string() {
      // TODO: should probably create a bettter setup function/macro for this kind of test
      let st = Strand::new_branch(
         Strand::new_branch(
            Strand::new_leaf("[[["),
            Strand::new_leaf("=foo"),
         ),
         Strand::new_branch(
            Strand::new_leaf(":~:"),
            Strand::new_branch(
               Strand::new_leaf("bar="),
               Strand::new_leaf("]]]"),
            ),
         ),
      );
      assert_eq!(st.to_string(), "[[[=foo:~:bar=]]]");
   }


   #[test]
   fn test_insert_trivial() {
      let s = "Hello, world!";
      let n = Strand::new_leaf(s);
      let n = n.insert(" ", 5);
      let n = n.insert(" ", 7);
      let n = n.insert(" ", 12);
      let n = n.insert(" ", 14);
      println!("{:?}", n);
   }


   // Sweep a removal of 2, 4, 8, and 16 chars across a reasonably nested test strand
   #[test]
   fn test_remove_sweep() -> Result<(), String> {
      let st = Strand::new_branch(
         Strand::new_branch(
            Strand::new_branch(
               Strand::new_leaf("Aa01"),
               Strand::new_leaf("Bb23"),
            ),
            Strand::new_branch(
               Strand::new_leaf("Cc45"),
               Strand::new_leaf("Dd67"),
            )
         ),
         Strand::new_branch(
            Strand::new_branch(
               Strand::new_leaf("Ee89"),
               Strand::new_leaf("Ff01"),
            ),
            Strand::new_branch(
               Strand::new_leaf("Gg23"),
               Strand::new_leaf("Hh45"),
            )
         )
      );

      let canon = String::from("Aa01Bb23Cc45Dd67Ee89Ff01Gg23Hh45");

      for x in [2, 4, 8, 16] {
         for i in 0..(34-x)/2 {
            let r = st.clone().remove(i*2, x);
            println!("remove {:0>2}-{x:0>2}: {r:?}", i*2);
            // TODO: use the proper error message thingy
            assert_eq!(format!("{r}"), format!("{}{}", &canon[0..i*2], &canon[(i*2)+x..canon.len()]));
         }
      }

      return Ok(());
   }


   // Remove all characters from leaf node
   #[test]
   fn test_remove_all_leaf() -> Result<(), String> {
      let n = Strand::new_leaf("1234");
      println!("Removing all: {:?}", n);
      let n = n.remove(0, 4);
      println!("Removing all: {:?}", n);
      assert_eq!(n.length(), 0, "Strand should be empty");
      match n {
         Strand::Leaf(_) => Ok(()),
         _default => Err(String::from("Strand should be a leaf")),
      }
   }

   // Remove all characters from branch node
   #[test]
   fn test_remove_all_branch() -> Result<(), String> {
      let n = Strand::new_branch(Strand::new_leaf("foo"), Strand::new_leaf("bar"));
      println!("Removing all: {:?}", n);
      let n = n.remove(0, 6);
      println!("Removing all: {:?}", n);
      assert_eq!(n.length(), 0, "Strand should be empty");
      match n {
         Strand::Leaf(_) => Ok(()),
         _ => Err(String::from("Strand should be a leaf")),
      }
   }

   // Test remove drop left branch
   #[test]
   fn test_remove_drop_left() {
      let n = Strand::new_branch(Strand::new_leaf("foo"), Strand::new_leaf("bar"));
      println!("Dropping left: {:?}", n);
      let n = n.remove(0, 3);
      println!("Dropping left: {:?}", n);
      match n {
         Strand::Leaf(l) => {
            assert_eq!(l.value, "bar");
         },
         _ => panic!("Strand should be a leaf"),
      }
   }

   // Test remove drop right branch
   #[test]
   fn test_remove_drop_right() {
      let n = Strand::new_branch(Strand::new_leaf("foo"), Strand::new_leaf("bar"));
      println!("Dropping right: {:?}", n);
      let n = n.remove(3, 3);
      println!("Dropping right: {:?}", n);
      match n {
         Strand::Leaf(l) => {
            assert_eq!(l.value, "foo");
         },
         _ => panic!("Strand should be a leaf"),
      }
   }

   // Test trim head of leaf
   #[test]
   fn test_remove_trim_head_leaf() {
      let n = Strand::new_leaf("foobar");
      println!("Trimming head: {:?}", n);
      let n = n.remove(0, 3);
      println!("Trimming head: {:?}", n);
      match n {
         Strand::Leaf(leaf) => {
            assert_eq!(leaf.value, "bar");
         },
         _ => panic!("Strand should be a leaf"),
      }
   }

   // Test trim tail of leaf
   #[test]
   fn test_remove_trim_tail_leaf() {
      let n = Strand::new_leaf("foobar");
      println!("Trimming tail: {:?}", n);
      let n = n.remove(3, 3);
      println!("Trimming tail: {:?}", n);
      match n {
         Strand::Leaf(leaf) => {
            assert_eq!(leaf.value, "foo");
         },
         _ => panic!("Strand should be a leaf"),
      }
   }

   // Test remove full split of leaf
   #[test]
   fn test_remove_full_split_leaf() {
      let n = Strand::new_leaf("foo_bar");
      println!("Full split: {:?}", n);
      let n = n.remove(3, 1);
      println!("Full split: {:?}", n);
      match n {
         Strand::Branch(branch) => {
            match (&branch.left, &branch.right) {
               (Strand::Leaf(left), Strand::Leaf(right)) => {
                  assert_eq!(left.value, "foo");
                  assert_eq!(right.value, "bar");
               },
               _ => panic!("New left & right segments should be leaves"),
            }
         },
         _ => panic!("Strand should be a branch"),
      }
   }

   // Test trim head of branch
   #[test]
   fn test_remove_trim_head_branch() {
      let n = Strand::new_branch(Strand::new_leaf("foo_"), Strand::new_leaf("bar"));
      println!("Trimming head: {:?}", n);
      let n = n.remove(0, 4);
      println!("Trimming head: {:?}", n);
      match n {
         Strand::Leaf(leaf) => {
            assert_eq!(leaf.value, "bar");
         },
         _ => panic!("Strand should be a leaf"),
      }
   }

   // Test trim tail of branch
   #[test]
   fn test_remove_trim_tail_branch() {
      let n = Strand::new_branch(Strand::new_leaf("foo"), Strand::new_leaf("_bar"));
      println!("Trimming tail: {:?}", n);
      let n = n.remove(3, 4);
      println!("Trimming tail: {:?}", n);
      match n {
         Strand::Leaf(leaf) => {
            assert_eq!(leaf.value, "foo");
         },
         _ => panic!("Strand should be a leaf"),
      }
   }

   // Test remove full split of branch
   #[test]
   fn test_remove_full_split_branch() {
      let n = Strand::new_branch(Strand::new_leaf("foo_"), Strand::new_leaf("_bar"));
      println!("Full split: {:?}", n);
      let n = n.remove(3, 2);
      println!("Full split: {:?}", n);
      match n {
         Strand::Branch(branch) => {
            match (&branch.left, &branch.right) {
               (Strand::Leaf(left), Strand::Leaf(right)) => {
                  assert_eq!(left.value, "foo");
                  assert_eq!(right.value, "bar");
               },
               _ => panic!("New left & right segments should be leaves"),
            }
         },
         _ => panic!("Strand should be a branch"),
      }
   }

   // Test trim tail of left side of branch
   #[test]
   fn test_remove_inner_trim_branch_left() {
      let n = Strand::new_branch(Strand::new_leaf("foo_"), Strand::new_leaf("bar"));
      println!("Inner trim: {:?}", n);
      let n = n.remove(3, 1);
      println!("Inner trim: {:?}", n);
      match n {
         Strand::Branch(branch) => {
            match (&branch.left, &branch.right) {
               (Strand::Leaf(left), Strand::Leaf(right)) => {
                  assert_eq!(left.value, "foo");
                  assert_eq!(right.value, "bar");
               },
               _ => panic!("New left & right segments should be leaves"),
            }
         },
         _ => panic!("Strand should be a branch"),
      }
   }

   // Test trim head of right side of branch
   #[test]
   fn test_remove_inner_trim_branch_right() {
      let n = Strand::new_branch(Strand::new_leaf("foo"), Strand::new_leaf("_bar"));
      println!("Inner trim: {:?}", n);
      let n = n.remove(3, 1);
      println!("Inner trim: {:?}", n);
      match n {
         Strand::Branch(branch) => {
            match (&branch.left, &branch.right) {
               (Strand::Leaf(left), Strand::Leaf(right)) => {
                  assert_eq!(left.value, "foo");
                  assert_eq!(right.value, "bar");
               },
               _ => panic!("New left & right segments should be leaves"),
            }
         },
         _ => panic!("Strand should be a branch"),
      }
   }

   // Test leaf_iter()
   #[test]
   fn test_leaf_iterator() {
      let st = Strand::new_branch(
         Strand::new_branch(
            Strand::new_leaf("["),
            Strand::new_leaf("foo"),
         ),
         Strand::new_branch(
            Strand::new_leaf(":"),
            Strand::new_branch(
               Strand::new_leaf("bar"),
               Strand::new_leaf("]"),
            ),
         ),
      );
      println!("{st:#?}");
      let iter = st.leaf_iter();
      assert_eq!(iter.map(|leaf| leaf.as_ref().value).collect::<String>(), "[foo:bar]");
   }

   // Test skip_iter(), which iterates all leaves which contain any part of a given char range
   #[test]
   fn test_skip_iterator() {
      let st = Strand::new_branch(
         Strand::new_branch(
            Strand::new_leaf("[[["),
            Strand::new_leaf("=foo"),
         ),
         Strand::new_branch(
            Strand::new_leaf(":~:"),
            Strand::new_branch(
               Strand::new_leaf("bar="),
               Strand::new_leaf("]]]"),
            ),
         ),
      );
      println!("full strand: {st:#?}");
      println!("as string: {}", st.leaf_iter().map(|leaf| leaf.as_ref().value).collect::<String>());
      let iter = st.skip_iter(4, 9);
      assert_eq!(iter.map(|leaf| leaf.as_ref().value).collect::<String>(), "=foo:~:bar=");
   }

   // Test char_iter(), which iterates over the [char]s in the given range
   #[test]
   fn test_char_iterator() {
      let st = Strand::new_branch(
         Strand::new_branch(
            Strand::new_leaf("[[["),
            Strand::new_leaf("=foo"),
         ),
         Strand::new_branch(
            Strand::new_leaf(":~:"),
            Strand::new_branch(
               Strand::new_leaf("bar="),
               Strand::new_leaf("]]]"),
            ),
         ),
      );
      println!("full strand: {st:#?}");
      println!("as string: {}", st.leaf_iter().map(|leaf| leaf.as_ref().value).collect::<String>());
      let iter = st.char_iter(4, 9);
      //for (i, c) in iter.enumerate() {
      //    println!("char[{}]: {:?}", i, c);
      //}
      assert_eq!(iter.collect::<String>(), "foo:~:bar");
   }


   // TODO: add sub-tests for more complex usage?
   #[test]
   fn test_findchar_iter() {
      let st = Strand::new_leaf("this\ntext\nhas\nnewlines");
      let iter = st.findchar_iter('\n', 0, st.length());
      let v = iter.collect::<Vec<usize>>();
      println!("st: {st:?}");
      println!("newlines at: {:?}", v);
      assert_eq!(v, vec![4, 9, 13]);
   }


   #[test]
   fn test_line_iter() {
      let st = Strand::new_leaf("this\ntext\nhas\na\nfew\nnewlines");
      let iter = st.line_iter();
      let v = iter.map(|l| l.collect::<String>()).collect::<Vec<String>>();
      println!("st: {st:?}");
      println!("lines: {:?}", v);
      assert_eq!(v, vec!["this", "text", "has", "a", "few", "newlines"]);
   }



   // Bench remove drop left branch
   #[bench]
   fn bench_remove_drop_left(b: &mut Bencher) {
      let n = Strand::new_branch(Strand::new_leaf("foo"), Strand::new_leaf("bar"));
      b.iter(|| for _ in 0..1000 {n.remove(0, 3);})
   }


   // Length of random string to insert into for benchmarking (128kb)
   const TEST_STRING_LEN: usize = 1024 * 128;
   // Number of times to insert a space at a random index
   const NUMBER_OF_INSERTS: usize = 1024;

   // Benchmark inserting into normal rust String
   #[bench]
   fn bench_insert_string(b: &mut Bencher) {
      let s = rand_string(TEST_STRING_LEN);
      let mut rng = Xoshiro256Plus::seed_from_u64(13);
      b.iter(|| {
         let mut s = s.clone();
         for _ in 0..NUMBER_OF_INSERTS {
            s.insert_str(rng.gen_range(0..s.len()), " ");
         }
      });
   }

   // Benchmark inserting into Strand
   #[bench]
   fn bench_insert_strand(b: &mut Bencher) {
      let s = rand_string(TEST_STRING_LEN);
      let s = Strand::new_leaf(s.as_str());
      let mut rng = Xoshiro256Plus::seed_from_u64(13);
      b.iter(|| {
         let mut sc = s.clone();
         for _ in 0..NUMBER_OF_INSERTS {
            sc = sc.insert(" ", rng.gen_range(0..s.length()));
         }
      });
   }
}

