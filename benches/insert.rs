// These benchmarks were lifted from ropey, to compare and sanity check our performance.
// Consequently, they should be considered as licensed under the MIT license.

extern crate criterion;
extern crate rand;

use criterion::{criterion_group, criterion_main, Criterion};
use rand::random;
use braid::strand::Strand;

const TEXT: &str = include_str!("data/large");


fn insert_small(c: &mut Criterion) {
   let mut group = c.benchmark_group("insert_small");

   group.bench_function("random", |bench| {
      let mut rope = Strand::new_leaf(TEXT);
      bench.iter(|| {
         let len = rope.length();
         rope = rope.insert("a", random::<usize>() % len)
      })
   });

   group.bench_function("start", |bench| {
      let mut rope = Strand::new_leaf(TEXT);
      bench.iter(|| {
         rope = rope.insert("a", 0);
      })
   });

   group.bench_function("middle", |bench| {
      let mut rope = Strand::new_leaf(TEXT);
      bench.iter(|| {
         let len = rope.length();
         rope = rope.insert("a", len / 2);
      })
   });

   group.bench_function("end", |bench| {
      let mut rope = Strand::new_leaf(TEXT);
      bench.iter(|| {
         let len = rope.length();
         rope = rope.insert("a", len);
      })
   });
}


fn insert_medium(c: &mut Criterion) {
   let mut group = c.benchmark_group("insert_medium");

   group.bench_function("random", |bench| {
      let mut rope = Strand::new_leaf(TEXT);
      bench.iter(|| {
         let len = rope.length();
         rope = rope.insert("This is some text.", random::<usize>() % len);
      })
   });

   group.bench_function("start", |bench| {
      let mut rope = Strand::new_leaf(TEXT);
      bench.iter(|| {
         rope = rope.insert("This is some text.", 0);
      })
   });

   group.bench_function("middle", |bench| {
      let mut rope = Strand::new_leaf(TEXT);
      bench.iter(|| {
         let len = rope.length();
         rope = rope.insert("This is some text.", len / 2);
      })
   });

   group.bench_function("end", |bench| {
      let mut rope = Strand::new_leaf(TEXT);
      bench.iter(|| {
         let len = rope.length();
         rope = rope.insert("This is some text.", len);
      })
   });
}


const INSERT_TEXT: &str = include_str!("data/small");

fn insert_large(c: &mut Criterion) {
   let mut group = c.benchmark_group("insert_large");

   group.bench_function("random", |bench| {
      let mut rope = Strand::new_leaf(TEXT);
      bench.iter(|| {
         let len = rope.length();
         rope = rope.insert(INSERT_TEXT, random::<usize>() % len);
      })
   });

   group.bench_function("start", |bench| {
      let mut rope = Strand::new_leaf(TEXT);
      bench.iter(|| {
         rope = rope.insert(INSERT_TEXT, 0);
      })
   });

   group.bench_function("middle", |bench| {
      let mut rope = Strand::new_leaf(TEXT);
      bench.iter(|| {
         let len = rope.length();
         rope = rope.insert(INSERT_TEXT, len / 2);
      })
   });

   group.bench_function("end", |bench| {
      let mut rope = Strand::new_leaf(TEXT);
      bench.iter(|| {
         let len = rope.length();
         rope = rope.insert(INSERT_TEXT, len);
      })
   });
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
   // TODO: replace this & insert_char with one thing as this API has no
   // char level insert method anyway
   //insert_char,
   insert_small,
   insert_medium,
   insert_large,
   insert_after_clone
);
criterion_main!(benches);
