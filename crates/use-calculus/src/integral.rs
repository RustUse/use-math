use crate::CalculusError;

/// A finite interval for definite integration.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IntegrationInterval {
    start: f64,
    end: f64,
}

impl IntegrationInterval {
    /// Creates an interval from `start` to `end`.
    #[must_use]
    pub const fn new(start: f64, end: f64) -> Self {
        Self { start, end }
    }

    /// Creates an interval from finite bounds.
    ///
    /// # Errors
    ///
    /// Returns [`CalculusError::NonFiniteBound`] when `start` or `end` is not
    /// finite.
    pub fn try_new(start: f64, end: f64) -> Result<Self, CalculusError> {
        CalculusError::validate_bound("start", start)?;
        CalculusError::validate_bound("end", end)?;

        Ok(Self::new(start, end))
    }

    /// Validates that the stored bounds are finite.
    ///
    /// # Errors
    ///
    /// Returns the same error variants as [`Self::try_new`].
    pub fn validate(self) -> Result<Self, CalculusError> {
        Self::try_new(self.start, self.end)
    }

    /// Returns the start bound.
    #[must_use]
    pub const fn start(&self) -> f64 {
        self.start
    }

    /// Returns the end bound.
    #[must_use]
    pub const fn end(&self) -> f64 {
        self.end
    }

    /// Returns the signed interval width, `end - start`.
    #[must_use]
    pub const fn width(&self) -> f64 {
        self.end - self.start
    }
}

/// Numerical integration configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Integrator {
    subintervals: usize,
}

impl Integrator {
    /// Creates an integrator from a subinterval count.
    #[must_use]
    pub const fn new(subintervals: usize) -> Self {
        Self { subintervals }
    }

    /// Creates an integrator from a positive subinterval count.
    ///
    /// # Errors
    ///
    /// Returns [`CalculusError::ZeroSubintervals`] when `subintervals == 0`.
    pub fn try_new(subintervals: usize) -> Result<Self, CalculusError> {
        CalculusError::validate_subintervals(subintervals)?;
        Ok(Self::new(subintervals))
    }

    /// Validates that the stored subinterval count is positive.
    ///
    /// # Errors
    ///
    /// Returns the same error variants as [`Self::try_new`].
    pub fn validate(self) -> Result<Self, CalculusError> {
        Self::try_new(self.subintervals)
    }

    /// Returns the stored subinterval count.
    #[must_use]
    pub const fn subintervals(&self) -> usize {
        self.subintervals
    }

    /// Approximates a definite integral with the trapezoidal rule.
    ///
    /// # Errors
    ///
    /// Returns [`CalculusError`] when the interval or subinterval count is
    /// invalid, or when sampled evaluations are not finite.
    pub fn trapezoidal<F>(
        self,
        function: F,
        interval: IntegrationInterval,
    ) -> Result<f64, CalculusError>
    where
        F: FnMut(f64) -> f64,
    {
        trapezoidal_integral(function, interval, self.subintervals)
    }

    /// Approximates a definite integral with Simpson's rule.
    ///
    /// # Errors
    ///
    /// Returns [`CalculusError`] when the interval or subinterval count is
    /// invalid, when an odd number of subintervals is used, or when sampled
    /// evaluations are not finite.
    pub fn simpson<F>(self, function: F, interval: IntegrationInterval) -> Result<f64, CalculusError>
    where
        F: FnMut(f64) -> f64,
    {
        simpson_integral(function, interval, self.subintervals)
    }
}

/// Approximates a definite integral with the trapezoidal rule.
///
/// # Errors
///
/// Returns [`CalculusError`] when the interval or subinterval count is
/// invalid, or when sampled evaluations are not finite.
#[must_use = "integral estimates should be used or handled"]
#[allow(clippy::cast_precision_loss)]
pub fn trapezoidal_integral<F>(
    mut function: F,
    interval: IntegrationInterval,
    subintervals: usize,
) -> Result<f64, CalculusError>
where
    F: FnMut(f64) -> f64,
{
    let interval = interval.validate()?;
    let subintervals = CalculusError::validate_subintervals(subintervals)?;
    let step = interval.width() / subintervals as f64;
    let start = interval.start();
    let end = interval.end();
    let first = evaluate(&mut function, start)?;
    let last = evaluate(&mut function, end)?;
    let mut sum = 0.5 * (first + last);

    for index in 1..subintervals {
        let sample = step.mul_add(index as f64, start);
        sum += evaluate(&mut function, sample)?;
    }

    Ok(sum * step)
}

