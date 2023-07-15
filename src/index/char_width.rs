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
use std::fmt;
use std::iter::{FromIterator, IntoIterator};
use std::ops::Range;

// TODO: seriously ponder the dodgy name on this one...
/// TODO: document Run struct
pub struct Run {
   width: u8,
   count: u32,
}

/// TODO: document CharWidth struct
pub struct CharWidth {
    widths: Box<[Run]>,
}

/// This is the primary struct which creates, and provides access to, the index.
//#[derive(Debug)]
pub struct CharWidthBuilder {
   widths: Vec<Run>,
}


impl Run {
   /// TODO: document Run::new
   #[inline]
   pub fn new(c: char) -> Run {
      return Run { width: c.len_utf8() as u8, count: 1 };
   }

   /// TODO: document Run::byte_count
   #[inline]
   pub fn byte_count(&self) -> usize {
      return self.width as usize * self.count as usize;
   }

   /// TODO: document Run::width_eq
   #[inline]
   pub fn width_eq(&self, c: char) -> bool {
      return self.width == c.len_utf8() as u8;
   }
}

impl fmt::Debug for Run {
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
      return write!(f, "{}:{}", self.width, self.count);
   }
}


/// TODO: document CharWidth Index
impl CharWidth {
   /// Provides an iterator over the raw width:count pairs.
   #[inline]
   pub fn iter(&self) -> impl Iterator<Item = &Run> + '_ {
      return self.widths.iter();
   }

   /// Get the total char count of the indexed string.
   #[inline]
   pub fn count(&self) -> usize {
      return self.widths.iter().fold(0, |a, x| a + x.count as usize);
   }

   /// Get the total byte count of the indexed string.
   #[inline]
   pub fn count_bytes(&self) -> usize {
      // TODO: delegate the calculation to Run
      return self.widths.iter().fold(0, |a, x| a + x.byte_count());
   }

   /// Get the byte index from a char index.
   #[inline]
   pub fn byte_index(&self, mut char_index: usize) -> usize {
      return self.widths.iter()
         .scan(0, |a, x| {
            if *a > char_index { None } else {
               *a += x.count as usize;
               Some(x)
            }
         })
         .fold(0, move |a, x| {
            if char_index < x.count as usize {
               return a + char_index * x.width as usize;
            }
            char_index -= x.count as usize;
            return a + x.byte_count() as usize;
         });
   }

   /// TODO: document this?
   #[inline]
   pub fn split(&self, mut r: Range<usize>) -> (Self, Self) where Self: Sized {
      let (a, b) = self.iter().fold((Vec::<Run>::new(), Vec::<Run>::new()), |mut v, x| {
         if r.start > 0 {
            v.0.push(Run { width: x.width, count: std::cmp::min(x.count, r.start as u32) });
         }
         if r.end < x.count as usize {
            v.1.push(Run { width: x.width, count: x.count - r.end as u32 });
         }
         r.start = r.start.saturating_sub(x.count as usize);
         r.end = r.end.saturating_sub(x.count as usize);
         return v;
      });
      return (Self{ widths: a.into_boxed_slice() }, Self{ widths: b.into_boxed_slice() });
   }
}

impl fmt::Debug for CharWidth {
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
      return write!(f, "{:?}", self.widths);
   }
}


impl CharWidthBuilder {
   /// Create a new empty CharWidthBuilder.
   ///
   /// You should only need to use this directly if you need to index an arbitrary sequence
   /// of chars; other methods are provided for most conmon use cases, most notably from_iter,
   /// which can be used by collect() in an interator chain.
   // TODO: list some of the common cases, when they are properly implemented.
   #[inline]
   pub fn new() -> CharWidthBuilder {
      return CharWidthBuilder {
         widths: Vec::new(),
      }
   }

   /// Add a single char to the end of the index.
   /// This should only need to be used directly if building your own index over an arbitrary sequence.
   #[inline]
   pub fn push(&mut self, c: char) {
      if let Some(x) = self.widths.last_mut() {
         if x.width_eq(c) {
            x.count += 1;
            return;
         }
      }
      self.widths.push(Run::new(c));
   }

   /// TODO: document CharWidthBuilder::freeze
   #[inline]
   pub fn freeze(self) -> CharWidth {
      return CharWidth {
          widths: Box::from(self.widths),
      };
   }
}


