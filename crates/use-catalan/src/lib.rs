#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Catalan-family utilities for `RustUse`.

pub mod counting;
pub mod error;
pub mod prelude;

pub use counting::{catalan, fuss_catalan};
pub use error::CatalanError;
