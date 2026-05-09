use core::fmt;

use crate::RationalError;

/// A normalized exact rational number.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rational {
    numerator: i128,
    denominator: i128,
}

impl Rational {
    /// Returns the exact integer `value / 1`.
    #[must_use]
    pub const fn from_integer(value: i128) -> Self {
        Self {
            numerator: value,
            denominator: 1,
        }
    }

    /// Returns `0 / 1`.
    #[must_use]
    pub const fn zero() -> Self {
        Self::from_integer(0)
    }

    /// Returns `1 / 1`.
    #[must_use]
    pub const fn one() -> Self {
        Self::from_integer(1)
    }

    /// Creates a normalized rational number from a numerator and denominator.
    ///
    /// # Errors
    ///
    /// Returns [`RationalError::ZeroDenominator`] when `denominator == 0`, and
    /// returns overflow-related errors when canonical sign normalization cannot
    /// be represented in the current integer type.
    ///
    /// # Examples
    ///
    /// ```
    /// use use_rational::{Rational, RationalError};
    ///
    /// let rational = Rational::try_new(2, 4)?;
    /// assert_eq!(rational, Rational::try_new(1, 2)?);
    ///
    /// assert!(matches!(
    ///     Rational::try_new(1, 0),
    ///     Err(RationalError::ZeroDenominator)
    /// ));
    /// # Ok::<(), RationalError>(())
    /// ```
    pub fn try_new(numerator: i128, denominator: i128) -> Result<Self, RationalError> {
        normalize(numerator, denominator)
    }

    /// Returns the normalized numerator.
    #[must_use]
    pub const fn numerator(&self) -> i128 {
        self.numerator
    }

    /// Returns the normalized positive denominator.
    #[must_use]
    pub const fn denominator(&self) -> i128 {
        self.denominator
    }

    /// Returns `true` when the rational is an integer.
    #[must_use]
    pub const fn is_integer(&self) -> bool {
        self.denominator == 1
    }

    /// Returns the exact integer value when the rational is integral.
    #[must_use]
    pub const fn to_integer(self) -> Option<i128> {
        if self.is_integer() {
            Some(self.numerator)
        } else {
            None
        }
    }

    /// Returns the reciprocal when the rational is non-zero.
    ///
    /// # Errors
    ///
    /// Returns [`RationalError::DivisionByZero`] when `self == 0`.
    pub fn reciprocal(self) -> Result<Self, RationalError> {
        if self.numerator == 0 {
            return Err(RationalError::DivisionByZero);
        }

        normalize(self.denominator, self.numerator)
    }

    /// Adds two rational numbers exactly.
    ///
    /// # Errors
    ///
    /// Returns [`RationalError::ArithmeticOverflow`] when the intermediate or
    /// normalized result cannot be represented exactly.
    pub fn checked_add(self, other: Self) -> Result<Self, RationalError> {
        let left = self
            .numerator
            .checked_mul(other.denominator)
            .ok_or(RationalError::ArithmeticOverflow { operation: "addition" })?;
        let right = other
            .numerator
            .checked_mul(self.denominator)
            .ok_or(RationalError::ArithmeticOverflow { operation: "addition" })?;
        let numerator = left
            .checked_add(right)
            .ok_or(RationalError::ArithmeticOverflow { operation: "addition" })?;
        let denominator = self
            .denominator
            .checked_mul(other.denominator)
            .ok_or(RationalError::ArithmeticOverflow { operation: "addition" })?;

        normalize(numerator, denominator)
    }

    /// Subtracts two rational numbers exactly.
    ///
    /// # Errors
    ///
    /// Returns [`RationalError::ArithmeticOverflow`] when the intermediate or
    /// normalized result cannot be represented exactly.
    pub fn checked_sub(self, other: Self) -> Result<Self, RationalError> {
        let left = self
            .numerator
            .checked_mul(other.denominator)
            .ok_or(RationalError::ArithmeticOverflow { operation: "subtraction" })?;
        let right = other
            .numerator
            .checked_mul(self.denominator)
            .ok_or(RationalError::ArithmeticOverflow { operation: "subtraction" })?;
        let numerator = left
            .checked_sub(right)
            .ok_or(RationalError::ArithmeticOverflow { operation: "subtraction" })?;
        let denominator = self
            .denominator
            .checked_mul(other.denominator)
            .ok_or(RationalError::ArithmeticOverflow { operation: "subtraction" })?;

        normalize(numerator, denominator)
    }

    /// Multiplies two rational numbers exactly.
    ///
    /// # Errors
    ///
    /// Returns [`RationalError::ArithmeticOverflow`] when the intermediate or
    /// normalized result cannot be represented exactly.
    pub fn checked_mul(self, other: Self) -> Result<Self, RationalError> {
        let numerator = self
            .numerator
            .checked_mul(other.numerator)
            .ok_or(RationalError::ArithmeticOverflow { operation: "multiplication" })?;
        let denominator = self
            .denominator
            .checked_mul(other.denominator)
            .ok_or(RationalError::ArithmeticOverflow { operation: "multiplication" })?;

        normalize(numerator, denominator)
    }

