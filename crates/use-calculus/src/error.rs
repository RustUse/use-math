use core::fmt;
use std::error::Error;

/// Errors returned by numerical-calculus helpers.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CalculusError {
    /// A sample point or evaluation point must be finite.
    NonFinitePoint {
        /// The name of the point that rejected the value.
        name: &'static str,
        /// The invalid value.
        value: f64,
    },
    /// An integration bound must be finite.
    NonFiniteBound {
        /// The bound name that rejected the value.
        bound: &'static str,
        /// The invalid value.
        value: f64,
    },
    /// A finite-difference or limit step must be finite.
    NonFiniteStep(f64),
    /// A finite-difference or limit step must be positive.
    NonPositiveStep(f64),
    /// A limit tolerance must be finite.
    NonFiniteTolerance(f64),
    /// A limit tolerance must not be negative.
    NegativeTolerance(f64),
    /// The number of subintervals must be positive.
    ZeroSubintervals,
    /// Simpson integration requires an even number of subintervals.
    OddSubintervalCount(usize),
    /// Function evaluation produced `NaN` or infinity.
    NonFiniteEvaluation {
        /// The sample point that produced the invalid value.
        input: f64,
        /// The invalid evaluation result.
        value: f64,
    },
    /// The left and right limit samples disagreed by more than tolerance.
    LimitMismatch {
        /// The left-hand sample.
        left: f64,
        /// The right-hand sample.
        right: f64,
        /// The accepted absolute tolerance.
        tolerance: f64,
    },
}

impl CalculusError {
    pub(crate) const fn validate_point(name: &'static str, value: f64) -> Result<f64, Self> {
        if !value.is_finite() {
            return Err(Self::NonFinitePoint { name, value });
        }

        Ok(value)
    }

    pub(crate) const fn validate_bound(bound: &'static str, value: f64) -> Result<f64, Self> {
        if !value.is_finite() {
            return Err(Self::NonFiniteBound { bound, value });
        }

        Ok(value)
    }

    pub(crate) const fn validate_step(step: f64) -> Result<f64, Self> {
        if !step.is_finite() {
            return Err(Self::NonFiniteStep(step));
        }

        if step <= 0.0 {
            return Err(Self::NonPositiveStep(step));
        }

        Ok(step)
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

    pub(crate) const fn validate_subintervals(subintervals: usize) -> Result<usize, Self> {
        if subintervals == 0 {
            return Err(Self::ZeroSubintervals);
        }

        Ok(subintervals)
    }

    pub(crate) fn validate_even_subintervals(subintervals: usize) -> Result<usize, Self> {
        Self::validate_subintervals(subintervals)?;

        if !subintervals.is_multiple_of(2_usize) {
            return Err(Self::OddSubintervalCount(subintervals));
        }

        Ok(subintervals)
    }

    pub(crate) const fn validate_evaluation(input: f64, value: f64) -> Result<f64, Self> {
        if !value.is_finite() {
            return Err(Self::NonFiniteEvaluation { input, value });
        }

        Ok(value)
    }
}

impl fmt::Display for CalculusError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFinitePoint { name, value } => {
                write!(formatter, "{name} point must be finite, got {value}")
            },
            Self::NonFiniteBound { bound, value } => {
                write!(formatter, "{bound} bound must be finite, got {value}")
            },
            Self::NonFiniteStep(value) => {
                write!(formatter, "step must be finite, got {value}")
            },
            Self::NonPositiveStep(value) => {
                write!(formatter, "step must be positive, got {value}")
            },
            Self::NonFiniteTolerance(value) => {
                write!(formatter, "tolerance must be finite, got {value}")
            },
            Self::NegativeTolerance(value) => {
                write!(formatter, "tolerance must be non-negative, got {value}")
            },
            Self::ZeroSubintervals => {
                write!(formatter, "subinterval count must be greater than zero")
            },
            Self::OddSubintervalCount(value) => write!(
                formatter,
                "Simpson integration requires an even number of subintervals, got {value}"
            ),
            Self::NonFiniteEvaluation { input, value } => write!(
                formatter,
                "function evaluation must be finite, got {value} at input {input}"
            ),
            Self::LimitMismatch {
                left,
                right,
                tolerance,
            } => write!(
                formatter,
                "left and right limit samples disagree beyond tolerance: left={left}, right={right}, tolerance={tolerance}"
            ),
        }
    }
}

impl Error for CalculusError {}
