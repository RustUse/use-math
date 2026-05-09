use core::fmt;
use std::error::Error;

/// Errors returned by validated rational-number helpers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RationalError {
    /// A rational denominator must not be zero.
    ZeroDenominator,
    /// Division by a zero-valued rational is undefined.
    DivisionByZero,
    /// Sign normalization could not be represented in the current integer type.
    NormalizationOverflow,
    /// Exact arithmetic overflowed the current integer representation.
    ArithmeticOverflow {
        /// The checked operation that overflowed.
        operation: &'static str,
    },
}

impl fmt::Display for RationalError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroDenominator => write!(formatter, "denominator must not be zero"),
            Self::DivisionByZero => write!(formatter, "division by zero-valued rational"),
            Self::NormalizationOverflow => {
                write!(formatter, "rational normalization overflowed the current representation")
            },
            Self::ArithmeticOverflow { operation } => {
                write!(formatter, "rational {operation} overflowed the current representation")
            },
        }
    }
}

impl Error for RationalError {}