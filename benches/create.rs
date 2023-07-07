// These benchmarks were lifted from ropey, to compare and sanity check our performance.
// Consequently, they should be considered as licensed under the MIT license.

extern crate criterion;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use braid::strand::Strand;

const TEXT_SMALL: &str = include_str!("data/small.txt");
const TEXT_MEDIUM: &str = include_str!("data/medium.txt");
const TEXT_LARGE: &str = include_str!("data/large.txt");
const TEXT_LF: &str = include_str!("data/lf.txt");


fn from_str(c: &mut Criterion) {
   let mut group = c.benchmark_group("from_str");

   group.bench_function("small", |bench| {
      bench.iter(|| {
         Strand::new_leaf(black_box(TEXT_SMALL));
      })
   });

   group.bench_function("medium", |bench| {
      bench.iter(|| {
         Strand::new_leaf(black_box(TEXT_MEDIUM));
      })
   });

   group.bench_function("large", |bench| {
      bench.iter(|| {
         Strand::new_leaf(black_box(TEXT_LARGE));
      })
   });

   group.bench_function("linefeeds", |bench| {
      bench.iter(|| {
         Strand::new_leaf(black_box(TEXT_LF));
      })
   });
}


fn rope_clone(c: &mut Criterion) {
   let rope = Strand::new_leaf(TEXT_LARGE);
   c.bench_function("rope_clone", |bench| {
      bench.iter(|| {
         let _ = black_box(&rope).clone();
      })
   });
}

criterion_group!(benches, from_str, rope_clone,);
criterion_main!(benches);
