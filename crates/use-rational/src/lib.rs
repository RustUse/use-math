#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Small rational-number primitives for `RustUse`.

pub mod error;
pub mod prelude;
pub mod rational;

pub use error::RationalError;
pub use rational::Rational;
