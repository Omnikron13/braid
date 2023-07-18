// Copyright 2023 Joey Sabey <joey.sabey@gmx.com>
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.

//! This module provides an index of newlines in the string.
use std::ops::Range;

/// TODO: document
#[derive(Debug)]
pub struct Newline {
   index: Box<[usize]>,
}

impl Newline {
   /// TODO: document
   #[inline]
   pub fn iter(&self) -> impl Iterator<Item=usize> + '_ {
      self.index.iter().copied()
   }

   /// TODO: document
   #[inline]
   pub fn split(&self, r: Range<usize>) -> (Self, Self) where Self: Sized {
      let s = self.index.iter().take_while(|&q| q <= &r.start).count();
      let e = self.index.iter().take_while(|&q| q < &r.end).count();
      (
         Newline{index: unsafe{ self.index.get_unchecked(..s) }.into()},
         Newline{index: unsafe{ self.index.get_unchecked(e..) }.into()},
      )
   }
}


/// TODO: document
pub struct NewlineBuilder {
   count: usize,
   index: Vec<usize>,
}

impl NewlineBuilder {
   /// TODO: document
   #[inline]
   pub fn new() -> Self {
      Self { count: 0, index: vec![] }
   }

   /// TODO: document
   #[inline]
   pub fn push(&mut self, c: char) {
      if c == '\n' {
         self.index.push(self.count);
      }
      self.count += 1;
   }

   /// TODO: document
   #[inline]
   pub fn freeze(self) -> Newline {
      return Newline{ index: self.index.into_boxed_slice() };
   }
}


#[cfg(test)]
#[test]
fn test_newline_index() {
   let s = "abc\nⒶⒷⒸ\n123\n①②③";
   let mut builder = NewlineBuilder::new();
   for c in s.chars() {
      builder.push(c);
   }
   let index: Newline = builder.freeze();
   assert_eq!(format!("{:?}", index.index), "[3, 7, 11]");
   let (a, b) = index.split(3..8);
   assert_eq!(format!("{:?}", a.index), "[3]");
   assert_eq!(format!("{:?}", b.index), "[11]");
}
