#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Composable series primitives for `RustUse`.

pub mod prelude;
pub mod series;

pub use series::Series;
