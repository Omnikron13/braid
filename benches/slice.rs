extern crate criterion;

use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use criterion::black_box;
use rand::random;
use braid::strand::Strand;
use braid::ranged::Ranged;
use braid::sliceable::Sliceable;

fn slice_char_width(c: &mut Criterion) {
   let mut g = c.benchmark_group("slice_char_width");
   g.sample_size(1000);

   for (name, data) in [
      ("small", include_str!("data/small")),
      ("medium", include_str!("data/medium")),
      ("large", include_str!("data/large")),
      ("cyrillic_1", include_str!("data/cyrillic_1")),
      ("cyrillic_2", include_str!("data/cyrillic_2")),
   ].iter() {
      g.bench_with_input(BenchmarkId::new("", name), &data, |b, data| {
         b.iter(|| {
            let s = Strand::new_leaf(data);
            match s {
               Strand::Leaf(s) => {
                  let x = random::<usize>() % s.length();
                  let y = (random::<usize>() % (s.length() - x)) + x + 1;
                  let cw = s.get_index().get_char_width();
                  let _ = black_box(cw.slice(x..y));
               },
               _ => panic!("expected leaf"),
            }
         });
      });
   }
}

criterion_group!(
   benches,
   slice_char_width,
);
criterion_main!(benches);
