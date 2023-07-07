// These benchmarks were lifted from ropey, to compare and sanity check our performance.
// Consequently, they should be considered as licensed under the MIT license.

extern crate criterion;
extern crate rand;

use criterion::{criterion_group, criterion_main, Criterion};
use rand::random;
use braid::strand::Strand;

const TEXT: &str = include_str!("large.txt");
const TEXT_SMALL: &str = include_str!("small.txt");

fn mul_string_length(text: &str, n: usize) -> String {
   let mut mtext = String::new();
   for _ in 0..n {
      mtext.push_str(text);
   }
   mtext
}


const LEN_MUL_SMALL: usize = 1;

fn remove_small(c: &mut Criterion) {
   let mut group = c.benchmark_group("remove_small");

   group.bench_function("random", |bench| {
      let text = mul_string_length(TEXT, LEN_MUL_SMALL);
      let mut rope = Strand::new_leaf(&text);

      bench.iter(|| {
         let len = rope.length();
         let start = random::<usize>() % len;
         rope = rope.remove(start, 1);

         if rope.length() <= TEXT.len() / 2 {
            rope = Strand::new_leaf(&text);
         }
      })
   });

   group.bench_function("start", |bench| {
      let text = mul_string_length(TEXT, LEN_MUL_SMALL);
      let mut rope = Strand::new_leaf(&text);

      bench.iter(|| {
         let _len = rope.length();
         let start = 0;
         rope = rope.remove(start, 1);

         if rope.length() <= TEXT.len() / 2 {
            rope = Strand::new_leaf(&text);
         }
      })
   });

   group.bench_function("middle", |bench| {
      let text = mul_string_length(TEXT, LEN_MUL_SMALL);
      let mut rope = Strand::new_leaf(&text);

      bench.iter(|| {
         let len = rope.length();
         let start = len / 2;
         rope = rope.remove(start, 1);

         if rope.length() <= TEXT.len() / 2 {
            rope = Strand::new_leaf(&text);
         }
      })
   });

   group.bench_function("end", |bench| {
      let text = mul_string_length(TEXT, LEN_MUL_SMALL);
      let mut rope = Strand::new_leaf(&text);

      bench.iter(|| {
         let len = rope.length();
         let start = len - 1;
         rope = rope.remove(start, 1);

         if rope.length() <= TEXT.len() / 2 {
            rope = Strand::new_leaf(&text);
         }
      })
   });
}


const LEN_MUL_MEDIUM: usize = 1;

fn remove_medium(c: &mut Criterion) {
   const MED_LEN: usize = 15;
   let mut group = c.benchmark_group("remove_medium");

   group.bench_function("random", |bench| {
      let text = mul_string_length(TEXT, LEN_MUL_MEDIUM);
      let mut rope = Strand::new_leaf(&text);

      bench.iter(|| {
         let len = rope.length();
         let start = random::<usize>() % (len - (MED_LEN-1));
         rope = rope.remove(start, MED_LEN);

         if rope.length() <= TEXT.len() / 2 {
            rope = Strand::new_leaf(&text);
         }
      })
   });

   group.bench_function("start", |bench| {
      let text = mul_string_length(TEXT, LEN_MUL_MEDIUM);
      let mut rope = Strand::new_leaf(&text);

      bench.iter(|| {
         let _len = rope.length();
         let start = 0;
         rope = rope.remove(start, MED_LEN);

         if rope.length() <= TEXT.len() / 2 {
            rope = Strand::new_leaf(&text);
         }
      })
   });

   group.bench_function("middle", |bench| {
      let text = mul_string_length(TEXT, LEN_MUL_MEDIUM);
      let mut rope = Strand::new_leaf(&text);

      bench.iter(|| {
         let len = rope.length();
         let start = len / 2;
         rope = rope.remove(start, MED_LEN);

         if rope.length() <= TEXT.len() / 2 {
            rope = Strand::new_leaf(&text);
         }
      })
   });

   group.bench_function("end", |bench| {
      let text = mul_string_length(TEXT, LEN_MUL_MEDIUM);
      let mut rope = Strand::new_leaf(&text);

      bench.iter(|| {
         let len = rope.length();
         let start = len - MED_LEN;
         rope = rope.remove(start, MED_LEN);

         if rope.length() <= TEXT.len() / 2 {
            rope = Strand::new_leaf(&text);
         }
      })
   });
}


const LEN_MUL_LARGE: usize = 4;

fn remove_large(c: &mut Criterion) {
   const LARGE_LEN: usize = TEXT_SMALL.len();
   let mut group = c.benchmark_group("remove_large");

   group.bench_function("random", |bench| {
      let text = mul_string_length(TEXT, LEN_MUL_LARGE);
      let mut rope = Strand::new_leaf(&text);

      bench.iter(|| {
         let len = rope.length();
         let start = random::<usize>() % (len - (LARGE_LEN-1));
         rope = rope.remove(start, LARGE_LEN);

         if rope.length() <= TEXT.len() / 2 {
            rope = Strand::new_leaf(&text);
         }
      })
   });

   group.bench_function("start", |bench| {
      let text = mul_string_length(TEXT, LEN_MUL_LARGE);
      let mut rope = Strand::new_leaf(&text);

      bench.iter(|| {
         let _len = rope.length();
         let start = 0;
         rope = rope.remove(start, LARGE_LEN);

         if rope.length() <= TEXT.len() / 2 {
            rope = Strand::new_leaf(&text);
         }
      })
   });

   group.bench_function("middle", |bench| {
      let text = mul_string_length(TEXT, LEN_MUL_LARGE);
      let mut rope = Strand::new_leaf(&text);

      bench.iter(|| {
         let len = rope.length();
         let start = len / 2;
         rope = rope.remove(start, LARGE_LEN);

         if rope.length() <= TEXT.len() / 2 {
            rope = Strand::new_leaf(&text);
         }
      })
   });

   group.bench_function("end", |bench| {
      let text = mul_string_length(TEXT, LEN_MUL_LARGE);
      let mut rope = Strand::new_leaf(&text);

      bench.iter(|| {
         let len = rope.length();
         let start = len - LARGE_LEN;
         rope = rope.remove(start, LARGE_LEN);

         if rope.length() <= TEXT.len() / 2 {
            rope = Strand::new_leaf(&text);
         }
      })
   });
}


fn remove_initial_after_clone(c: &mut Criterion) {
   c.bench_function("remove_initial_after_clone", |bench| {
      let rope = Strand::new_leaf(TEXT);
      let mut rope_clone = rope.clone();
      let mut i = 0;
      bench.iter(|| {
         if i > 32 {
            i = 0;
            rope_clone = rope.clone();
         }
         let len = rope_clone.length();
         let start = random::<usize>() % len;
         rope_clone = rope_clone.remove(start, 1);
         i += 1;
      })
   });
}

criterion_group!(
   benches,
   remove_small,
   remove_medium,
   remove_large,
   remove_initial_after_clone
);
criterion_main!(benches);
