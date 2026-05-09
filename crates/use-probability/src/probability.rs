use core::fmt;

use crate::ProbabilityError;

/// A validated probability value in the closed interval `[0, 1]`.
#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct Probability {
    value: f64,
}

impl Probability {
    /// Creates a probability without validation.
    #[must_use]
    pub const fn new(value: f64) -> Self {
        Self { value }
    }

    /// Creates a probability from a finite value in `[0, 1]`.
    ///
    /// # Errors
    ///
    /// Returns [`ProbabilityError::NonFiniteProbability`] when `value` is
    /// `NaN` or infinite, and [`ProbabilityError::ProbabilityOutOfRange`] when
    /// `value` is outside `[0, 1]`.
    ///
    /// # Examples
    ///
    /// ```
    /// use use_probability::{Probability, ProbabilityError};
    ///
    /// let probability = Probability::try_new(0.25)?;
    /// assert_eq!(probability, Probability::new(0.25));
    ///
    /// assert!(matches!(
    ///     Probability::try_new(1.5),
    ///     Err(ProbabilityError::ProbabilityOutOfRange(1.5))
    /// ));
    /// # Ok::<(), ProbabilityError>(())
    /// ```
    pub const fn try_new(value: f64) -> Result<Self, ProbabilityError> {
        match ProbabilityError::validate_probability(value) {
            Ok(value) => Ok(Self::new(value)),
            Err(error) => Err(error),
        }
    }

    /// Creates a probability from `part / total`.
    ///
    /// # Errors
    ///
    /// Returns [`ProbabilityError::ZeroTotal`] when `total == 0`, and
    /// [`ProbabilityError::PartExceedsTotal`] when `part > total`.
    ///
    /// # Examples
    ///
    /// ```
    /// use use_probability::Probability;
    ///
    /// let probability = Probability::from_fraction(1, 4)?;
    /// assert!((probability.value() - 0.25).abs() < 1.0e-12);
    /// # Ok::<(), use_probability::ProbabilityError>(())
    /// ```
    #[allow(clippy::cast_precision_loss)]
    pub fn from_fraction(part: u64, total: u64) -> Result<Self, ProbabilityError> {
        ProbabilityError::validate_fraction(part, total)?;
        Ok(Self::new(part as f64 / total as f64))
    }

    /// Validates that an existing probability remains normalized.
    ///
    /// # Errors
    ///
    /// Returns the same error variants as [`Self::try_new`].
    pub const fn validate(self) -> Result<Self, ProbabilityError> {
        Self::try_new(self.value)
    }

    /// Returns the stored probability value.
    #[must_use]
    pub const fn value(&self) -> f64 {
        self.value
    }

    /// Returns the stored probability as a percentage.
    #[must_use]
    pub const fn as_percentage(&self) -> f64 {
        self.value * 100.0
    }

    /// Returns the complementary probability `1 - p`.
    #[must_use]
    pub const fn complement(self) -> Self {
        Self::new(1.0 - self.value)
    }

    /// Returns the impossible event probability `0`.
    #[must_use]
    pub const fn impossible() -> Self {
        Self::new(0.0)
    }

    /// Returns the certain event probability `1`.
    #[must_use]
    pub const fn certainty() -> Self {
        Self::new(1.0)
    }

    /// Returns the probability of two independent events both happening.
    #[must_use]
    pub const fn intersection_independent(self, other: Self) -> Self {
        Self::new(self.value * other.value)
    }

    /// Returns the probability of at least one of two independent events
    /// happening.
    #[must_use]
    pub fn union_independent(self, other: Self) -> Self {
        Self::new(self.value.mul_add(-other.value, self.value + other.value))
    }
}

impl From<Probability> for f64 {
    fn from(value: Probability) -> Self {
        value.value
    }
}

impl fmt::Display for Probability {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.value)
    }
}

/// Returns the probability of two independent events both happening.
#[must_use]
pub const fn independent_intersection(left: Probability, right: Probability) -> Probability {
    left.intersection_independent(right)
}

/// Returns the probability of at least one of two independent events
/// happening.
#[must_use]
pub fn independent_union(left: Probability, right: Probability) -> Probability {
    left.union_independent(right)
}

#[cfg(test)]
mod tests {
    use super::{Probability, ProbabilityError, independent_intersection, independent_union};

    fn assert_close(left: f64, right: f64, tolerance: f64) {
        assert!(
            (left - right).abs() <= tolerance,
            "expected {left} to be within {tolerance} of {right}"
        );
    }

    #[test]
    fn validates_probability_values() {
        assert!(matches!(
            Probability::try_new(f64::NAN),
            Err(ProbabilityError::NonFiniteProbability(_))
        ));
        assert!(matches!(
            Probability::try_new(-0.01),
            Err(ProbabilityError::ProbabilityOutOfRange(_))
        ));
        assert!(matches!(
            Probability::try_new(1.01),
            Err(ProbabilityError::ProbabilityOutOfRange(_))
        ));
    }

    #[test]
    fn constructs_from_fractions() -> Result<(), ProbabilityError> {
        let probability = Probability::from_fraction(1, 4)?;

        assert_close(probability.value(), 0.25, 1.0e-12);
        Ok(())
    }

    #[test]
    fn rejects_invalid_fractions() {
        assert!(matches!(
            Probability::from_fraction(1, 0),
            Err(ProbabilityError::ZeroTotal)
        ));
        assert!(matches!(
            Probability::from_fraction(3, 2),
            Err(ProbabilityError::PartExceedsTotal { .. })
        ));
    }

    #[test]
    fn computes_complements_and_percentages() -> Result<(), ProbabilityError> {
        let probability = Probability::from_fraction(1, 4)?;

        assert_eq!(probability.complement(), Probability::try_new(0.75)?);
        assert_close(probability.as_percentage(), 25.0, 1.0e-12);
        assert_eq!(Probability::impossible(), Probability::try_new(0.0)?);
        assert_eq!(Probability::certainty(), Probability::try_new(1.0)?);

        Ok(())
    }

    #[test]
    fn computes_independent_event_combinations() -> Result<(), ProbabilityError> {
        let left = Probability::from_fraction(1, 4)?;
        let right = Probability::try_new(0.5)?;

        assert_close(independent_intersection(left, right).value(), 0.125, 1.0e-12);
        assert_close(independent_union(left, right).value(), 0.625, 1.0e-12);

        Ok(())
    }
}