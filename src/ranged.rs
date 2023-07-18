// Copyright 2023 Joey Sabey <joey.sabey@gmx.com>
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.

//! This module provides the [`Ranged`] trait, which ensures a type can supply
//! its own [`length()`](Ranged::length) (generally in [`char`]'s), and can
//! translate the various range types (`..`, `a..`, `..b`, `..=c`, `d..=e`)
//! into the 'normalised' `x..y` form.
use std::ops::{Bound, Range, RangeBounds};

/// Something with a defined range; a concrete size/length. `0..N`
pub trait Ranged {
   /// Get the length/size of the thing (typically in [`char`]'s).
   /// This is required for various calculations that this type, and sub-types,
   /// need to perform.
   fn length(&self) -> usize;

   /// Convert ranges (see [`std::ops::RangeBounds`]) into a 'normalised' `x..y`
   /// form. Accepts any of the other valid forms: `..`, `a..`, `..b`, `..=c`, `d..=e`
   ///
   /// # Example
   /// ```
   /// use braid::ranged::Ranged;
   ///
   /// struct TestRanged { length: usize }
   ///
   /// impl Ranged for TestRanged {
   ///    fn length(&self) -> usize { self.length }
   /// }
   ///
   /// let tr = TestRanged { length: 10 };
   ///
   /// assert_eq!(tr.normalise_range(..), 0..10);
   /// assert_eq!(tr.normalise_range(0..), 0..10);
   /// assert_eq!(tr.normalise_range(..10), 0..10);
   /// assert_eq!(tr.normalise_range(..=9), 0..10);
   /// assert_eq!(tr.normalise_range(0..10), 0..10);
   /// assert_eq!(tr.normalise_range(..5), 0..5);
   /// assert_eq!(tr.normalise_range(5..), 5..10);
   /// ```
   fn normalise_range(&self, r: impl RangeBounds<usize>) -> Range<usize> {
      let r = match (r.start_bound(), r.end_bound()) {
         (Bound::Unbounded, Bound::Unbounded) => 0..self.length(),
         (Bound::Unbounded, Bound::Excluded(&e)) => 0..e,
         (Bound::Unbounded, Bound::Included(&e)) => 0..(e + 1),
         (Bound::Included(&s), Bound::Unbounded) => s..self.length(),
         (Bound::Included(&s), Bound::Excluded(&e)) => s..e,
         (Bound::Included(&s), Bound::Included(&e)) => s..(e + 1),
         _ => unreachable!("start_bound() should never be exclusive"),
      };
      assert!(r.end <= self.length(), "range end out of bounds");
      assert!(r.start <= r.end, "range start after end");
      return r;
   }
}

#[cfg(test)]
mod tests {
   use super::*;

   struct TestRanged { length: usize }
   impl Ranged for TestRanged {
      fn length(&self) -> usize { self.length }
   }

   #[test]
   fn normalise_range() {
      let tr = TestRanged { length: 10 };
      assert_eq!(tr.normalise_range(..), 0..10);
      assert_eq!(tr.normalise_range(0..), 0..10);
      assert_eq!(tr.normalise_range(..10), 0..10);
      assert_eq!(tr.normalise_range(..=9), 0..10);
      assert_eq!(tr.normalise_range(0..10), 0..10);
      assert_eq!(tr.normalise_range(..5), 0..5);
      assert_eq!(tr.normalise_range(5..), 5..10);
      // TODO: test for panic on out of bounds/out of order
   }
}
