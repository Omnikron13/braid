// Copyright 2023 Joey Sabey <joey.sabey@gmx.com>
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
use crate::ranged::Ranged;

/// Something which can be 'sliced' (i.e. using the `a[x..y]` syntax) to
/// produce a new object of the same type.
pub trait Sliceable: Ranged {
   /// TODO: document Sliceable::slice()
   fn slice(&self, range: impl std::ops::RangeBounds<usize>) -> Self;
}
