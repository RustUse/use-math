use core::fmt;
use std::error::Error;

/// Errors returned by validated real-number helpers.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RealError {
    /// A real-number component must be finite.
    NonFiniteValue {
        /// The input name that rejected the value.
        name: &'static str,
        /// The invalid value.
        value: f64,
    },
    /// A comparison tolerance must be finite.
    NonFiniteTolerance(f64),
    /// A comparison tolerance must not be negative.
    NegativeTolerance(f64),
    /// An interval must have `min <= max`.
    InvalidInterval {
        /// The lower bound candidate.
        min: f64,
        /// The upper bound candidate.
        max: f64,
    },
}

impl RealError {
    pub(crate) const fn validate_value(name: &'static str, value: f64) -> Result<f64, Self> {
        if !value.is_finite() {
            return Err(Self::NonFiniteValue { name, value });
        }

        Ok(value)
    }

    pub(crate) const fn validate_tolerance(tolerance: f64) -> Result<f64, Self> {
        if !tolerance.is_finite() {
            return Err(Self::NonFiniteTolerance(tolerance));
        }

        if tolerance < 0.0 {
            return Err(Self::NegativeTolerance(tolerance));
        }

        Ok(tolerance)
    }
}

impl fmt::Display for RealError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFiniteValue { name, value } => {
                write!(formatter, "{name} must be finite, got {value}")
            },
            Self::NonFiniteTolerance(value) => {
                write!(formatter, "tolerance must be finite, got {value}")
            },
            Self::NegativeTolerance(value) => {
                write!(formatter, "tolerance must be non-negative, got {value}")
            },
            Self::InvalidInterval { min, max } => {
                write!(formatter, "interval min must not exceed max, got min={min}, max={max}")
            },
        }
    }
}

impl Error for RealError {}