    /// Divides two rational numbers exactly.
    ///
    /// # Errors
    ///
    /// Returns [`RationalError::DivisionByZero`] when `other == 0`, and returns
    /// [`RationalError::ArithmeticOverflow`] when the exact result cannot be
    /// represented.
    pub fn checked_div(self, other: Self) -> Result<Self, RationalError> {
        if other.numerator == 0 {
            return Err(RationalError::DivisionByZero);
        }

        let numerator = self
            .numerator
            .checked_mul(other.denominator)
            .ok_or(RationalError::ArithmeticOverflow { operation: "division" })?;
        let denominator = self
            .denominator
            .checked_mul(other.numerator)
            .ok_or(RationalError::ArithmeticOverflow { operation: "division" })?;

        normalize(numerator, denominator)
    }

    /// Returns the rational as an approximate `f64`.
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn as_f64(&self) -> f64 {
        self.numerator as f64 / self.denominator as f64
    }
}

impl fmt::Display for Rational {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.denominator == 1 {
            write!(formatter, "{}", self.numerator)
        } else {
            write!(formatter, "{}/{}", self.numerator, self.denominator)
        }
    }
}

fn normalize(numerator: i128, denominator: i128) -> Result<Rational, RationalError> {
    if denominator == 0 {
        return Err(RationalError::ZeroDenominator);
    }

    if numerator == 0 {
        return Ok(Rational::zero());
    }

    let mut numerator = numerator;
    let mut denominator = denominator;

    if denominator < 0 {
        numerator = numerator
            .checked_neg()
            .ok_or(RationalError::NormalizationOverflow)?;
        denominator = denominator
            .checked_neg()
            .ok_or(RationalError::NormalizationOverflow)?;
    }

    let divisor = gcd_u128(numerator.unsigned_abs(), denominator.cast_unsigned());
    let divisor = i128::try_from(divisor).map_err(|_| RationalError::NormalizationOverflow)?;

    Ok(Rational {
        numerator: numerator / divisor,
        denominator: denominator / divisor,
    })
}

const fn gcd_u128(mut left: u128, mut right: u128) -> u128 {
    while right != 0 {
        let remainder = left % right;
        left = right;
        right = remainder;
    }

    left
}

#[cfg(test)]
mod tests {
    use super::Rational;
    use crate::RationalError;

    fn assert_close(left: f64, right: f64, tolerance: f64) {
        assert!(
            (left - right).abs() <= tolerance,
            "expected {left} to be within {tolerance} of {right}"
        );
    }

    #[test]
    fn normalizes_signs_and_reduces_values() -> Result<(), RationalError> {
        assert_eq!(Rational::try_new(2, 4)?, Rational::try_new(1, 2)?);
        assert_eq!(Rational::try_new(3, -9)?, Rational::try_new(-1, 3)?);
        assert_eq!(Rational::try_new(-3, -9)?, Rational::try_new(1, 3)?);

        Ok(())
    }

    #[test]
    fn exposes_integer_and_zero_helpers() {
        assert_eq!(Rational::zero(), Rational::from_integer(0));
        assert_eq!(Rational::one(), Rational::from_integer(1));
        assert!(Rational::from_integer(7).is_integer());
        assert_eq!(Rational::from_integer(7).to_integer(), Some(7));
    }

    #[test]
    fn rejects_zero_denominators() {
        assert!(matches!(
            Rational::try_new(1, 0),
            Err(RationalError::ZeroDenominator)
        ));
    }

    #[test]
    fn computes_checked_arithmetic() -> Result<(), RationalError> {
        let half = Rational::try_new(1, 2)?;
        let third = Rational::try_new(1, 3)?;

        assert_eq!(half.checked_add(third)?, Rational::try_new(5, 6)?);
        assert_eq!(half.checked_sub(third)?, Rational::try_new(1, 6)?);
        assert_eq!(half.checked_mul(third)?, Rational::try_new(1, 6)?);
        assert_eq!(half.checked_div(third)?, Rational::try_new(3, 2)?);

        Ok(())
    }

    #[test]
    fn rejects_division_by_zero() -> Result<(), RationalError> {
        let half = Rational::try_new(1, 2)?;

        assert!(matches!(
            half.checked_div(Rational::zero()),
            Err(RationalError::DivisionByZero)
        ));
        assert!(matches!(
            Rational::zero().reciprocal(),
            Err(RationalError::DivisionByZero)
        ));

        Ok(())
    }

    #[test]
    fn converts_to_f64_explicitly() -> Result<(), RationalError> {
        let rational = Rational::try_new(5, 6)?;

        assert_close(rational.as_f64(), 5.0 / 6.0, 1.0e-12);
        Ok(())
    }
}