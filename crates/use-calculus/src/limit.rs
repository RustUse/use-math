use crate::CalculusError;

/// Symmetric two-sided limit approximation settings.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LimitApproximator {
    step: f64,
    tolerance: f64,
}

impl LimitApproximator {
    /// Creates a limit approximator from a sample step and comparison
    /// tolerance.
    #[must_use]
    pub const fn new(step: f64, tolerance: f64) -> Self {
        Self { step, tolerance }
    }

    /// Creates a limit approximator from a finite positive step and a finite
    /// non-negative tolerance.
    ///
    /// # Errors
    ///
    /// Returns [`CalculusError::NonFiniteStep`] or
    /// [`CalculusError::NonPositiveStep`] when `step` is invalid, and returns
    /// [`CalculusError::NonFiniteTolerance`] or
    /// [`CalculusError::NegativeTolerance`] when `tolerance` is invalid.
    pub fn try_new(step: f64, tolerance: f64) -> Result<Self, CalculusError> {
        CalculusError::validate_step(step)?;
        CalculusError::validate_tolerance(tolerance)?;

        Ok(Self::new(step, tolerance))
    }

    /// Validates that the stored step and tolerance are acceptable.
    ///
    /// # Errors
    ///
    /// Returns the same error variants as [`Self::try_new`].
    pub fn validate(self) -> Result<Self, CalculusError> {
        Self::try_new(self.step, self.tolerance)
    }

    /// Returns the symmetric sample step.
    #[must_use]
    pub const fn step(&self) -> f64 {
        self.step
    }

    /// Returns the acceptance tolerance between the left and right samples.
    #[must_use]
    pub const fn tolerance(&self) -> f64 {
        self.tolerance
    }

    /// Approximates a two-sided limit at `at` using one symmetric sample scale.
    ///
    /// # Errors
    ///
    /// Returns [`CalculusError`] when the stored step or tolerance is invalid,
    /// `at` is not finite, sampled evaluations are not finite, or the left and
    /// right samples disagree by more than `tolerance`.
    pub fn at<F>(self, function: F, at: f64) -> Result<f64, CalculusError>
    where
        F: FnMut(f64) -> f64,
    {
        symmetric_limit(function, at, self.step, self.tolerance)
    }
}

/// Approximates a two-sided limit with one symmetric sample scale.
///
/// # Errors
///
/// Returns [`CalculusError`] when `step` or `tolerance` is invalid, `at` is
/// not finite, sampled evaluations are not finite, or the left and right
/// samples disagree by more than `tolerance`.
///
/// # Examples
///
/// ```
/// use use_calculus::symmetric_limit;
///
/// let limit = symmetric_limit(
///     |x| {
///         if x == 0.0 {
///             1.0
///         } else {
///             x.sin() / x
///         }
///     },
///     0.0,
///     1.0e-6,
///     1.0e-5,
/// )?;
///
/// assert!((limit - 1.0).abs() < 1.0e-5);
/// # Ok::<(), use_calculus::CalculusError>(())
/// ```
#[must_use = "limit estimates should be used or handled"]
pub fn symmetric_limit<F>(
    mut function: F,
    at: f64,
    step: f64,
    tolerance: f64,
) -> Result<f64, CalculusError>
where
    F: FnMut(f64) -> f64,
{
    let at = CalculusError::validate_point("at", at)?;
    let step = CalculusError::validate_step(step)?;
    let tolerance = CalculusError::validate_tolerance(tolerance)?;
    let left = evaluate(&mut function, at - step)?;
    let right = evaluate(&mut function, at + step)?;

    if (left - right).abs() > tolerance {
        return Err(CalculusError::LimitMismatch {
            left,
            right,
            tolerance,
        });
    }

    Ok(f64::midpoint(left, right))
}

fn evaluate<F>(function: &mut F, input: f64) -> Result<f64, CalculusError>
where
    F: FnMut(f64) -> f64,
{
    let input = CalculusError::validate_point("sample", input)?;
    let value = function(input);

    CalculusError::validate_evaluation(input, value)
}

#[cfg(test)]
mod tests {
    use super::{CalculusError, LimitApproximator, symmetric_limit};

    fn assert_close(left: f64, right: f64, tolerance: f64) {
        assert!(
            (left - right).abs() <= tolerance,
            "expected {left} to be within {tolerance} of {right}"
        );
    }

    #[test]
    fn validates_limit_configuration() {
        assert!(matches!(
            LimitApproximator::try_new(0.0, 1.0e-4),
            Err(CalculusError::NonPositiveStep(0.0))
        ));
        assert!(matches!(
            LimitApproximator::try_new(1.0e-4, -1.0),
            Err(CalculusError::NegativeTolerance(-1.0))
        ));
    }

    #[test]
    fn approximates_two_sided_limits() -> Result<(), CalculusError> {
        let limit = symmetric_limit(
            |x| {
                if x == 0.0 { 1.0 } else { x.sin() / x }
            },
            0.0,
            1.0e-6,
            1.0e-5,
        )?;

        assert_close(limit, 1.0, 1.0e-5);
        Ok(())
    }

    #[test]
    fn rejects_mismatched_limits() {
        assert!(matches!(
            symmetric_limit(|x| if x < 0.0 { -1.0 } else { 1.0 }, 0.0, 1.0e-6, 1.0e-3,),
            Err(CalculusError::LimitMismatch { .. })
        ));
    }
}
