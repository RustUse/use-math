#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Algebraic-structure utilities for `RustUse`.

pub mod laws;
pub mod prelude;

pub use laws::{
    has_inverses, identity_element, is_abelian_group, is_associative, is_closed_under,
    is_commutative, is_distributive_over, is_group, is_monoid, is_ring,
};
