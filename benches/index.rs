extern crate criterion;

use rand::random;
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use braid::index::char_width::CharWidthBuilder;

fn count(c: &mut Criterion) {
   let mut g = c.benchmark_group("count_chars");

   // Hangs for fucking forever doing 'warmup', despite duration claim, so just cludge away the warmup...
   g.warm_up_time(std::time::Duration::from_nanos(1));

   for name in ["small", "medium", "large", "unicode_01", "unicode_02", "cyrillic_01"] {
      let path = format!("benches/data/{name}");
      let data = std::fs::read_to_string(path).unwrap();
      g.bench_with_input(BenchmarkId::new("manual", name), &data, |b, data| {
         b.iter(|| {
            let _ = data.chars().count();
         });
      });
      g.bench_with_input(BenchmarkId::new("indexed", name), &data, |b, data| {
         let m = CharWidthBuilder::from_iter(data.chars());
         b.iter(|| {
            let _ = m.count();
         });
      });
      g.bench_with_input(BenchmarkId::new("indexed-frozen", name), &data, |b, data| {
         let m = CharWidthBuilder::from_iter(data.chars()).freeze();
         b.iter(|| {
            let _ = m.count();
         });
      });
   }
   g.finish();
}


fn byte_index(c: &mut Criterion) {
   let mut g = c.benchmark_group("byte_index");

   let mut run = |f| {
      let path = format!("benches/data/{f}");
      let data = std::fs::read_to_string(path).unwrap();
      let index: CharWidthBuilder = data.chars().collect();

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

   run("tiny");
   run("small");
   run("medium");
   run("large");
   run("unicode_trivial");
   run("unicode_01");
   run("unicode_02");
   run("cyrillic_01");
}


fn push(c: &mut Criterion) {
   let mut g = c.benchmark_group("push");
   g
      .sample_size(100000)
      .measurement_time(std::time::Duration::from_secs(45))
      .warm_up_time(std::time::Duration::from_secs(15));

   for n in [1024, 2048, 4096] {
      let chars: Vec<char> = (0..n).map(|_| random::<char>()).collect();
      let cyrillic: Vec<char> = include_str!("data/cyrillic_01").chars().collect();
      for (name, input) in [("rand", chars), ("uniform", vec!('~')), ("alternating", vec!['~', '💖']), ("cyrillic", cyrillic)].iter() {
         g.throughput(Throughput::Bytes(n as u64));
         g.bench_with_input(BenchmarkId::new(*name, n), input, |b, input| {
            b.iter(|| {
               let mut m = CharWidthBuilder::new();
               let mut char_iter = input.iter().cycle();
               for _ in 0..n {
                  m.push(*char_iter.next().unwrap());
               }
            });
         });
      }
   }
   g.finish();
}

criterion_group!(
   benches,
   count,
   byte_index,
   push,
);
criterion_main!(benches);
