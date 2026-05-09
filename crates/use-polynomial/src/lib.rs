#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Small polynomial primitives for `RustUse`.

pub mod polynomial;
pub mod prelude;

pub use polynomial::Polynomial;
