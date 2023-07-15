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
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use xxhash_rust::xxh3::Xxh3;
use crate::index::{CharWidthBuilder, CharWidth};


// Quickly constructs test strand from string literals.
//  e.g. strand!("foo", "bar", "baz") -> ['foo'  ['bar'  baz']]
// Can also be nested to produce unbalanced trees, if required.
//  e.g. strand!("foo", strand!("bar, "baz"), "qux") -> ['foo'  [['bar'  'baz']  'qux']]
#[cfg(test)]
macro_rules! strand {
   ($s:literal) => { Strand::new_leaf($s) };
   ($( $l:expr, $r:expr ),+) => { strand!($( Strand::new_branch(strand!($l), strand!($r)) ),+) };
   ($x:expr, $( $l:expr, $r:expr ),+) => { strand!($x, $( strand!($l, $r) ),+) };
   ($e:expr) => { $e };
}


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
    index: CharWidth,
    length: usize,
    value: &'a str,
}

impl<'a> Strand<'a> {
   pub fn new_leaf(value: &'a str) -> Strand<'a> {
      let index = value.chars().collect::<CharWidthBuilder>().freeze();
      let length = index.count();
      Strand::Leaf(Rc::new(LeafNode{ index, length, value }))
   }

   pub fn new_branch(left: Strand<'a>, right: Strand<'a>) -> Strand<'a> {
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
            )} else {
               // TODO: clean up
               let (a, b) = leaf.split(i..=i);
               let a = Rc::new(a.unwrap());
               let b = Rc::new(b.unwrap());
               let c = Strand::new_leaf(s);
               (
                  Strand::Leaf(a),
                  Strand::Branch(Rc::new(BranchNode{length: c.length() + b.length, left: c, right: Strand::Leaf(b)})),
               )
            }
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
            }

            // Contained to left branch
            if i + n <= branch.left.length() {
               return Strand::new_branch(
                  branch.left.remove(i, n),
                  branch.right.clone(),
               );
            }

            // Contained to right branch
            if i >= branch.left.length() {
               // Discard right entirely
               if i == branch.left.length() && n >= branch.right.length() {
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
            match leaf.split(i..=(i + n)) {
               (Some(a), Some(b)) => {
                  return Strand::new_branch(
                     Strand::Leaf(Rc::new(a)),
                     Strand::Leaf(Rc::new(b)),
                  );
               },
               (Some(a), None) => {
                  return Strand::Leaf(Rc::new(a));
               },
               (None, Some(b)) => {
                  return Strand::Leaf(Rc::new(b));
               },
               _ => unreachable!("results in empty leaf"),
            }
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


   /// Return an iterator over the individual char's in the strand
   pub fn char_iter(&'a self) -> impl Iterator<Item=char> + 'a {
      return self.leaf_iter().flat_map(|x| x.value.chars());
   }


   // Return at iterator over the index of all occurences of a given char in the given range
   pub fn findchar_iter(&'a self, needle: char, from: usize, to: usize) -> impl Iterator<Item=usize> + 'a {
      return self.char_iter().skip(from).take(to - from).enumerate().filter_map(move |(i, x)| {
         if x == needle {
            return Some(i);
         }
         return None;
      });
   }


   // Return an iterator over all the lines in the strand (as String's)
   pub fn line_iter(&'a self) -> impl Iterator<Item = String> + 'a {
      let mut chars = self.char_iter().peekable();
      return iter::from_fn(move || {
         chars.peek() ?;
         return Some(chars.by_ref().take_while(|c| c != &'\n').collect::<String>());
      });
   }


   // Return an iterator over all the bytes that, ultimately, comprise the strand
   pub fn byte_iter(&'a self) -> impl Iterator<Item=u8> + 'a {
      return self.leaf_iter().flat_map(|x| x.value.bytes());
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
            write!(f, "[{:?}  {:?}]", branch.left, branch.right)
         },
         Strand::Leaf(leaf) => {
            write!(f, "'{}'", leaf.value)
         },
      }
   }
}


// Enable == operator and shit
impl Eq for Strand<'_> {}
impl PartialEq for Strand<'_> {
   fn eq(&self, other: &Self) -> bool {
      // TODO: precompute & store the hashes
      let mut hasher = Xxh3::new();
      self.hash(&mut hasher);
      let hash = hasher.digest();
      hasher.reset();
      other.hash(&mut hasher);
      return hash == hasher.digest() && self.byte_iter().eq(other.byte_iter());
   }
}
#[cfg(test)] #[test]
fn test_eq() {
    let st_1 = strand!("foo", "bar", strand!("baz", "qux"));
    assert_eq!(st_1, strand!("foo", "bar", strand!("baz", "qux")));
    assert_eq!(st_1, strand!("foo", strand!("bar", "baz"), "qux"));
    assert_eq!(st_1, strand!("foobarbazqux"));
}


// Strand should be hashable not only for common use cases like hash maps, but perhaps
// more interesting uses, like pinpointing where strands diverge?
// Note: this implementation treats the Strand as opaque, only caring about the underlying
//       string that it is storing.
impl Hash for Strand<'_> {
   fn hash<H: Hasher>(&self, state: &mut H) {
      self.byte_iter().for_each(|b| state.write_u8(b));
      // Kinda annoying hack to make it hash the same as an identical str...
      state.write_u8(0xff);
   }
}


impl LeafNode<'_> {
   // Convert a char/unicode index into a raw byte index for low level indexing/slicing/etc.
   fn byte_index(&self, i: usize) -> usize {
      self.index.byte_index(i)
   }

   fn split<T>(&self, r: T) -> (Option<Self>, Option<Self>) where T: std::ops::RangeBounds<usize> {
      let start = match r.start_bound() {
         std::ops::Bound::Unbounded => 0,
         std::ops::Bound::Included(&i) => i,
         _ => unimplemented!(),
      };
      let end = match r.end_bound() {
         std::ops::Bound::Unbounded => self.value.chars().count(),
         std::ops::Bound::Included(&i) => i,
         std::ops::Bound::Excluded(&i) => i - 1,
      };
      assert!(end <= self.length, "index out of bounds");
      return (
         match start {
            0 => None,
            s => Some(Self{ length: s, value: unsafe{ self.value.get_unchecked(..self.byte_index(s)) } }),
         },
         match end {
            e if e == self.length => None,
            e => Some(Self{ length: self.length - e, value: unsafe{ self.value.get_unchecked(self.byte_index(e)..) } }),
         },
      );
   }
}