impl FromIterator<char> for CharWidthBuilder {
   /// Fold any arbitrary Iterator<Item = char> into a CharWidthBuilder.
   ///
   /// This is the most general way to create a CharWidthBuilder, and the favoured way to do so
   /// unless you have actual compelling reasons to do otherwise.
   ///
   /// // # Examples
   /// // ```
   /// // use braid::index::char_width::CharWidthBuilder;
   /// //
   /// // let map: CharWidthBuilder = "abcⓐⓑⓒ".chars().collect();
   /// // let map = map.freeze();
   /// // assert_eq!(map.count(), 6);
   /// // assert_eq!(map.count_bytes(), 12);
   /// // ```
   #[inline]
   fn from_iter<T>(iter: T) -> Self
   where T: IntoIterator<Item = char> {
      iter.into_iter().fold(CharWidthBuilder::new(), |mut m, c| {
         m.push(c);
         return m;
      })
   }
}


/// Enable ::from() convesion for anything that can be converted into an Iterator<Item = char>.
impl<T> From<T> for CharWidthBuilder
where T: IntoIterator<Item = char> {
   #[inline]
   fn from(iter: T) -> Self {
      return iter.into_iter().collect();
   }
}


#[cfg(test)]
mod tests {
   use super::*;
   use pretty_assertions::{assert_eq, /*assert_ne*/};

   #[test]
   fn counts() {
      let mut m = super::CharWidthBuilder::new();
      m.push('a');
      m.push('Č');
      m.push('ਇ');
      m.push('𑄗');
      let m = m.freeze();
      assert_eq!(m.count(), 4);
      assert_eq!(m.count_bytes(), 10);
   }

   #[test]
   fn from_iter() {
      let s = "test ‣ string ‣ alpha";
      let m = s.chars().collect::<super::CharWidthBuilder>().freeze();
      assert_eq!(m.count(), 21);
      assert_eq!(m.count_bytes(), 25);
      assert_eq!(m.iter().map(|r| format!("({}:{})", r.width, r.count)).fold(String::new(), |s, x| format!("{s}{x} ")), "(1:5) (3:1) (1:8) (3:1) (1:6) ");
   }

   // Yeah, it's a bit of a knockoff... But apparently this is needed...
   // TODO: merge tests
   #[test]
   fn from_intoiterator() {
      let s = "test ‣ string ‣ alpha";
      let m = CharWidthBuilder::from(s.chars()).freeze();
      assert_eq!(m.count(), 21);
      assert_eq!(m.count_bytes(), 25);
      assert_eq!(m.iter().map(|r| format!("({}:{})", r.width, r.count)).fold(String::new(), |s, x| format!("{s}{x} ")), "(1:5) (3:1) (1:8) (3:1) (1:6) ");
   }

   #[test]
   fn split_index() {
      let i = CharWidthBuilder::from("abc󰯬󰯯󰯲123ⅠⅡⅢ".chars()).freeze();
      let (a, b) = i.split(0..0);
      assert_eq!(format!("{a:?}"), "[]");
      assert_eq!(format!("{b:?}"), "[1:3, 4:3, 1:3, 3:3]");
      let (a, b) = i.split(6..6);
      assert_eq!(format!("{a:?}"), "[1:3, 4:3]");
      assert_eq!(format!("{b:?}"), "[1:3, 3:3]");
      let (a, b) = i.split(0..12);
      assert_eq!(format!("{a:?}"), "[]");
      assert_eq!(format!("{b:?}"), "[]");
      let (a, b) = i.split(12..12);
      assert_eq!(format!("{a:?}"), "[1:3, 4:3, 1:3, 3:3]");
      assert_eq!(format!("{b:?}"), "[]");
      let (a, b) = i.split(1..11);
      assert_eq!(format!("{a:?}"), "[1:1]");
      assert_eq!(format!("{b:?}"), "[3:1]");
      let (a, b) = i.split(3..9);
      assert_eq!(format!("{a:?}"), "[1:3]");
      assert_eq!(format!("{b:?}"), "[3:3]");
      let (a, b) = i.split(4..8);
      assert_eq!(format!("{a:?}"), "[1:3, 4:1]");
      assert_eq!(format!("{b:?}"), "[1:1, 3:3]");
   }
}
