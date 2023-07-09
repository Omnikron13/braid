extern crate criterion;

use rand::random;
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use braid::index::char_width::CharWidth;

fn count(c: &mut Criterion) {
   let mut group = c.benchmark_group("count_chars");

   let mut run = |name, data: &str| {
       let index: CharWidth = data.chars().collect();
       group.bench_function(format!("{name}-count_manual"), |bench| {
          bench.iter(|| {
             let _ = black_box(&data).chars().count();
          })
       });
       group.bench_function(format!("{name}-count_indexed"), |bench| {
          bench.iter(|| {
             let _ = index.count();
          })
       });
       assert_eq!(index.count(), data.chars().count());
   };

   run("small", include_str!("data/small.txt"));
   run("medium", include_str!("data/medium.txt"));
   run("large", include_str!("data/large.txt"));
   run("unicode-1", include_str!("data/unicode_01"));
   run("unicode-2", include_str!("data/unicode_02"));
   run("cyrillic", include_str!("data/cyrillic_01"));
}

fn byte_index(c: &mut Criterion) {
   let mut g = c.benchmark_group("byte_index");

   let mut run = |f| {
      let path = format!("benches/data/{f}");
      let data = std::fs::read_to_string(path).unwrap();
      let index: CharWidth = data.chars().collect();

      // Sanity check
      for _ in 0..1024 {
         let i = random::<usize>() % (index.count() + 0);
         let a = match data.char_indices().nth(i) {
            Some((i, _)) => i,
            None => data.len(),
         };
         let b = index.byte_index(i);
         assert_eq!(a, b);
      }

      g.bench_function(format!("byte_manual-{f}"), |bench| {
         bench.iter(|| {
            let i = random::<usize>() % (index.count() + 0);
            let _ = match black_box(&data).char_indices().nth(i) {
               Some((i, _)) => i,
               None => data.len(),
            };
         });
      });

      g.bench_function(format!("byte_index-{f}"), |bench| {
         bench.iter(|| {
            let i = random::<usize>() % (index.count() + 0);
            let _ = index.byte_index(i);
         });
      });

   };

   run("tiny.txt");
   run("small.txt");
   run("medium.txt");
   run("large.txt");
   run("unicode_trivial");
   run("unicode_01");
   run("unicode_02");
   run("cyrillic_01");
}

criterion_group!(
   benches,
   count,
   byte_index,
);
criterion_main!(benches);
