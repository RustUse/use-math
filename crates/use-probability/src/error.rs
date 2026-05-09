use core::fmt;
use std::error::Error;

/// Errors returned by validated probability helpers.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProbabilityError {
    /// A probability value must be finite.
    NonFiniteProbability(f64),
    /// A probability value must stay in the closed interval `[0, 1]`.
    ProbabilityOutOfRange(f64),
    /// A ratio denominator must be greater than zero.
    ZeroTotal,
    /// A ratio numerator must not exceed the denominator.
    PartExceedsTotal {
        /// The numerator.
        part: u64,
        /// The denominator.
        total: u64,
    },
}

impl ProbabilityError {
    #[allow(clippy::manual_range_contains)]
    pub(crate) const fn validate_probability(value: f64) -> Result<f64, Self> {
        if !value.is_finite() {
            return Err(Self::NonFiniteProbability(value));
        }

        if value < 0.0 || value > 1.0 {
            return Err(Self::ProbabilityOutOfRange(value));
        }

        Ok(value)
    }

    pub(crate) const fn validate_fraction(part: u64, total: u64) -> Result<(), Self> {
        if total == 0 {
            return Err(Self::ZeroTotal);
        }

        if part > total {
            return Err(Self::PartExceedsTotal { part, total });
        }

        Ok(())
    }
}

impl fmt::Display for ProbabilityError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFiniteProbability(value) => {
                write!(formatter, "probability must be finite, got {value}")
            },
            Self::ProbabilityOutOfRange(value) => {
                write!(formatter, "probability must be in [0, 1], got {value}")
            },
            Self::ZeroTotal => write!(formatter, "total count must be greater than zero"),
            Self::PartExceedsTotal { part, total } => {
                write!(formatter, "part must be less than or equal to total, got part={part}, total={total}")
            },
        }
    }
}

impl Error for ProbabilityError {}