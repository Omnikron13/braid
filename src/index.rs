// Copyright 2023 Joey Sabey <joey.sabey@gmx.com>
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
mod char_width;
mod newline;

use std::iter::{FromIterator, IntoIterator};
use std::ops::RangeBounds;
use char_width::{CharWidth, CharWidthBuilder};
use newline::{Newline, NewlineBuilder};
use crate::ranged::Ranged;
use crate::splittable::Splittable;

/// TODO: document
pub struct Index {
   length: usize,
   char_width: CharWidth,
   newline: Newline,
}

impl Index {
   // TODO: document? maybe remove... this is essentially an alias for length(), which is required by Ranged
   #[inline]
   pub fn count(&self) -> usize {
      self.length()
   }

   /// TODO: document
   #[inline]
   pub fn count_bytes(&self) -> usize {
      self.char_width.count_bytes()
   }

   /// TODO: document
   #[inline]
   pub fn byte_index(&self, char_index: usize) -> usize {
      return self.char_width.byte_index(char_index);
   }

   /// TODO: document
   #[inline]
   pub fn newline_iter(&self) -> impl Iterator<Item=usize> + '_ {
      self.newline.iter()
   }
}

impl Ranged for Index {
   /// Get the length (in [`char`]s) that the index covers.
   ///
   /// Implementing this as part of this trait also provides a default implementation
   /// for of the `normalise_range()` method.
   #[inline]
   fn length(&self) -> usize {
      self.length
   }
}

impl Splittable for Index {
   /// TODO: document
   #[inline]
   fn split(&self, r: impl RangeBounds<usize>) -> (Option<Self>, Option<Self>) {
      let r = self.normalise_range(r);
      let (cw_a, cw_b) = self.char_width.split(r.clone());
      let (nl_a, nl_b) = self.newline.split(r.clone());
      return (
         Some(Index {
            length: r.start,
            char_width: cw_a,
            newline: nl_a,
         }),
         Some(Index {
            length: self.length - r.start,
            char_width: cw_b,
            newline: nl_b,
         }),
      );
   }
}

impl FromIterator<char> for Index {
   /// Fold any arbitrary Iterator<Item = char> into an Index.
   ///
   /// This is the most general way to create an Index, and the favoured way to do so
   /// unless you have actual compelling reasons to do otherwise.
   ///
   /// # Examples
   /// ```
   /// use braid::index::Index;
   ///
   /// let index: Index = "abcⓐⓑⓒ".chars().collect();
   /// assert_eq!(index.count(), 6);
   /// assert_eq!(index.count_bytes(), 12);
   /// ```
   #[inline]
   fn from_iter<T>(iter: T) -> Self where T: IntoIterator<Item = char> {
      IndexBuilder::from_iter(iter).freeze()
   }
}

impl From<&str> for Index {
   /// TODO: document
   #[inline]
   fn from(s: &str) -> Self {
      return s.chars().collect();
   }
}

impl From<String> for Index {
   /// TODO: document
   #[inline]
   fn from(s: String) -> Self {
      return s.chars().collect();
   }
}


/// TODO: document IndexBuilder
pub struct IndexBuilder {
   char_width: CharWidthBuilder,
   newline: NewlineBuilder,
}

impl IndexBuilder {
   /// TODO: document
   #[inline]
   pub fn new() -> Self {
      Self {
         char_width: CharWidthBuilder::new(),
         newline: NewlineBuilder::new(),
      }
   }

   /// TODO: document
   #[inline]
   pub fn push(&mut self, c: char) {
      self.char_width.push(c);
      self.newline.push(c);
   }

   /// TODO: document
   #[inline]
   pub fn freeze(self) -> Index {
      let char_width = self.char_width.freeze();
      let length = char_width.count();
      let newline = self.newline.freeze();
      return Index {
         length,
         char_width,
         newline,
      };
   }
}


impl FromIterator<char> for IndexBuilder {
   /// Fold any arbitrary Iterator<Item = char> into an IndexBuilder.
   ///
   /// This is the most general way to create an IndexBuilder, and the favoured way to do so
   /// unless you have actual compelling reasons to do otherwise.
   ///
   /// # Examples
   /// ```
   /// use braid::index::{Index, IndexBuilder};
   ///
   /// let builder: IndexBuilder = "abcⓐⓑⓒ".chars().collect();
   /// let index: Index = builder.freeze();
   /// assert_eq!(index.count(), 6);
   /// assert_eq!(index.count_bytes(), 12);
   /// ```
   #[inline]
   fn from_iter<T>(iter: T) -> Self
   where T: IntoIterator<Item = char> {
      iter.into_iter().fold(Self::new(), |mut m, c| {
         m.push(c);
         return m;
      })
   }
}

impl From<&str> for IndexBuilder {
   /// TODO: document
   #[inline]
   fn from(s: &str) -> Self {
      return s.chars().collect();
   }
}

impl From<String> for IndexBuilder {
   /// TODO: document
   #[inline]
   fn from(s: String) -> Self {
      return s.chars().collect();
   }
}


#[cfg(test)]
#[test]
fn test_index_from() {
   let s = "abcⓐⓑⓒ";
   let index: Index = s.chars().collect();
   assert_eq!(index.char_width.count(), 6);
   assert_eq!(index.char_width.count_bytes(), 12);

   let index: Index = Index::from(s);
   assert_eq!(index.char_width.count(), 6);
   assert_eq!(index.char_width.count_bytes(), 12);

   let s = String::from(s);
   let index: Index = Index::from(s);
   assert_eq!(index.char_width.count(), 6);
   assert_eq!(index.char_width.count_bytes(), 12);
}
