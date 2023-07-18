// Copyright 2023 Joey Sabey <joey.sabey@gmx.com>
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.

//! This module provides the [`Splittable`] trait, which enforces that a type
//! can be split into 0-2 parts (i.e. left & right) when removing a given range.
//!
//! To achieve this, the type must also implement [`Ranged`], which ensures it
//! has a defined [`length()`](Ranged::length) (generally in [`char`]'s), so
//! that the length of the left & right parts can be calculated.
use crate::ranged::Ranged;
use std::ops::{Range, RangeBounds};

/// A ([`Ranged`]) type which can be split into 0-2 parts.
pub trait Splittable: Ranged {
   /// Get a 'left' & 'right' range which would cover everything before & after
   /// the given range (respectively), if that range were to be removed.
   #[inline(always)]
   fn get_split_ranges(&self, r: impl RangeBounds<usize>) -> (Range<usize>, Range<usize>) {
      let r = self.normalise_range(r);
      (0..r.start, r.end..self.length())
   }

   /// Convenience method for splitting at a specific scalar offset (i.e. a zero-width range).
   #[inline(always)]
   fn get_split_at_ranges(&self, i: usize) -> (Range<usize>, Range<usize>) {
      self.get_split_ranges(i..i)
   }

   /// Convenience method for getting `Option<Range>`'s instead of raw `Range`'s.
   /// (Where `None` indicates that the range would be empty/zero-width.)
   #[inline(always)]
   fn get_split_ranges_opt(&self, r: impl RangeBounds<usize>) -> (Option<Range<usize>>, Option<Range<usize>>) {
      let (a, b) = self.get_split_ranges(r);
      (
         if a.start == a.end { None } else { Some(a) },
         if b.start == b.end { None } else { Some(b) },
      )
   }

   /// Convenience method for splitting at a specific scalar offset (i.e. a zero-width range),
   /// returning `Option<Range>`'s instead of raw `Range`'s.
   /// (Where `None` indicates that the range would be empty/zero-width.)
   #[inline(always)]
   fn get_split_at_ranges_opt(&self, i: usize) -> (Option<Range<usize>>, Option<Range<usize>>) {
      self.get_split_ranges_opt(i..i)
   }

   /// TODO: document!
   fn split(&self, r: impl RangeBounds<usize>) -> (Option<Self>, Option<Self>) where Self: Sized;

}


#[cfg(test)]
mod tests {
   use super::*;

   #[derive(Debug, PartialEq, Eq)]
   struct TestSplittable { length: usize }
   impl Ranged for TestSplittable {
      fn length(&self) -> usize { self.length }
   }
   impl Splittable for TestSplittable {
      fn split(&self, r: impl RangeBounds<usize>) -> (Option<Self>, Option<Self>) where Self: Sized {
         let (a, b) = self.get_split_ranges_opt(r);
         (a.map(|r| TestSplittable{length: r.len()}), b.map(|r| TestSplittable{length: r.len()}))
      }
   }

   #[test]
   fn get_split_ranges() {
      let x = TestSplittable { length: 10 };
      assert_eq!(x.get_split_ranges(..), (0..0, 10..10));
      assert_eq!(x.get_split_ranges(0..0), (0..0, 0..10));
      assert_eq!(x.get_split_ranges(10..10), (0..10, 10..10));
      assert_eq!(x.get_split_ranges(5..5), (0..5, 5..10));
      assert_eq!(x.get_split_ranges(1..9), (0..1, 9..10));
      assert_eq!(x.get_split_ranges(0..=8), (0..0, 9..10));
   }

   #[test]
   fn get_split_at_ranges() {
      let x = TestSplittable { length: 10 };
      assert_eq!(x.get_split_at_ranges(0), (0..0, 0..10));
      assert_eq!(x.get_split_at_ranges(10), (0..10, 10..10));
      assert_eq!(x.get_split_at_ranges(5), (0..5, 5..10));
   }

   #[test]
   fn get_split_ranges_opt() {
      let x = TestSplittable { length: 10 };
      assert_eq!(x.get_split_ranges_opt(..), (None, None));
      assert_eq!(x.get_split_ranges_opt(0..0), (None, Some(0..10)));
      assert_eq!(x.get_split_ranges_opt(10..10), (Some(0..10), None));
      assert_eq!(x.get_split_ranges_opt(..5), (None, Some(5..10)));
      assert_eq!(x.get_split_ranges_opt(5..), (Some(0..5), None));
   }

   #[test]
   fn get_split_at_ranges_opt() {
      let x = TestSplittable { length: 10 };
      assert_eq!(x.get_split_at_ranges_opt(0), (None, Some(0..10)));
      assert_eq!(x.get_split_at_ranges_opt(10), (Some(0..10), None));
      assert_eq!(x.get_split_at_ranges_opt(5), (Some(0..5), Some(5..10)));
   }

   // TODO: maybe drop this? or move into a doc comment above; it's more a demo than a real test tbh...
   #[test]
   fn split() {
      let x = TestSplittable { length: 10 };
      assert_eq!(x.split(..), (None, None));
      assert_eq!(x.split(0..0), (None, Some(TestSplittable{length: 10})));
      assert_eq!(x.split(10..10), (Some(TestSplittable{length: 10}), None));
      assert_eq!(x.split(..5), (None, Some(TestSplittable{length: 5})));
      assert_eq!(x.split(5..), (Some(TestSplittable{length: 5}), None));
      assert_eq!(x.split(1..9), (Some(TestSplittable{length: 1}), Some(TestSplittable{length: 1})));
   }
}