#[cfg(test)] #[test]
fn test_split() {
   let leaf = LeafNode{ length: 6, value: &"abcⒶⒷⒸ" };
   let (a, b) = leaf.split(..);
   assert!(a.is_none());
   assert!(b.is_none());
   let (a, b) = leaf.split(..4);
   assert!(a.is_none());
   assert_eq!(b.unwrap().value, "ⒶⒷⒸ");
   let (a, b) = leaf.split(3..);
   assert_eq!(a.unwrap().value, "abc");
   assert!(b.is_none());
   let (a, b) = leaf.split(3..4);
   assert_eq!(a.unwrap().value, "abc");
   assert_eq!(b.unwrap().value, "ⒶⒷⒸ");
}


#[cfg(test)] #[test]
fn test_hash() {
   let st = strand!("foo", "bar", strand!("baz", "qux"));
   let mut h = Xxh3::new();
   st.hash(&mut h);
   let st_h = h.digest();

   let s = "foobarbazqux";
   let mut h = Xxh3::new();
   s.hash(&mut h);
   let s_h = h.digest();

   assert_eq!(st.to_string(), s);
   assert_eq!(st_h, s_h);
}



// Some lovely tests to try and catch regressions, etc.
#[cfg(test)]
mod tests {
   use super::*;
   use pretty_assertions::{assert_eq, /*assert_ne*/};


   #[test]
   fn test_macro() {
      let st = strand!("foo", strand!("bar", "baz"), "qux");
      assert_eq!(format!("{st}"), "foobarbazqux");
      assert_eq!(format!("{st:?}"), "['foo'  [['bar'  'baz']  'qux']]");
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


   // Test inserting into a leaf comprised of multi-byte characters
   #[test]
   fn test_insert_unicode() {
      let st = Strand::new_leaf("ⅠⅡⅢⅣ");
      assert_eq!(st.insert("-", 0).to_string(), "-ⅠⅡⅢⅣ");
      assert_eq!(st.insert("-", 2).to_string(), "ⅠⅡ-ⅢⅣ");
      assert_eq!(st.insert("-", 4).to_string(), "ⅠⅡⅢⅣ-");
   }


   // Test removing from a leaf comprised of multi-byte characters
   #[test]
   fn test_remove_unicode() {
      let st = Strand::new_leaf("ⅠⅡⅢⅣⅤ");
      assert_eq!(st.remove(0, 1).to_string(), "ⅡⅢⅣⅤ");
      assert_eq!(st.remove(1, 1).to_string(), "ⅠⅢⅣⅤ");
      assert_eq!(st.remove(2, 1).to_string(), "ⅠⅡⅣⅤ");
      assert_eq!(st.remove(3, 1).to_string(), "ⅠⅡⅢⅤ");
      assert_eq!(st.remove(4, 1).to_string(), "ⅠⅡⅢⅣ");
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
      let iter = st.char_iter().skip(4).take(9);
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
      let v = iter.collect::<Vec<String>>();
      println!("st: {st:?}");
      println!("lines: {:?}", v);
      assert_eq!(v, vec!["this", "text", "has", "a", "few", "newlines"]);
   }


   #[test]
   fn test_byte_iter() {
      let st = strand!("a", "b", strand!("c", "d"), "e", strand!("f", "g"));
      assert_eq!(st.byte_iter().collect::<Vec<u8>>(), vec![97, 98, 99, 100, 101, 102, 103]);
   }
}
