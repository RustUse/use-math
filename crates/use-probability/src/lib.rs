#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Small probability primitives for `RustUse`.

pub mod bernoulli;
pub mod error;
pub mod prelude;
pub mod probability;

pub use bernoulli::Bernoulli;
pub use error::ProbabilityError;
pub use probability::{Probability, independent_intersection, independent_union};
