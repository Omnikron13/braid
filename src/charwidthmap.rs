// Copyright 2023 Joey Sabey <joey.sabey@gmx.com>
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.

//! This module provides an index of the widths of characters in a str.
//!
//! This can be used to quickly calculate the width of a string in char's, or
//! to find the char index from a given byte index, etc.
//!
//! These tasks can often be quite slow, as they usually require iterating over
//! the entire string, char by char, to compute. This module provides a way to
//! cache the relevant information, so this kind of operation becomes much
//! cheaper past the initial iteration.
use std::iter::{FromIterator, IntoIterator};

/// This is the primary struct which creates, and provides access to, the index.
pub struct CharWidthMap {
   widths: Vec<(u8, usize)>,
}

impl CharWidthMap {
   /// Create a new empty CharWidthMap.
   ///
   /// You should only need to use this directly if you need to index an arbitrary sequence
   /// of chars; other methods are provided for most conmon use cases, most notably from_iter,
   /// which can be used by collect() in an interator chain.
   // TODO: list some of the common cases, when they are properly implemented.
   pub fn new() -> CharWidthMap {
      return CharWidthMap {
         widths: Vec::new(),
      }
   }

   /// Add a single char to the end of the index.
   /// This should only need to be used directly if building your own index over an arbitrary sequence.
   pub fn push(&mut self, c: char) {
      let w = c.len_utf8() as u8;
      if self.widths.len() == 0 || self.widths.last().unwrap().0 != w {
         self.widths.push((w, 1));
      } else {
         self.widths.last_mut().unwrap().1 += 1;
      }
   }

   /// Provides an iterator over the raw width:count paits.
   pub fn iter(&self) -> impl Iterator<Item = (u8, usize)> + '_ {
      return self.widths.iter().copied();
   }

   /// Get the total char count of the indexed string.
   pub fn count(&self) -> usize {
      return self.widths.iter().map(|(_, n)| n).sum();
   }

   /// Get the total byte count of the indexed string.
   pub fn count_bytes(&self) -> usize {
      return self.widths.iter().map(|(w, n)| *w as usize * n).sum();
   }
}


impl FromIterator<char> for CharWidthMap {
   /// Fold any arbitrary Iterator<Item = char> into a CharWidthMap.
   ///
   /// This is the most general way to create a CharWidthMap, and the favoured way to do so
   /// unless you have actual compelling reasons to do otherwise.
   ///
   /// # Examples
   ///
   /// ```
   /// use braid::charwidthmap::CharWidthMap;
   ///
   /// let map: CharWidthMap = "abcⓐⓑⓒ".chars().collect();
   /// assert_eq!(map.count(), 6);
   /// assert_eq!(map.count_bytes(), 12);
   /// ```
   fn from_iter<T>(iter: T) -> Self
   where T: IntoIterator<Item = char> {
      iter.into_iter().fold(CharWidthMap::new(), |mut m, c| {
         m.push(c);
         return m;
      })
   }
}


/// Enable ::from() convesion for anything that can be converted into an Iterator<Item = char>.
impl<T> From<T> for CharWidthMap
where T: IntoIterator<Item = char> {
   fn from(iter: T) -> Self {
      return iter.into_iter().collect();
   }
}


#[cfg(test)]
mod tests {
   use pretty_assertions::{assert_eq, /*assert_ne*/};

   #[test]
   fn test_counts() {
      let mut m = super::CharWidthMap::new();
      assert_eq!(m.count(), 0);
      assert_eq!(m.count_bytes(), 0);
      m.push('a');
      assert_eq!(m.count(), 1);
      assert_eq!(m.count_bytes(), 1);
      m.push('Č');
      assert_eq!(m.count(), 2);
      assert_eq!(m.count_bytes(), 3);
      m.push('ਇ');
      assert_eq!(m.count(), 3);
      assert_eq!(m.count_bytes(), 6);
      m.push('𑄗');
      assert_eq!(m.count(), 4);
      assert_eq!(m.count_bytes(), 10);
   }

   #[test]
   fn test_from_iter() {
      let s = "test ‣ string ‣ alpha";
      let m = s.chars().collect::<super::CharWidthMap>();
      assert_eq!(m.count(), 21);
      assert_eq!(m.count_bytes(), 25);
      assert_eq!(m.iter().map(|(w, n)| format!("({w}:{n})")).fold(String::new(), |s, x| format!("{s}{x} ")), "(1:5) (3:1) (1:8) (3:1) (1:6) ");
   }

   // Yeah, it's a bit of a knockoff... But apparently this is needed...
   // TODO: merge tests
   #[test]
   fn test_from_intoiterator() {
      let s = "test ‣ string ‣ alpha";
      let m = CharWidthMap::from(s.chars());
      assert_eq!(m.count(), 21);
      assert_eq!(m.count_bytes(), 25);
      assert_eq!(m.iter().map(|(w, n)| format!("({w}:{n})")).fold(String::new(), |s, x| format!("{s}{x} ")), "(1:5) (3:1) (1:8) (3:1) (1:6) ");
   }
}
