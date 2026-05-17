#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Small approximate numerical helpers for `RustUse`.

#[doc(inline)]
pub use approx::{approx_eq, clamp_epsilon, relative_eq};
#[doc(inline)]
pub use difference::{backward_difference, central_difference, forward_difference};
#[doc(inline)]
pub use integration::{midpoint_rule, rectangle_rule, simpsons_rule, trapezoidal_rule};
#[cfg(feature = "interval")]
#[doc(inline)]
pub use root::bisection_interval;
#[doc(inline)]
pub use root::{RootError, RootOptions, bisection, newton_raphson};

/// Floating-point comparison helpers for approximate numerical work.
pub mod approx {
    /// Normalizes an epsilon to a non-negative finite value.
    ///
    /// Negative finite epsilons are converted to their absolute value. `NaN`
    /// becomes `0.0`, and infinite values clamp to [`f64::MAX`].
    #[must_use]
    pub const fn clamp_epsilon(epsilon: f64) -> f64 {
        if epsilon.is_nan() {
            0.0
        } else if epsilon.is_infinite() {
            f64::MAX
        } else {
            epsilon.abs()
        }
    }

    /// Returns `true` when `a` and `b` are within an absolute epsilon.
    ///
    /// `NaN` never compares equal. Equal infinities compare equal. Mixed
    /// finite and non-finite values compare unequal.
    #[must_use]
    pub fn approx_eq(a: f64, b: f64, epsilon: f64) -> bool {
        if a.is_nan() || b.is_nan() {
            return false;
        }

        if a == b {
            return true;
        }

        if !a.is_finite() || !b.is_finite() {
            return false;
        }

        (a - b).abs() <= clamp_epsilon(epsilon)
    }

    /// Returns `true` when `a` and `b` are within a relative epsilon.
    ///
    /// Near zero, this falls back to an absolute comparison by using a scale
    /// factor of at least `1.0`. `NaN` never compares equal. Equal infinities
    /// compare equal.
    #[must_use]
    pub fn relative_eq(a: f64, b: f64, epsilon: f64) -> bool {
        if a.is_nan() || b.is_nan() {
            return false;
        }

        if a == b {
            return true;
        }

        if !a.is_finite() || !b.is_finite() {
            return false;
        }

        let epsilon = clamp_epsilon(epsilon);
        let scale = a.abs().max(b.abs()).max(1.0);

        (a - b).abs() <= epsilon * scale
    }
}

/// Finite-difference helpers for first-derivative approximation.
pub mod difference {
    /// Approximates the first derivative with a forward difference.
    ///
    /// This computes `(f(x + h) - f(x)) / h`. When `h == 0.0`, the result may
    /// be infinite or `NaN` according to normal floating-point behavior.
    #[must_use]
    pub fn forward_difference<F>(f: F, x: f64, h: f64) -> f64
    where
        F: Fn(f64) -> f64,
    {
        (f(x + h) - f(x)) / h
    }

    /// Approximates the first derivative with a backward difference.
    ///
    /// This computes `(f(x) - f(x - h)) / h`. When `h == 0.0`, the result may
    /// be infinite or `NaN` according to normal floating-point behavior.
    #[must_use]
    pub fn backward_difference<F>(f: F, x: f64, h: f64) -> f64
    where
        F: Fn(f64) -> f64,
    {
        (f(x) - f(x - h)) / h
    }

    /// Approximates the first derivative with a central difference.
    ///
    /// This computes `(f(x + h) - f(x - h)) / (2h)`. When `h == 0.0`, the
    /// result may be infinite or `NaN` according to normal floating-point
    /// behavior.
    #[must_use]
    pub fn central_difference<F>(f: F, x: f64, h: f64) -> f64
    where
        F: Fn(f64) -> f64,
    {
        (f(x + h) - f(x - h)) / (2.0 * h)
    }
}

/// Deterministic numerical integration rules over `f64` intervals.
pub mod integration {
    #[allow(clippy::cast_precision_loss)]
    const fn usize_to_f64(value: usize) -> f64 {
        value as f64
    }

    fn normalized_bounds(a: f64, b: f64) -> Option<(f64, f64, f64)> {
        if !a.is_finite() || !b.is_finite() {
            return None;
        }

        if a <= b {
            Some((a, b, 1.0))
        } else {
            Some((b, a, -1.0))
        }
    }

