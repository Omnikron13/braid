// These benchmarks were lifted from ropey, to compare and sanity check our performance.
// Consequently, they should be considered as licensed under the MIT license.
extern crate criterion;
extern crate rand;

use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use rand::random;
use braid::strand::Strand;

const TEXT: &str = include_str!("data/large");

fn mul_string_length(text: &str, n: usize) -> String {
   let mut mtext = String::new();
   for _ in 0..n {
      mtext.push_str(text);
   }
   mtext
}

fn remove(c: &mut Criterion) {
   let mut g = c.benchmark_group("remove");
   g.sample_size(1000);
   for (name, data, mult) in [
      ("small", 1, 1),
      ("medium", 15, 1),
      ("large", 1522, 4),
   ].iter() {
      g.bench_with_input(BenchmarkId::new("random", name), &data, |b, data| {
         let text = mul_string_length(TEXT, *mult as usize);
         let mut rope = Strand::new_leaf(&text);
         b.iter(|| {
            let len = rope.length();
            let start = random::<usize>() % (len - **data as usize);
            rope = rope.remove(start, **data as usize);
            if rope.length() <= TEXT.len() / 2 {
               rope = Strand::new_leaf(&text);
            }
         });
      });
      g.bench_with_input(BenchmarkId::new("start", name), &data, |b, data| {
         let text = mul_string_length(TEXT, *mult as usize);
         let mut rope = Strand::new_leaf(&text);
         b.iter(|| {
            let start = 0;
            rope = rope.remove(start, **data as usize);
            if rope.length() <= TEXT.len() / 2 {
               rope = Strand::new_leaf(&text);
            }
         });
      });
      g.bench_with_input(BenchmarkId::new("middle", name), &data, |b, data| {
         let text = mul_string_length(TEXT, *mult as usize);
         let mut rope = Strand::new_leaf(&text);
         b.iter(|| {
            let len = rope.length();
            let start = len / 2;
            rope = rope.remove(start, **data as usize);
            if rope.length() <= TEXT.len() / 2 {
               rope = Strand::new_leaf(&text);
            }
         });
      });
      g.bench_with_input(BenchmarkId::new("end", name), &data, |b, data| {
         let text = mul_string_length(TEXT, *mult as usize);
         let mut rope = Strand::new_leaf(&text);
         b.iter(|| {
            let len = rope.length();
            let start = len - **data as usize;
            rope = rope.remove(start, **data as usize);
            if rope.length() <= TEXT.len() / 2 {
               rope = Strand::new_leaf(&text);
            }
         });
      });
   }
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
   remove,
   remove_initial_after_clone
);
criterion_main!(benches);
