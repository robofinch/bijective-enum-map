#![no_std]

// See https://linebender.org/blog/doc-include for this README inclusion strategy
//! [`From`]: From
//! [`TryFrom`]: TryFrom
// File links are not supported by rustdoc
//! [LICENSE-APACHE]: https://github.com/robofinch/bijective-enum-map/blob/main/LICENSE-APACHE
//! [LICENSE-MIT]: https://github.com/robofinch/bijective-enum-map/blob/main/LICENSE-MIT
//!
//! <style>
//! .rustdoc-hidden { display: none; }
//! </style>
#![doc =  include_str!("../README.md")]

mod bijective;
mod injective;
// The helper macros in this module should not be considered part of the public API
// (for either usage or semver purposes).
mod helpers;