    /// Approximates an integral with the left-rectangle rule.
    ///
    /// Returns `None` when `n == 0`, when the bounds are not finite, or when a
    /// sampled function value is not finite. Reversed bounds return the
    /// negative integral.
    #[must_use]
    pub fn rectangle_rule<F>(
        integrand: F,
        lower_bound: f64,
        upper_bound: f64,
        subinterval_count: usize,
    ) -> Option<f64>
    where
        F: Fn(f64) -> f64,
    {
        if subinterval_count == 0 {
            return None;
        }

        let (start, end, sign) = normalized_bounds(lower_bound, upper_bound)?;
        let step = (end - start) / usize_to_f64(subinterval_count);
        let mut sum = 0.0;

        for sample_index in 0..subinterval_count {
            let sample_point = usize_to_f64(sample_index).mul_add(step, start);
            let value = integrand(sample_point);
            if !value.is_finite() {
                return None;
            }

            sum += value;
        }

        Some(sign * sum * step)
    }

    /// Approximates an integral with the midpoint rule.
    ///
    /// Returns `None` when `n == 0`, when the bounds are not finite, or when a
    /// sampled function value is not finite. Reversed bounds return the
    /// negative integral.
    #[must_use]
    pub fn midpoint_rule<F>(
        integrand: F,
        lower_bound: f64,
        upper_bound: f64,
        subinterval_count: usize,
    ) -> Option<f64>
    where
        F: Fn(f64) -> f64,
    {
        if subinterval_count == 0 {
            return None;
        }

        let (start, end, sign) = normalized_bounds(lower_bound, upper_bound)?;
        let step = (end - start) / usize_to_f64(subinterval_count);
        let mut sum = 0.0;

        for sample_index in 0..subinterval_count {
            let sample_point = (usize_to_f64(sample_index) + 0.5).mul_add(step, start);
            let value = integrand(sample_point);
            if !value.is_finite() {
                return None;
            }

            sum += value;
        }

        Some(sign * sum * step)
    }

    /// Approximates an integral with the trapezoidal rule.
    ///
    /// Returns `None` when `n == 0`, when the bounds are not finite, or when a
    /// sampled function value is not finite. Reversed bounds return the
    /// negative integral.
    #[must_use]
    pub fn trapezoidal_rule<F>(
        integrand: F,
        lower_bound: f64,
        upper_bound: f64,
        subinterval_count: usize,
    ) -> Option<f64>
    where
        F: Fn(f64) -> f64,
    {
        if subinterval_count == 0 {
            return None;
        }

        let (start, end, sign) = normalized_bounds(lower_bound, upper_bound)?;
        let step = (end - start) / usize_to_f64(subinterval_count);
        let start_value = integrand(start);
        let end_value = integrand(end);

        if !start_value.is_finite() || !end_value.is_finite() {
            return None;
        }

        let mut sum = 0.5 * (start_value + end_value);

        for sample_index in 1..subinterval_count {
            let sample_point = usize_to_f64(sample_index).mul_add(step, start);
            let value = integrand(sample_point);
            if !value.is_finite() {
                return None;
            }

            sum += value;
        }

        Some(sign * sum * step)
    }

    /// Approximates an integral with Simpson's rule.
    ///
    /// Returns `None` when `n == 0`, when `n` is odd, when the bounds are not
    /// finite, or when a sampled function value is not finite. Reversed bounds
    /// return the negative integral.
    #[must_use]
    pub fn simpsons_rule<F>(
        integrand: F,
        lower_bound: f64,
        upper_bound: f64,
        subinterval_count: usize,
    ) -> Option<f64>
    where
        F: Fn(f64) -> f64,
    {
        if subinterval_count == 0 || !subinterval_count.is_multiple_of(2) {
            return None;
        }

        let (start, end, sign) = normalized_bounds(lower_bound, upper_bound)?;
        let step = (end - start) / usize_to_f64(subinterval_count);
        let start_value = integrand(start);
        let end_value = integrand(end);

        if !start_value.is_finite() || !end_value.is_finite() {
            return None;
        }

        let mut sum = start_value + end_value;

        for sample_index in 1..subinterval_count {
            let sample_point = usize_to_f64(sample_index).mul_add(step, start);
            let value = integrand(sample_point);
            if !value.is_finite() {
                return None;
            }

            if sample_index.is_multiple_of(2) {
                sum += 2.0 * value;
            } else {
                sum += 4.0 * value;
            }
        }

        Some(sign * sum * step / 3.0)
    }
}

