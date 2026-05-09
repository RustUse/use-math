#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Integer helpers for `RustUse`.

pub mod error;
pub mod integer;
pub mod prelude;

pub use error::IntegerError;
pub use integer::{
    IntegerSign, are_coprime, classify_sign, gcd, is_divisible_by, is_even, is_odd, lcm,
};
