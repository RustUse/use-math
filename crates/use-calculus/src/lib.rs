#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Small numerical-calculus helpers for `RustUse`.

pub mod derivative;
pub mod error;
pub mod integral;
pub mod limit;
pub mod prelude;

pub use derivative::{Differentiator, central_difference, second_central_difference};
pub use error::CalculusError;
pub use integral::{IntegrationInterval, Integrator, simpson_integral, trapezoidal_integral};
pub use limit::{LimitApproximator, symmetric_limit};
