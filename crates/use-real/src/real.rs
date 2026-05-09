use core::fmt;

use crate::RealError;

/// A validated finite `f64` value.
#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct Real {
    value: f64,
}

/// A checked closed interval `[min, max]` over finite real values.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RealInterval {
    min: Real,
    max: Real,
}

impl Real {
    /// Creates a real value without validation.
    #[must_use]
    pub const fn new(value: f64) -> Self {
        Self { value }
    }

    /// Creates a real value from a finite `f64`.
    ///
    /// # Errors
    ///
    /// Returns [`RealError::NonFiniteValue`] when `value` is `NaN` or
    /// infinite.
    ///
    /// # Examples
    ///
    /// ```
    /// use use_real::{Real, RealError};
    ///
    /// let real = Real::try_new(2.5)?;
    /// assert_eq!(real, Real::new(2.5));
    ///
    /// assert!(matches!(
    ///     Real::try_new(f64::INFINITY),
    ///     Err(RealError::NonFiniteValue { .. })
    /// ));
    /// # Ok::<(), RealError>(())
    /// ```
    pub const fn try_new(value: f64) -> Result<Self, RealError> {
        match RealError::validate_value("value", value) {
            Ok(value) => Ok(Self::new(value)),
            Err(error) => Err(error),
        }
    }

    /// Validates that an existing real value remains finite.
    ///
    /// # Errors
    ///
    /// Returns the same error variants as [`Self::try_new`].
    pub const fn validate(self) -> Result<Self, RealError> {
        Self::try_new(self.value)
    }

    /// Returns the stored `f64` value.
    #[must_use]
    pub const fn value(&self) -> f64 {
        self.value
    }

    /// Returns `0.0` as a validated real value.
    #[must_use]
    pub const fn zero() -> Self {
        Self::new(0.0)
    }

    /// Returns `1.0` as a validated real value.
    #[must_use]
    pub const fn one() -> Self {
        Self::new(1.0)
    }

    /// Returns the absolute value.
    #[must_use]
    pub const fn abs(self) -> Self {
        Self::new(self.value.abs())
    }
}

impl fmt::Display for Real {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.value)
    }
}

impl RealInterval {
    /// Creates an interval from two validated bounds without rechecking order.
    #[must_use]
    pub const fn new(min: Real, max: Real) -> Self {
        Self { min, max }
    }

    /// Creates an interval from finite `min` and `max` bounds.
    ///
    /// # Errors
    ///
    /// Returns [`RealError::NonFiniteValue`] when either bound is not finite,
    /// and [`RealError::InvalidInterval`] when `min > max`.
    ///
    /// # Examples
    ///
    /// ```
    /// use use_real::RealInterval;
    ///
    /// let interval = RealInterval::try_new(-2.0, 6.0)?;
    /// assert!((interval.width() - 8.0).abs() < 1.0e-12);
    /// # Ok::<(), use_real::RealError>(())
    /// ```
    pub fn try_new(min: f64, max: f64) -> Result<Self, RealError> {
        let min = Real::try_new(min)?;
        let max = Real::try_new(max)?;

        if min.value() > max.value() {
            return Err(RealError::InvalidInterval {
                min: min.value(),
                max: max.value(),
            });
        }

        Ok(Self::new(min, max))
    }

    /// Validates that an existing interval remains ordered.
    ///
    /// # Errors
    ///
    /// Returns the same error variants as [`Self::try_new`].
    pub fn validate(self) -> Result<Self, RealError> {
        Self::try_new(self.min.value(), self.max.value())
    }

    /// Returns the lower bound.
    #[must_use]
    pub const fn min(&self) -> Real {
        self.min
    }

    /// Returns the upper bound.
    #[must_use]
    pub const fn max(&self) -> Real {
        self.max
    }

    /// Returns the interval width, `max - min`.
    #[must_use]
    pub const fn width(&self) -> f64 {
        self.max.value() - self.min.value()
    }

    /// Returns the midpoint of the interval.
    #[must_use]
    pub const fn midpoint(&self) -> Real {
        Real::new(f64::midpoint(self.min.value(), self.max.value()))
    }

    /// Returns `true` when `value` lies inside the closed interval.
    #[must_use]
    pub const fn contains(&self, value: Real) -> bool {
        value.value() >= self.min.value() && value.value() <= self.max.value()
    }

    /// Clamps `value` into the interval.
    #[must_use]
    pub const fn clamp(&self, value: Real) -> Real {
        Real::new(value.value().clamp(self.min.value(), self.max.value()))
    }

    /// Returns `true` when the interval has zero width.
    #[must_use]
    pub const fn is_degenerate(&self) -> bool {
        self.min.value().to_bits() == self.max.value().to_bits()
    }
}

impl fmt::Display for RealInterval {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "[{}, {}]", self.min, self.max)
    }
}

/// Compares two real values with an explicit non-negative tolerance.
///
/// # Errors
///
/// Returns [`RealError::NonFiniteTolerance`] when `tolerance` is not finite,
/// and [`RealError::NegativeTolerance`] when `tolerance < 0.0`.
///
/// # Examples
///
/// ```
/// use use_real::{Real, approx_eq};
///
/// let left = Real::try_new(1.0)?;
/// let right = Real::try_new(1.0 + 1.0e-10)?;
///
/// assert!(approx_eq(left, right, 1.0e-9)?);
/// # Ok::<(), use_real::RealError>(())
/// ```
pub fn approx_eq(left: Real, right: Real, tolerance: f64) -> Result<bool, RealError> {
    let tolerance = RealError::validate_tolerance(tolerance)?;

    Ok((left.value() - right.value()).abs() <= tolerance)
}

#[cfg(test)]
mod tests {
    use super::{Real, RealInterval, approx_eq};
    use crate::RealError;

    fn assert_close(left: f64, right: f64, tolerance: f64) {
        assert!(
            (left - right).abs() <= tolerance,
            "expected {left} to be within {tolerance} of {right}"
        );
    }

    #[test]
    fn validates_real_values() {
        assert!(matches!(
            Real::try_new(f64::NAN),
            Err(RealError::NonFiniteValue { .. })
        ));
    }

    #[test]
    fn exposes_abs_zero_and_one() -> Result<(), RealError> {
        let value = Real::try_new(-3.5)?;

        assert_eq!(value.abs(), Real::try_new(3.5)?);
        assert_eq!(Real::zero(), Real::try_new(0.0)?);
        assert_eq!(Real::one(), Real::try_new(1.0)?);

        Ok(())
    }

    #[test]
    fn compares_values_with_explicit_tolerance() -> Result<(), RealError> {
        let left = Real::try_new(1.0)?;
        let right = Real::try_new(1.0 + 1.0e-10)?;

        assert!(approx_eq(left, right, 1.0e-9)?);
        assert!(!approx_eq(left, right, 1.0e-12)?);

        Ok(())
    }

    #[test]
    fn validates_interval_bounds() {
        assert!(matches!(
            RealInterval::try_new(2.0, -2.0),
            Err(RealError::InvalidInterval { .. })
        ));
    }

    #[test]
    fn computes_interval_properties() -> Result<(), RealError> {
        let interval = RealInterval::try_new(-2.0, 6.0)?;

        assert_close(interval.width(), 8.0, 1.0e-12);
        assert_close(interval.midpoint().value(), 2.0, 1.0e-12);
        assert!(interval.contains(Real::try_new(1.5)?));
        assert_eq!(interval.clamp(Real::try_new(8.0)?), Real::try_new(6.0)?);
        assert!(!interval.is_degenerate());

        Ok(())
    }
}
