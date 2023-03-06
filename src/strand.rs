use std::rc::Rc;

#[derive(Clone, Debug)]
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
}


// 
#[cfg(test)]
mod tests {
   use super::*;
   use test::Bencher;
   use rand::{distributions::Alphanumeric, SeedableRng, Rng}; // 0.8
   use rand_xoshiro::Xoshiro256Plus;

   fn rand_string(n: usize) -> String {
      Xoshiro256Plus::seed_from_u64(13)
         .sample_iter(&Alphanumeric)
         .take(n)
         .map(char::from)
         .collect()
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