/// Iterative approximate root-finding helpers.
pub mod root {
    use crate::approx::approx_eq;

    #[cfg(feature = "interval")]
    use use_interval::{Bound, Interval};

    /// Configuration for iterative approximate root finders.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct RootOptions {
        /// Absolute tolerance used for convergence and zero checks.
        pub tolerance: f64,
        /// Maximum number of solver iterations before returning an error.
        pub max_iterations: usize,
    }

    impl Default for RootOptions {
        fn default() -> Self {
            Self {
                tolerance: 1e-10,
                max_iterations: 100,
            }
        }
    }

    /// Failure modes for approximate iterative root finders.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum RootError {
        /// The initial interval does not bracket a root.
        InvalidInterval,
        /// The configured tolerance is not finite and positive.
        InvalidTolerance,
        /// The solver did not converge within the iteration budget.
        MaxIterationsReached,
        /// A function evaluation, derivative evaluation, or iterate became non-finite.
        NonFiniteValue,
        /// A Newton-Raphson step encountered an approximately zero derivative.
        ZeroDerivative,
    }

    fn validate_options(options: RootOptions) -> Result<(), RootError> {
        if !options.tolerance.is_finite() || options.tolerance <= 0.0 {
            Err(RootError::InvalidTolerance)
        } else {
            Ok(())
        }
    }

    fn same_sign(left: f64, right: f64) -> bool {
        (left < 0.0 && right < 0.0) || (left > 0.0 && right > 0.0)
    }

    fn bisection_with_policy<F>(
        f: &F,
        lower: f64,
        upper: f64,
        options: RootOptions,
        allow_lower_endpoint: bool,
        allow_upper_endpoint: bool,
    ) -> Result<f64, RootError>
    where
        F: Fn(f64) -> f64,
    {
        validate_options(options)?;

        if !lower.is_finite() || !upper.is_finite() || lower > upper {
            return Err(RootError::InvalidInterval);
        }

        let lower_value = f(lower);
        let upper_value = f(upper);

        if !lower_value.is_finite() || !upper_value.is_finite() {
            return Err(RootError::NonFiniteValue);
        }

        if approx_eq(lower_value, 0.0, options.tolerance) {
            return if allow_lower_endpoint {
                Ok(lower)
            } else {
                Err(RootError::InvalidInterval)
            };
        }

        if approx_eq(upper_value, 0.0, options.tolerance) {
            return if allow_upper_endpoint {
                Ok(upper)
            } else {
                Err(RootError::InvalidInterval)
            };
        }

        if same_sign(lower_value, upper_value) {
            return Err(RootError::InvalidInterval);
        }

        let mut left = lower;
        let mut right = upper;
        let mut left_value = lower_value;

        for _ in 0..options.max_iterations {
            let midpoint = (right - left).mul_add(0.5, left);
            let midpoint_value = f(midpoint);

            if !midpoint.is_finite() || !midpoint_value.is_finite() {
                return Err(RootError::NonFiniteValue);
            }

            if approx_eq(midpoint_value, 0.0, options.tolerance)
                || (right - left).abs() * 0.5 <= options.tolerance
            {
                return Ok(midpoint);
            }

            if same_sign(left_value, midpoint_value) {
                left = midpoint;
                left_value = midpoint_value;
            } else {
                right = midpoint;
            }
        }

        Err(RootError::MaxIterationsReached)
    }

    /// Finds a root with the bisection method on a bracketing interval.
    ///
    /// The endpoint values must have opposite signs, unless an endpoint is
    /// already approximately zero. The interval bounds and function values must
    /// be finite. Convergence uses [`RootOptions::tolerance`] as an absolute
    /// tolerance.
    ///
    /// # Errors
    ///
    /// Returns [`RootError::InvalidInterval`] when the bounds are non-finite,
    /// reversed, or do not bracket a root; [`RootError::InvalidTolerance`]
    /// when the tolerance is not finite and positive;
    /// [`RootError::NonFiniteValue`] when a function evaluation is non-finite;
    /// and [`RootError::MaxIterationsReached`] when the iteration budget is
    /// exhausted without convergence.
    pub fn bisection<F>(
        f: F,
        lower: f64,
        upper: f64,
        options: RootOptions,
    ) -> Result<f64, RootError>
    where
        F: Fn(f64) -> f64,
    {
        bisection_with_policy(&f, lower, upper, options, true, true)
    }

    /// Finds a root with Newton-Raphson iteration.
    ///
    /// This is an approximate iterative solver, not an exact equation helper.
    /// The tolerance must be finite and positive. Non-finite values and
    /// approximately zero derivatives return explicit errors.
    ///
    /// # Errors
    ///
    /// Returns [`RootError::InvalidTolerance`] when the tolerance is not
    /// finite and positive; [`RootError::NonFiniteValue`] when the initial
    /// value, a function evaluation, a derivative evaluation, or the next
    /// iterate is non-finite; [`RootError::ZeroDerivative`] when the
    /// derivative is approximately zero; and
    /// [`RootError::MaxIterationsReached`] when the iteration budget is
    /// exhausted without convergence.
    pub fn newton_raphson<F, D>(
        f: F,
        derivative: D,
        initial: f64,
        options: RootOptions,
    ) -> Result<f64, RootError>
    where
        F: Fn(f64) -> f64,
        D: Fn(f64) -> f64,
    {
        validate_options(options)?;

        if !initial.is_finite() {
            return Err(RootError::NonFiniteValue);
        }

        let mut current = initial;

        for _ in 0..options.max_iterations {
            let value = f(current);
            if !value.is_finite() {
                return Err(RootError::NonFiniteValue);
            }

            if approx_eq(value, 0.0, options.tolerance) {
                return Ok(current);
            }

            let slope = derivative(current);
            if !slope.is_finite() {
                return Err(RootError::NonFiniteValue);
            }

            if approx_eq(slope, 0.0, options.tolerance) {
                return Err(RootError::ZeroDerivative);
            }

            let next = current - value / slope;
            if !next.is_finite() {
                return Err(RootError::NonFiniteValue);
            }

            if (next - current).abs() <= options.tolerance {
                return Ok(next);
            }

            current = next;
        }

        Err(RootError::MaxIterationsReached)
    }

    /// Finds a root with the bisection method over a bounded `use-interval` interval.
    ///
    /// Unbounded and empty intervals return [`RootError::InvalidInterval`].
    /// Open endpoints participate in bracketing, but only closed endpoints are
    /// eligible for the immediate endpoint-root fast path.
    ///
    /// # Errors
    ///
    /// Returns [`RootError::InvalidInterval`] when the interval is empty,
    /// unbounded, or does not bracket a root; [`RootError::InvalidTolerance`]
    /// when the tolerance is not finite and positive;
    /// [`RootError::NonFiniteValue`] when a function evaluation is non-finite;
    /// and [`RootError::MaxIterationsReached`] when the iteration budget is
    /// exhausted without convergence.
    #[cfg(feature = "interval")]
    pub fn bisection_interval<F>(
        f: F,
        interval: Interval<f64>,
        options: RootOptions,
    ) -> Result<f64, RootError>
    where
        F: Fn(f64) -> f64,
    {
        if interval.is_empty() {
            return Err(RootError::InvalidInterval);
        }

        let (lower, allow_lower_endpoint) = match interval.lower {
            Bound::Open(value) => (value, false),
            Bound::Closed(value) => (value, true),
            Bound::Unbounded => return Err(RootError::InvalidInterval),
        };

        let (upper, allow_upper_endpoint) = match interval.upper {
            Bound::Open(value) => (value, false),
            Bound::Closed(value) => (value, true),
            Bound::Unbounded => return Err(RootError::InvalidInterval),
        };

        bisection_with_policy(
            &f,
            lower,
            upper,
            options,
            allow_lower_endpoint,
            allow_upper_endpoint,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{
        RootError, RootOptions, approx_eq, backward_difference, bisection, central_difference,
        clamp_epsilon, forward_difference, midpoint_rule, newton_raphson, rectangle_rule,
        relative_eq, simpsons_rule, trapezoidal_rule,
    };

    #[cfg(feature = "interval")]
    use super::bisection_interval;

    #[cfg(feature = "interval")]
    use use_interval::Interval;

    #[test]
    fn absolute_approximation_equality_uses_absolute_difference() {
        assert!(approx_eq(1.0, 1.0 + 5.0e-7, 1.0e-6));
        assert!(!approx_eq(1.0, 1.0 + 2.0e-6, 1.0e-6));
    }

    #[test]
    fn relative_approximation_equality_uses_relative_difference() {
        assert!(relative_eq(10_000.0, 10_000.01, 1.0e-6));
        assert!(!relative_eq(10_000.0, 10_000.5, 1.0e-6));
    }

    #[test]
    fn zero_and_near_zero_comparisons_use_safe_fallback() {
        assert!(relative_eq(1.0e-12, 0.0, 1.0e-9));
        assert!(!relative_eq(1.0e-3, 0.0, 1.0e-6));
    }

    #[test]
    fn negative_epsilon_behavior_is_clamped_to_positive() {
        assert!((clamp_epsilon(-1.0e-6) - 1.0e-6).abs() <= f64::EPSILON);
        assert!(approx_eq(2.0, 2.0 + 5.0e-7, -1.0e-6));
    }

    #[test]
    fn non_finite_comparison_behavior_is_explicit() {
        assert!(!approx_eq(f64::NAN, f64::NAN, 1.0e-6));
        assert!(!relative_eq(f64::NAN, 1.0, 1.0e-6));
        assert!(approx_eq(f64::INFINITY, f64::INFINITY, 1.0e-6));
        assert!(!approx_eq(f64::INFINITY, f64::NEG_INFINITY, 1.0e-6));
        assert!(!relative_eq(f64::INFINITY, 1.0, 1.0e-6));
    }

    #[test]
    fn forward_difference_approximates_first_derivative() {
        let derivative = forward_difference(|x| x * x, 3.0, 1.0e-6);

        assert!((derivative - 6.0).abs() < 1.0e-5);
    }

    #[test]
    fn backward_difference_approximates_first_derivative() {
        let derivative = backward_difference(|x| x * x, 3.0, 1.0e-6);

        assert!((derivative - 6.0).abs() < 1.0e-5);
    }

    #[test]
    fn central_difference_approximates_first_derivative() {
        let derivative = central_difference(|x| x * x, 3.0, 1.0e-6);

        assert!((derivative - 6.0).abs() < 1.0e-5);
    }

    #[test]
    fn rectangle_rule_approximates_integrals() {
        let area = rectangle_rule(|x| x * x, 0.0, 1.0, 10_000).unwrap();

        assert!((area - 1.0 / 3.0).abs() < 1.0e-4);
    }

    #[test]
    fn midpoint_rule_approximates_integrals() {
        let area = midpoint_rule(|x| x * x, 0.0, 1.0, 1_000).unwrap();

        assert!((area - 1.0 / 3.0).abs() < 1.0e-6);
    }

    #[test]
    fn trapezoidal_rule_approximates_integrals() {
        let area = trapezoidal_rule(|x| x * x, 0.0, 1.0, 1_000).unwrap();

        assert!((area - 1.0 / 3.0).abs() < 1.0e-6);
    }

    #[test]
    fn simpsons_rule_approximates_integrals() {
        let area = simpsons_rule(|x| x * x, 0.0, 1.0, 10).unwrap();

        assert!((area - 1.0 / 3.0).abs() < 1.0e-12);
    }

    #[test]
    fn invalid_integration_subdivision_count_returns_none() {
        assert_eq!(rectangle_rule(|x| x, 0.0, 1.0, 0), None);
        assert_eq!(midpoint_rule(|x| x, 0.0, 1.0, 0), None);
        assert_eq!(trapezoidal_rule(|x| x, 0.0, 1.0, 0), None);
    }

    #[test]
    fn simpsons_rule_rejects_odd_subdivision_counts() {
        assert_eq!(simpsons_rule(|x| x, 0.0, 1.0, 3), None);
    }

    #[test]
    fn reversed_integration_bounds_return_negative_integral() {
        let area = trapezoidal_rule(|x| x * x, 1.0, 0.0, 1_000).unwrap();

        assert!((area + 1.0 / 3.0).abs() < 1.0e-6);
    }

    #[test]
    fn bisection_succeeds_for_bracketed_root() {
        let root = bisection(|x| x.mul_add(x, -2.0), 1.0, 2.0, RootOptions::default()).unwrap();

        assert!((root - 2.0_f64.sqrt()).abs() < 1.0e-8);
    }

    #[test]
    fn bisection_returns_endpoint_root_when_present() {
        let root = bisection(|x| x - 1.0, 1.0, 3.0, RootOptions::default()).unwrap();

        assert!((root - 1.0).abs() <= f64::EPSILON);
    }

    #[test]
    fn bisection_rejects_invalid_intervals() {
        assert_eq!(
            bisection(|x| x.mul_add(x, 1.0), -1.0, 1.0, RootOptions::default()),
            Err(RootError::InvalidInterval)
        );
    }

    #[test]
    fn bisection_rejects_invalid_tolerance() {
        assert_eq!(
            bisection(
                |x| x.mul_add(x, -2.0),
                1.0,
                2.0,
                RootOptions {
                    tolerance: 0.0,
                    max_iterations: 100,
                },
            ),
            Err(RootError::InvalidTolerance)
        );
    }

    #[test]
    fn bisection_reports_max_iterations_reached() {
        assert_eq!(
            bisection(
                |x| x.mul_add(x, -2.0),
                1.0,
                2.0,
                RootOptions {
                    tolerance: 1.0e-20,
                    max_iterations: 1,
                },
            ),
            Err(RootError::MaxIterationsReached)
        );
    }

    #[test]
    fn newton_raphson_succeeds_for_simple_root() {
        let root = newton_raphson(
            |x| x.mul_add(x, -2.0),
            |x| 2.0 * x,
            1.0,
            RootOptions::default(),
        )
        .unwrap();

        assert!((root - 2.0_f64.sqrt()).abs() < 1.0e-8);
    }

    #[test]
    fn newton_raphson_reports_zero_derivative() {
        assert_eq!(
            newton_raphson(
                |x| (x * x).mul_add(x, 1.0),
                |x| 3.0 * x * x,
                0.0,
                RootOptions::default(),
            ),
            Err(RootError::ZeroDerivative)
        );
    }

    #[test]
    fn newton_raphson_rejects_invalid_tolerance() {
        assert_eq!(
            newton_raphson(
                |x| x.mul_add(x, -2.0),
                |x| 2.0 * x,
                1.0,
                RootOptions {
                    tolerance: 0.0,
                    max_iterations: 100,
                },
            ),
            Err(RootError::InvalidTolerance)
        );
    }

    #[test]
    fn newton_raphson_reports_max_iterations_reached() {
        assert_eq!(
            newton_raphson(
                |x| x.mul_add(x, -2.0),
                |x| 2.0 * x,
                1.0,
                RootOptions {
                    tolerance: 1.0e-20,
                    max_iterations: 1,
                },
            ),
            Err(RootError::MaxIterationsReached)
        );
    }

    #[test]
    fn non_finite_root_finding_behavior_is_reported() {
        assert_eq!(
            bisection(|_| f64::NAN, 0.0, 1.0, RootOptions::default()),
            Err(RootError::NonFiniteValue)
        );
        assert_eq!(
            newton_raphson(|_| f64::NAN, |_| 1.0, 0.0, RootOptions::default(),),
            Err(RootError::NonFiniteValue)
        );
    }

    #[cfg(feature = "interval")]
    #[test]
    fn bisection_interval_supports_bounded_intervals() {
        let root = bisection_interval(
            |x| x.mul_add(x, -2.0),
            Interval::closed(1.0, 2.0),
            RootOptions::default(),
        )
        .unwrap();

        assert!((root - 2.0_f64.sqrt()).abs() < 1.0e-8);
    }

    #[cfg(feature = "interval")]
    #[test]
    fn bisection_interval_does_not_accept_open_endpoint_roots() {
        assert_eq!(
            bisection_interval(|x| x, Interval::open(0.0, 2.0), RootOptions::default()),
            Err(RootError::InvalidInterval)
        );
    }
}
