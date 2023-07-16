//! braid is an exercise in cute names and bad ideas.
//! (I got so far as 'cute' and autopilot just _roasted_ me...)
//!
//! It is true however that it is entirely intended as a learning project.
//! Let it be a monument to exactly what _not_ to attempt first when learning
//! and experimenting with Rust.
//!
//! Due to the above, heed the following warning: this codebase has no guarantees
//! of stability, correctness, efficiency, cleanliness, or iodomatic design.
//! The design decisions (and Rust specific implementations) may not be ideal for
//! _any_ use language, never mind for Rust.
//!
//! # What?
//! It's an implementation of a [Rope](http://www.bitsavers.org/pdf/xerox/parc/techReports/CSL-94-10_Ropes_Are_Better_Than_Strings.pdf),
//! a data structure which I gather was first developed at Xerox PARC in the early 1980s.
//! The name is a joke about a rope being more robust and heavyweight than a string.
//! I hope my own humble naming choices do their part to keep your eyes rolling just
//! that little longer.
//!
//! It's a version written in Rust (duh), which aims to be a pretty 'pure' implementation
//! of the original concept, at it's core at least. Agnostic about where its actual data is,
//! recursively structured so that no (sub)tree has the faintest idea where it may or may not
//! live in any particular hierarchy, and immutable.
//!
//! # Why?
//! I'm learning Rust.
//!
//! Or, at least, I'm in a battle to the death with the borrow checker, the compiler at large,
//! and whatever dark gods smile upon the whole eldritch behemoth.
//!
//! No, it turns out that it's not the _ideal_ starter project to highlight the best of the language.
//! There's worse data structures out there though, I can easily imagine.
//!
//! ## No, but, why..?
//! _'There's this crate called [ropey](https://crates.io/crates/ropey)...'_
//!
//! Yup, I saw it, I skimmed the (non-code) details, and elected to ignore it and reinvent some
//! wheels becuase I needed a learning project. I also elected to avoid the source and any real
//! implementation details, so as not to be influenced.
//!
//! As it happens, I seem to have made just about every possible design decision differently.
//! This is a little worrying, but at least it means this codebase has a more unique architecture.
//!
//! # Practical overview
//! The core data structure (right now) is the [`Strand`](strand::Strand), which represents every
//! position of the tree; root, branches, leaves...
//!
//! Initialised empty or with an inital string as a monolithic leaf, subsequent insert & remove
//! operations will create new `Strand`s, which are split and merged as necessary, while sharing
//! as much underlying data as possible.
//!
//! Access to the data is provided almost exclusively via iterators, and is _heavily_ intended to
//! be [`char`] based, rather than [`byte`] based.
//!
//! Secondarily the intent is to have robust support for line based operations. Of note here is that
//! the design direction is _opinionated_ about line endings; `LF`, `\n`, `0x0A`, etc. is canonical.
//! At present `CR` is simply ignored as a non-printing character, though in future it may even be
//! sanitised out of the data entirely (though in that case some support would be included to export
//! with `CRLF` line endings).
//!
//! ## Higher level functionality
//! It is intended that there will be some array of higher level functionality provided by additional
//! modules/data structures, including features like slab/arena memory allocation to improve cache
//! performance, helper functionality for managing edit history, possibly even utilities to assist
//! line-wrapping etc.
//!
//! None of this is at all concrete right now, though the former couple of things there are likely
//! to be a must in some form for milestone `1.0.0`.

#![feature(exclusive_range_pattern)]
#![feature(test)]
pub mod strand;
pub mod index;