/// Approximates a definite integral with Simpson's rule.
///
/// # Errors
///
/// Returns [`CalculusError`] when the interval or subinterval count is
/// invalid, when an odd number of subintervals is used, or when sampled
/// evaluations are not finite.
///
/// # Examples
///
/// ```
/// use use_calculus::{IntegrationInterval, simpson_integral};
///
/// let interval = IntegrationInterval::try_new(0.0, 1.0)?;
/// let area = simpson_integral(|x| x * x, interval, 64)?;
///
/// assert!((area - (1.0 / 3.0)).abs() < 1.0e-6);
/// # Ok::<(), use_calculus::CalculusError>(())
/// ```
#[must_use = "integral estimates should be used or handled"]
#[allow(clippy::cast_precision_loss)]
pub fn simpson_integral<F>(
    mut function: F,
    interval: IntegrationInterval,
    subintervals: usize,
) -> Result<f64, CalculusError>
where
    F: FnMut(f64) -> f64,
{
    let interval = interval.validate()?;
    let subintervals = CalculusError::validate_even_subintervals(subintervals)?;
    let step = interval.width() / subintervals as f64;
    let start = interval.start();
    let end = interval.end();
    let first = evaluate(&mut function, start)?;
    let last = evaluate(&mut function, end)?;
    let mut odd_sum = 0.0;
    let mut even_sum = 0.0;

    for index in 1..subintervals {
        let sample = step.mul_add(index as f64, start);
        let value = evaluate(&mut function, sample)?;

        if index.is_multiple_of(2_usize) {
            even_sum += value;
        } else {
            odd_sum += value;
        }
    }

    let edge_sum = 4.0_f64.mul_add(odd_sum, first + last);
    let total = 2.0_f64.mul_add(even_sum, edge_sum);

    Ok((step / 3.0) * total)
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
    use super::{CalculusError, IntegrationInterval, Integrator, simpson_integral, trapezoidal_integral};

    fn assert_close(left: f64, right: f64, tolerance: f64) {
        assert!(
            (left - right).abs() <= tolerance,
            "expected {left} to be within {tolerance} of {right}"
        );
    }

    #[test]
    fn validates_interval_bounds() {
        assert!(matches!(
            IntegrationInterval::try_new(f64::NAN, 1.0),
            Err(CalculusError::NonFiniteBound { bound: "start", .. })
        ));
    }

    #[test]
    fn validates_subinterval_counts() {
        assert!(matches!(
            Integrator::try_new(0),
            Err(CalculusError::ZeroSubintervals)
        ));
        assert!(matches!(
            simpson_integral(|x| x, IntegrationInterval::new(0.0, 1.0), 3),
            Err(CalculusError::OddSubintervalCount(3))
        ));
    }

    #[test]
    fn computes_trapezoidal_integrals() -> Result<(), CalculusError> {
        let interval = IntegrationInterval::try_new(0.0, 1.0)?;
        let area = trapezoidal_integral(|x| x * x, interval, 2_048)?;

        assert_close(area, 1.0 / 3.0, 1.0e-6);
        Ok(())
    }

    #[test]
    fn computes_simpson_integrals() -> Result<(), CalculusError> {
        let interval = IntegrationInterval::try_new(-1.0, 1.0)?;
        let area = simpson_integral(|x| x * x, interval, 64)?;

        assert_close(area, 2.0 / 3.0, 1.0e-10);
        Ok(())
    }

    #[test]
    fn rejects_non_finite_evaluations() {
        assert!(matches!(
            trapezoidal_integral(|_| f64::NAN, IntegrationInterval::new(0.0, 1.0), 8),
            Err(CalculusError::NonFiniteEvaluation { .. })
        ));
    }
}