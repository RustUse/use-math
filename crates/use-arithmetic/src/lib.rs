#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Arithmetic primitives for `RustUse`.

pub mod checked;
pub mod divisibility;
pub mod division;
pub mod gcd;
pub mod lcm;
pub mod parity;
pub mod prelude;
pub mod saturating;
pub mod wrapping;

pub use checked::{CheckedArithmetic, checked_add, checked_mul, checked_sub};
pub use divisibility::{checked_is_divisible_by, is_divisible_by};
pub use division::{
    checked_div_ceil, checked_div_floor, checked_mod_floor, div_ceil, div_floor, mod_floor,
};
pub use gcd::gcd;
pub use lcm::{checked_lcm, lcm};
pub use parity::{is_even, is_odd};
pub use saturating::{SaturatingArithmetic, saturating_add, saturating_mul, saturating_sub};
pub use wrapping::{WrappingArithmetic, wrapping_add, wrapping_mul, wrapping_sub};
