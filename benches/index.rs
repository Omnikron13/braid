extern crate criterion;

use rand::random;
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use criterion::black_box;
use braid::index::{Index, IndexBuilder};

fn count(c: &mut Criterion) {
   let mut g = c.benchmark_group("count");
   g.sample_size(1000);
   for name in [
      "small",
      "medium",
      "large",
      "unicode_01",
      //"unicode_02",
      "cyrillic_1",
      "cyrillic_2",
   ] {
      let data = std::fs::read_to_string(format!("benches/data/{name}")).unwrap();
      let index: Index = data.chars().collect();
      g.bench_with_input(BenchmarkId::new("chars/manual", name), &data, |b, data| {
         b.iter(|| {
            let _ = black_box(data.chars()).count();
         });
      });
      g.bench_with_input(BenchmarkId::new("chars/indexed", name), &index, |b, data| {
         b.iter(|| {
            let _ = black_box(data).count();
         });
      });
      g.bench_with_input(BenchmarkId::new("bytes/manual", name), &data, |b, data| {
         b.iter(|| {
            let _ = black_box(data).len();
         });
      });
      g.bench_with_input(BenchmarkId::new("bytes/indexed", name), &index, |b, data| {
         b.iter(|| {
            let _ = black_box(data).count_bytes();
         });
      });
   }
   g.finish();
}


fn byte_index(c: &mut Criterion) {
   let mut g = c.benchmark_group("byte_index");
   g.sample_size(1000);
   for name in [
      "tiny",
      "small",
      "medium",
      "large",
      //"unicode_trivial",
      "unicode_01",
      //"unicode_02",
      "cyrillic_1",
      "cyrillic_2",
   ] {
      let data = std::fs::read_to_string(format!("benches/data/{name}")).unwrap();
      let index: Index = data.chars().collect();

      g.bench_with_input(BenchmarkId::new("manual", name), &data, |b, data| {
         b.iter(|| {
            let i = random::<usize>() % index.count();
            let _ = black_box(data).char_indices().nth(i).unwrap().0;
         });
      });
      g.bench_with_input(BenchmarkId::new("indexed", name), &index, |b, data| {
         b.iter(|| {
            let i = random::<usize>() % index.count();
            let _ = black_box(data).byte_index(i);
         });
      });
   }
}


fn push(c: &mut Criterion) {
   let mut g = c.benchmark_group("push");
   g.sample_size(1000);
   g.measurement_time(std::time::Duration::from_secs(10));
   for n in [
      1024,
      //2048,
      4096,
   ] {
      for (name, input) in [
         ("rand", (0..n).map(|_| random::<char>()).collect()),
         ("uniform", vec!('~')),
         ("alternating", vec!['~', '💖']),
         ("cyrillic_1", include_str!("data/cyrillic_1").chars().collect()),
      ].iter() {
         g.throughput(Throughput::Bytes(n as u64));
         g.bench_with_input(BenchmarkId::new(*name, n), input, |b, input| {
            b.iter(|| {
               let mut m = IndexBuilder::new();
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
