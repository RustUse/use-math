#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Series utilities for `RustUse`.

pub mod error;
pub mod prelude;
pub mod progression;

pub use error::SeriesError;
pub use progression::{arithmetic_nth_term, arithmetic_sum, geometric_nth_term, geometric_sum};
