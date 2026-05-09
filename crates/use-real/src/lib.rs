#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Small real-number primitives for `RustUse`.

pub mod error;
pub mod prelude;
pub mod real;

pub use error::RealError;
pub use real::{Real, RealInterval, approx_eq};
