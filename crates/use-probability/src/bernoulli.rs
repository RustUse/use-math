use crate::Probability;

/// A Bernoulli distribution with a single success probability.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bernoulli {
    success: Probability,
}

impl Bernoulli {
    /// Creates a Bernoulli model from a validated success probability.
    #[must_use]
    pub const fn new(success: Probability) -> Self {
        Self { success }
    }

    /// Returns the success probability.
    #[must_use]
    pub const fn success_probability(&self) -> Probability {
        self.success
    }

    /// Returns the failure probability.
    #[must_use]
    pub const fn failure_probability(&self) -> Probability {
        self.success.complement()
    }

    /// Returns the mean of the Bernoulli model.
    #[must_use]
    pub const fn mean(&self) -> f64 {
        self.success.value()
    }

    /// Returns the variance of the Bernoulli model.
    #[must_use]
    pub fn variance(&self) -> f64 {
        let success = self.success.value();
        let failure = self.failure_probability().value();

        success * failure
    }

    /// Returns the probability mass for `success`.
    #[must_use]
    pub const fn pmf(&self, success: bool) -> Probability {
        if success {
            self.success
        } else {
            self.failure_probability()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Bernoulli;
    use crate::{Probability, ProbabilityError};

    fn assert_close(left: f64, right: f64, tolerance: f64) {
        assert!(
            (left - right).abs() <= tolerance,
            "expected {left} to be within {tolerance} of {right}"
        );
    }

    #[test]
    fn exposes_success_and_failure_probabilities() -> Result<(), ProbabilityError> {
        let model = Bernoulli::new(Probability::from_fraction(1, 4)?);

        assert_eq!(model.success_probability(), Probability::try_new(0.25)?);
        assert_eq!(model.failure_probability(), Probability::try_new(0.75)?);

        Ok(())
    }

    #[test]
    fn computes_mean_variance_and_mass_values() -> Result<(), ProbabilityError> {
        let model = Bernoulli::new(Probability::from_fraction(1, 4)?);

        assert_close(model.mean(), 0.25, 1.0e-12);
        assert_close(model.variance(), 0.1875, 1.0e-12);
        assert_eq!(model.pmf(true), Probability::try_new(0.25)?);
        assert_eq!(model.pmf(false), Probability::try_new(0.75)?);

        Ok(())
    }
}