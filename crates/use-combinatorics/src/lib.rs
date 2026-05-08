#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Small combinatorics helpers for `RustUse`.

pub mod counting;
pub mod error;
pub mod prelude;

pub use counting::{combinations, factorial, permutations};
pub use error::CombinatoricsError;
