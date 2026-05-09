#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Numeric building blocks for `RustUse`.

pub mod constants;
pub mod number;
pub mod prelude;

pub use constants::{GOLDEN_RATIO, GOLDEN_RATIO_F32, SQRT_3, SQRT_3_F32};
pub use number::{
    NumberCategory, NumberSign, classify_number, classify_number_sign, is_finite_number,
};
