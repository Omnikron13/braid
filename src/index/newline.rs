// Copyright 2023 Joey Sabey <joey.sabey@gmx.com>
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.

//! This module provides an index of newlines in the string.
use std::ops::{Bound, Range};
use crate::sliceable::Sliceable;

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


impl Sliceable for Newline {
   /// TODO: document Newline::slice()
   #[inline]
   fn slice(&self, range: impl std::ops::RangeBounds<usize>) -> Self {
      // Normalise range bounds
      let s = match range.start_bound() {
         Bound::Included(&s) => s,
         Bound::Unbounded => 0,
         _ => unreachable!("start bound must be inclusive or unbounded"),
      };
      let e = match range.end_bound() {
         Bound::Included(&e) => e,
         Bound::Excluded(&e) => e - 1,
         Bound::Unbounded => usize::MAX,
      };
      // Error out nonsensible ranges
      assert!(s <= e, "start bound must be less than or equal to end bound");
      // Find internal array start/end indices
      let s = self.index.iter().enumerate().skip_while(|(_, &q)| q < s).take(1).map(|(i, _)| i).next().unwrap_or(self.index.len());
      let e = self.index.iter().enumerate().skip_while(|(_, &q)| q <= e).take(1).map(|(i, _)| i).next().unwrap_or(self.index.len());
      // Return sliced Newline index (literally a slice really, with this one)
      return Newline{index: unsafe{ self.index.get_unchecked(s..e) }.into()};
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

#[cfg(test)]
mod tests {
   use super::*;
   use pretty_assertions::assert_eq;

   #[test]
   fn slice_newline() {
      let nl =  Newline{index: Box::new([2, 3, 5, 7, 11, 13, 17, 19, 23, 29])};
      assert_eq!(format!("{:?}", nl), "Newline { index: [2, 3, 5, 7, 11, 13, 17, 19, 23, 29] }");
      assert_eq!(format!("{:?}", nl.slice(..)), "Newline { index: [2, 3, 5, 7, 11, 13, 17, 19, 23, 29] }");
      assert_eq!(format!("{:?}", nl.slice(4..)), "Newline { index: [5, 7, 11, 13, 17, 19, 23, 29] }");
      assert_eq!(format!("{:?}", nl.slice(5..)), "Newline { index: [5, 7, 11, 13, 17, 19, 23, 29] }");
      assert_eq!(format!("{:?}", nl.slice(..=19)), "Newline { index: [2, 3, 5, 7, 11, 13, 17, 19] }");
      assert_eq!(format!("{:?}", nl.slice(..=20)), "Newline { index: [2, 3, 5, 7, 11, 13, 17, 19] }");
      assert_eq!(format!("{:?}", nl.slice(4..=20)), "Newline { index: [5, 7, 11, 13, 17, 19] }");
      assert_eq!(format!("{:?}", nl.slice(5..=19)), "Newline { index: [5, 7, 11, 13, 17, 19] }");
      assert_eq!(format!("{:?}", nl.slice(2..=2)), "Newline { index: [2] }");
      assert_eq!(format!("{:?}", nl.slice(0..1)), "Newline { index: [] }");
      assert_eq!(format!("{:?}", nl.slice(30..40)), "Newline { index: [] }");
   }
}
