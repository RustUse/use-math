use crate::CalculusError;

/// Finite-difference differentiation configuration.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Differentiator {
    step: f64,
}

impl Differentiator {
    /// Creates a differentiator with the given sample step.
    #[must_use]
    pub const fn new(step: f64) -> Self {
        Self { step }
    }

    /// Creates a differentiator from a finite positive step.
    ///
    /// # Errors
    ///
    /// Returns [`CalculusError::NonFiniteStep`] when `step` is `NaN` or
    /// infinite, and [`CalculusError::NonPositiveStep`] when `step <= 0.0`.
    ///
    /// # Examples
    ///
    /// ```
    /// use use_calculus::{CalculusError, Differentiator};
    ///
    /// let differentiator = Differentiator::try_new(1.0e-4)?;
    /// assert_eq!(differentiator, Differentiator::new(1.0e-4));
    ///
    /// assert!(matches!(
    ///     Differentiator::try_new(0.0),
    ///     Err(CalculusError::NonPositiveStep(0.0))
    /// ));
    /// # Ok::<(), CalculusError>(())
    /// ```
    pub fn try_new(step: f64) -> Result<Self, CalculusError> {
        CalculusError::validate_step(step)?;
        Ok(Self::new(step))
    }

    /// Validates that the stored step is finite and positive.
    ///
    /// # Errors
    ///
    /// Returns the same error variants as [`Self::try_new`].
    pub fn validate(self) -> Result<Self, CalculusError> {
        Self::try_new(self.step)
    }

    /// Returns the sample step.
    #[must_use]
    pub const fn step(&self) -> f64 {
        self.step
    }

    /// Approximates the first derivative using the central-difference formula.
    ///
    /// # Errors
    ///
    /// Returns [`CalculusError`] when the stored step is invalid, `at` is not
    /// finite, or sampled evaluations are not finite.
    ///
    /// # Examples
    ///
    /// ```
    /// use use_calculus::Differentiator;
    ///
    /// let differentiator = Differentiator::try_new(1.0e-5)?;
    /// let slope = differentiator.derivative_at(|x| x.powi(2), 3.0)?;
    ///
    /// assert!((slope - 6.0).abs() < 1.0e-6);
    /// # Ok::<(), use_calculus::CalculusError>(())
    /// ```
    pub fn derivative_at<F>(self, function: F, at: f64) -> Result<f64, CalculusError>
    where
        F: FnMut(f64) -> f64,
    {
        central_difference(function, at, self.step)
    }

    /// Approximates the second derivative using the central-difference formula.
    ///
    /// # Errors
    ///
    /// Returns [`CalculusError`] when the stored step is invalid, `at` is not
    /// finite, or sampled evaluations are not finite.
    pub fn second_derivative_at<F>(self, function: F, at: f64) -> Result<f64, CalculusError>
    where
        F: FnMut(f64) -> f64,
    {
        second_central_difference(function, at, self.step)
    }
}

/// Approximates the first derivative with a central difference.
///
/// # Errors
///
/// Returns [`CalculusError`] when `step` is invalid, `at` is not finite, or
/// sampled evaluations are not finite.
#[must_use = "derivative estimates should be used or handled"]
pub fn central_difference<F>(
    mut function: F,
    at: f64,
    step: f64,
) -> Result<f64, CalculusError>
where
    F: FnMut(f64) -> f64,
{
    let at = CalculusError::validate_point("at", at)?;
    let step = CalculusError::validate_step(step)?;
    let left = evaluate(&mut function, at - step)?;
    let right = evaluate(&mut function, at + step)?;

    Ok((right - left) / (2.0 * step))
}

/// Approximates the second derivative with a central difference.
///
/// # Errors
///
/// Returns [`CalculusError`] when `step` is invalid, `at` is not finite, or
/// sampled evaluations are not finite.
#[must_use = "second-derivative estimates should be used or handled"]
pub fn second_central_difference<F>(
    mut function: F,
    at: f64,
    step: f64,
) -> Result<f64, CalculusError>
where
    F: FnMut(f64) -> f64,
{
    let at = CalculusError::validate_point("at", at)?;
    let step = CalculusError::validate_step(step)?;
    let left = evaluate(&mut function, at - step)?;
    let center = evaluate(&mut function, at)?;
    let right = evaluate(&mut function, at + step)?;
    let step_squared = step * step;
    let numerator = (-2.0_f64).mul_add(center, left + right);

    Ok(numerator / step_squared)
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
    use super::{CalculusError, Differentiator, central_difference, second_central_difference};

    fn assert_close(left: f64, right: f64, tolerance: f64) {
        assert!(
            (left - right).abs() <= tolerance,
            "expected {left} to be within {tolerance} of {right}"
        );
    }

    #[test]
    fn validates_differentiator_steps() {
        assert!(matches!(
            Differentiator::try_new(f64::INFINITY),
            Err(CalculusError::NonFiniteStep(f64::INFINITY))
        ));
        assert!(matches!(
            Differentiator::try_new(0.0),
            Err(CalculusError::NonPositiveStep(0.0))
        ));
    }

    #[test]
    fn computes_first_derivatives() -> Result<(), CalculusError> {
        let slope = central_difference(|x| x.powi(2), 3.0, 1.0e-5)?;

        assert_close(slope, 6.0, 1.0e-6);
        Ok(())
    }

    #[test]
    fn computes_second_derivatives() -> Result<(), CalculusError> {
        let curvature = second_central_difference(|x| x.powi(2), 1.5, 1.0e-4)?;

        assert_close(curvature, 2.0, 1.0e-6);
        Ok(())
    }

    #[test]
    fn rejects_non_finite_points() {
        assert!(matches!(
            central_difference(|x| x, f64::NAN, 1.0e-5),
            Err(CalculusError::NonFinitePoint { name: "at", .. })
        ));
    }

    #[test]
    fn rejects_non_finite_evaluations() {
        assert!(matches!(
            central_difference(|_| f64::NAN, 1.0, 1.0e-5),
            Err(CalculusError::NonFiniteEvaluation { .. })
        ));
    }
}