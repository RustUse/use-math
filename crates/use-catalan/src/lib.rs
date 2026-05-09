#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Composable Catalan number primitives for `RustUse`.

pub mod catalan;
pub mod iter;
pub mod prelude;

pub use catalan::{
    catalan, catalan_by_binomial, catalan_recursive, catalan_sequence, is_catalan_number,
};
pub use iter::CatalanSequence;
