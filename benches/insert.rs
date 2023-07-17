// These benchmarks were lifted from ropey, to compare and sanity check our performance.
// Consequently, they should be considered as licensed under the MIT license.
extern crate criterion;
extern crate rand;

use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use rand::random;
use braid::strand::Strand;
use braid::ranged::Ranged;

const TEXT: &str = include_str!("data/large");

fn bench_insert(c: &mut Criterion, src: &str, text: &str) {
   let mut g = c.benchmark_group(format!("insert"));
   g.sample_size(1000);
   g.noise_threshold(0.05);

   for (name, data) in [
      ("small", "a"),
      ("medium", "This is some text."),
      ("large", include_str!("data/small")),
   ].iter() {
      g.bench_with_input(BenchmarkId::new(format!("{src}/random"), name), &data, |b, data| {
         let mut rope = Strand::new_leaf(text);
         b.iter(|| {
            let len = rope.length();
            rope = rope.insert(data, random::<usize>() % len);
         });
      });
      g.bench_with_input(BenchmarkId::new(format!("{src}/start"), name), &data, |b, data| {
         let mut rope = Strand::new_leaf(text);
         b.iter(|| {
            rope = rope.insert(data, 0);
         });
      });
      g.bench_with_input(BenchmarkId::new(format!("{src}/middle"), name), &data, |b, data| {
         let mut rope = Strand::new_leaf(text);
         b.iter(|| {
            let len = rope.length();
            rope = rope.insert(data, len / 2);
         });
      });
      g.bench_with_input(BenchmarkId::new(format!("{src}/end"), name), &data, |b, data| {
         let mut rope = Strand::new_leaf(text);
         b.iter(|| {
            let len = rope.length();
            rope = rope.insert(data, len);
         });
      });
   }
}

fn insert(c: &mut Criterion) {
   bench_insert(c, "large", include_str!("data/large"));
   bench_insert(c, "cyrillic_1", include_str!("data/cyrillic_1"));
   bench_insert(c, "cyrillic_2", include_str!("data/cyrillic_2"));
}

fn insert_after_clone(c: &mut Criterion) {
   c.bench_function("insert_after_clone", |bench| {
      let rope = Strand::new_leaf(TEXT);
      let mut rope_clone = rope.clone();
      let mut i = 0;
      bench.iter(|| {
         if i > 32 {
            i = 0;
            rope_clone = rope.clone();
         }
         let len = rope_clone.length();
         rope_clone.insert("a", random::<usize>() % len);
         i += 1;
      })
   });
}

criterion_group!(
   benches,
   insert,
   insert_after_clone
);
criterion_main!(benches);
