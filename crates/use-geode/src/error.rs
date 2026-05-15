use core::fmt;
use std::error::Error;

/// Errors returned by checked Geode helpers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeodeError {
    /// Type vectors must contain at least one component.
    EmptyTypeVector,
    /// A requested component index was not present in the type vector.
    IndexOutOfBounds,
    /// A checked arithmetic operation overflowed `u64` or `u128`.
    ArithmeticOverflow,
    /// An exact division request had a zero divisor or non-zero remainder.
    DivisionNotExact,
    /// Input values violated a crate invariant.
    InvalidInput,
}

impl fmt::Display for GeodeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyTypeVector => {
                formatter.write_str("type vectors must contain at least one component")
            },
            Self::IndexOutOfBounds => formatter.write_str("type vector index is out of bounds"),
            Self::ArithmeticOverflow => {
                formatter.write_str("geode arithmetic overflowed its exact integer representation")
            },
            Self::DivisionNotExact => formatter.write_str("geode division was not exact"),
            Self::InvalidInput => formatter.write_str("geode input violated a crate invariant"),
        }
    }
}

impl Error for GeodeError {}
