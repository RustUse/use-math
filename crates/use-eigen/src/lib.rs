#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Small structural eigenvalue primitives for `RustUse`.

pub mod eigenpair;
pub mod eigenspace;
pub mod eigensystem;
pub mod eigenvalue;
pub mod eigenvector;
pub mod error;
pub mod multiplicity;

pub use eigenpair::Eigenpair;
pub use eigenspace::EigenSpace;
pub use eigensystem::Eigensystem;
pub use eigenvalue::Eigenvalue;
pub use eigenvector::Eigenvector;
pub use error::EigenError;
pub use multiplicity::EigenMultiplicity;
