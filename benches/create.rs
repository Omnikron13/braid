// These benchmarks were lifted from ropey, to compare and sanity check our performance.
// Consequently, they should be considered as licensed under the MIT license.

extern crate criterion;

use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use criterion::black_box;
use braid::strand::Strand;


fn from_str(c: &mut Criterion) {
   let mut g = c.benchmark_group("from_str");
   g.sample_size(1000);

   for (name, data) in [
      ("small", include_str!("data/small")),
      ("medium", include_str!("data/medium")),
      ("large", include_str!("data/large")),
      ("linefeeds", include_str!("data/lf")),
   ].iter() {
      g.bench_with_input(BenchmarkId::new("", name), &data, |b, data| {
         b.iter(|| {
            Strand::new_leaf(data);
         });
      });
   }
}


const TEXT_LARGE: &str = include_str!("data/large");

fn rope_clone(c: &mut Criterion) {
   let rope = Strand::new_leaf(TEXT_LARGE);
   c.bench_function("rope_clone", |bench| {
      bench.iter(|| {
         let _ = black_box(&rope).clone();
      })
   });
}

criterion_group!(
   benches,
   from_str,
   rope_clone,
);
criterion_main!(benches);
