use core::fmt;

/// Errors produced by integer helper operations.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntegerError {
    /// The requested operation used a zero divisor.
    DivisionByZero,
    /// The requested integer helper overflowed while computing its result.
    ArithmeticOverflow { operation: &'static str },
}

impl fmt::Display for IntegerError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DivisionByZero => {
                formatter.write_str("integer operation requires a non-zero divisor")
            }
            Self::ArithmeticOverflow { operation } => {
                write!(formatter, "integer operation overflowed while computing {operation}")
            }
        }
    }
}

impl std::error::Error for IntegerError {}