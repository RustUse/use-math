#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Small polynomial primitives and operations for `RustUse`.

#[doc(inline)]
pub use polynomial::Polynomial;
#[doc(inline)]
pub use roots::{linear_root, quadratic_roots};

/// Polynomial primitives and direct operations.
pub mod polynomial {
    use core::ops::{Add, Div, Mul, Neg, Sub};

    use crate::roots::{linear_root, quadratic_roots};

    /// A polynomial whose coefficients are stored in ascending power order.
    ///
    /// `coefficients[i]` is the coefficient for `x^i`. For example,
    /// `3 + 2x + x^2` is represented as `vec![3.0, 2.0, 1.0]`.
    ///
    /// The zero polynomial is represented as an empty coefficient vector.
    #[derive(Debug, Clone, PartialEq)]
    pub struct Polynomial {
        coefficients: Vec<f64>,
    }

    impl Polynomial {
        /// Creates a polynomial from coefficients in ascending power order.
        ///
        /// Trailing zero coefficients are trimmed. The zero polynomial is
        /// stored as an empty coefficient vector.
        #[must_use]
        pub fn new(mut coefficients: Vec<f64>) -> Self {
            while coefficients
                .last()
                .is_some_and(|coefficient| *coefficient == 0.0)
            {
                coefficients.pop();
            }

            Self { coefficients }
        }

        /// Creates a constant polynomial.
        #[must_use]
        pub fn constant(value: f64) -> Self {
            Self::new(vec![value])
        }

        /// Creates the zero polynomial.
        #[must_use]
        pub fn zero() -> Self {
            Self::new(Vec::new())
        }

        /// Creates the constant polynomial `1`.
        #[must_use]
        pub fn one() -> Self {
            Self::constant(1.0)
        }

        /// Returns the coefficients in ascending power order.
        #[must_use]
        pub fn coefficients(&self) -> &[f64] {
            &self.coefficients
        }

        /// Returns the polynomial degree, or `None` for the zero polynomial.
        #[must_use]
        pub const fn degree(&self) -> Option<usize> {
            if self.coefficients.is_empty() {
                None
            } else {
                Some(self.coefficients.len() - 1)
            }
        }

        /// Returns the leading coefficient, or `None` for the zero polynomial.
        #[must_use]
        pub fn leading_coefficient(&self) -> Option<f64> {
            self.coefficients.last().copied()
        }

        /// Returns `true` when the polynomial is the zero polynomial.
        #[must_use]
        pub const fn is_zero(&self) -> bool {
            self.coefficients.is_empty()
        }

        /// Evaluates the polynomial at `x` using Horner's method.
        #[must_use]
        pub fn evaluate(&self, x: f64) -> f64 {
            self.coefficients
                .iter()
                .rev()
                .fold(0.0, |accumulator, coefficient| {
                    accumulator * x + coefficient
                })
        }

        /// Returns the derivative of the polynomial.
        #[must_use]
        #[allow(clippy::cast_precision_loss)]
        pub fn derivative(&self) -> Self {
            if self.coefficients.len() < 2 {
                return Self::zero();
            }

            let coefficients = self
                .coefficients
                .iter()
                .enumerate()
                .skip(1)
                .map(|(index, coefficient)| *coefficient * index as f64)
                .collect();

            Self::new(coefficients)
        }

        /// Returns the `n`th derivative of the polynomial.
        #[must_use]
        pub fn nth_derivative(&self, n: usize) -> Self {
            if n == 0 {
                return self.clone();
            }

            let mut derivative = self.clone();
            for _ in 0..n {
                derivative = derivative.derivative();
                if derivative.is_zero() {
                    break;
                }
            }

            derivative
        }

        /// Returns the indefinite integral with the provided constant term.
        #[must_use]
        #[allow(clippy::cast_precision_loss)]
        pub fn integral(&self, constant: f64) -> Self {
            let mut coefficients = Vec::with_capacity(self.coefficients.len() + 1);
            coefficients.push(constant);
            coefficients.extend(
                self.coefficients
                    .iter()
                    .enumerate()
                    .map(|(index, coefficient)| *coefficient / (index + 1) as f64),
            );

            Self::new(coefficients)
        }

        /// Returns real roots for degree 0, 1, or 2 polynomials.
        ///
        /// Higher-degree polynomials return `None`. Constant and zero
        /// polynomials return `Some(vec![])`.
        #[must_use]
        pub fn real_roots_degree_1_or_2(&self) -> Option<Vec<f64>> {
            match self.degree() {
                None | Some(0) => Some(Vec::new()),
                Some(1) => Some(
                    linear_root(self.coefficients[1], self.coefficients[0])
                        .into_iter()
                        .collect(),
                ),
                Some(2) => Some(quadratic_roots(
                    self.coefficients[2],
                    self.coefficients[1],
                    self.coefficients[0],
                )),
                Some(_) => None,
            }
        }
    }

    impl Add for Polynomial {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            let max_len = self.coefficients.len().max(rhs.coefficients.len());
            let coefficients = (0..max_len)
                .map(|index| {
                    self.coefficients.get(index).copied().unwrap_or(0.0)
                        + rhs.coefficients.get(index).copied().unwrap_or(0.0)
                })
                .collect();

            Self::new(coefficients)
        }
    }

    impl Sub for Polynomial {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self::Output {
            let max_len = self.coefficients.len().max(rhs.coefficients.len());
            let coefficients = (0..max_len)
                .map(|index| {
                    self.coefficients.get(index).copied().unwrap_or(0.0)
                        - rhs.coefficients.get(index).copied().unwrap_or(0.0)
                })
                .collect();

            Self::new(coefficients)
        }
    }

    impl Mul for Polynomial {
        type Output = Self;

        fn mul(self, rhs: Self) -> Self::Output {
            if self.is_zero() || rhs.is_zero() {
                return Self::zero();
            }

            let mut coefficients = vec![0.0; self.coefficients.len() + rhs.coefficients.len() - 1];

            for (left_index, left_coefficient) in self.coefficients.iter().enumerate() {
                for (right_index, right_coefficient) in rhs.coefficients.iter().enumerate() {
                    coefficients[left_index + right_index] += left_coefficient * right_coefficient;
                }
            }

            Self::new(coefficients)
        }
    }

    impl Neg for Polynomial {
        type Output = Self;

        fn neg(self) -> Self::Output {
            Self::new(
                self.coefficients
                    .into_iter()
                    .map(|coefficient| -coefficient)
                    .collect(),
            )
        }
    }

    impl Mul<f64> for Polynomial {
        type Output = Self;

        fn mul(self, rhs: f64) -> Self::Output {
            Self::new(
                self.coefficients
                    .into_iter()
                    .map(|coefficient| coefficient * rhs)
                    .collect(),
            )
        }
    }

    impl Div<f64> for Polynomial {
        type Output = Self;

        fn div(self, rhs: f64) -> Self::Output {
            Self::new(
                self.coefficients
                    .into_iter()
                    .map(|coefficient| coefficient / rhs)
                    .collect(),
            )
        }
    }
}

/// Small real-root helpers for low-degree polynomials.
pub mod roots {
    /// Returns the real root of `ax + b = 0`.
    ///
    /// Returns `None` when `a == 0.0`.
    #[must_use]
    pub fn linear_root(a: f64, b: f64) -> Option<f64> {
        (a != 0.0).then_some(-b / a)
    }

    /// Returns the real roots of `ax^2 + bx + c = 0`.
    ///
    /// When `a == 0.0`, this falls back to the linear case. Only real roots are
    /// returned. Distinct real roots are returned in ascending order.
    #[must_use]
    pub fn quadratic_roots(a: f64, b: f64, c: f64) -> Vec<f64> {
        if a == 0.0 {
            return linear_root(b, c).into_iter().collect();
        }

        let discriminant = b.mul_add(b, -(4.0 * a * c));

        if discriminant.is_nan() || discriminant < 0.0 {
            return Vec::new();
        }

        if discriminant == 0.0 {
            return vec![-b / (2.0 * a)];
        }

        let square_root = discriminant.sqrt();
        let first = (-b - square_root) / (2.0 * a);
        let second = (-b + square_root) / (2.0 * a);

        if first <= second {
            vec![first, second]
        } else {
            vec![second, first]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Polynomial, linear_root, quadratic_roots};

    #[test]
    fn constructor_trims_trailing_zeros() {
        let polynomial = Polynomial::new(vec![1.0, 2.0, 0.0, 0.0]);

        assert_eq!(polynomial.coefficients(), &[1.0, 2.0]);
    }

    #[test]
    fn zero_polynomial_uses_empty_representation() {
        let zero = Polynomial::new(vec![0.0, 0.0]);

        assert!(zero.coefficients().is_empty());
        assert_eq!(zero.degree(), None);
        assert_eq!(zero.leading_coefficient(), None);
        assert!(zero.is_zero());
    }

    #[test]
    fn constant_constructors_work() {
        assert_eq!(Polynomial::constant(4.0).coefficients(), &[4.0]);
        assert!(Polynomial::zero().is_zero());
        assert_eq!(Polynomial::one().coefficients(), &[1.0]);
    }

    #[test]
    fn reports_degree_and_leading_coefficient() {
        let polynomial = Polynomial::new(vec![3.0, 2.0, 1.0]);

        assert_eq!(polynomial.degree(), Some(2));
        assert_eq!(polynomial.leading_coefficient(), Some(1.0));
    }

    #[test]
    fn evaluates_with_horner_method() {
        let polynomial = Polynomial::new(vec![3.0, 2.0, 1.0]);

        assert!((polynomial.evaluate(2.0) - 11.0).abs() <= f64::EPSILON);
        assert!(Polynomial::zero().evaluate(3.0).abs() <= f64::EPSILON);
    }

    #[test]
    fn derivative_handles_zero_constant_linear_and_quadratic() {
        assert_eq!(Polynomial::zero().derivative(), Polynomial::zero());
        assert_eq!(Polynomial::constant(5.0).derivative(), Polynomial::zero());
        assert_eq!(
            Polynomial::new(vec![3.0, 2.0]).derivative(),
            Polynomial::constant(2.0)
        );
        assert_eq!(
            Polynomial::new(vec![3.0, 2.0, 1.0]).derivative(),
            Polynomial::new(vec![2.0, 2.0])
        );
    }

    #[test]
    fn nth_derivative_behaves_as_expected() {
        let polynomial = Polynomial::new(vec![1.0, 2.0, 3.0, 4.0]);

        assert_eq!(
            polynomial.nth_derivative(0),
            Polynomial::new(vec![1.0, 2.0, 3.0, 4.0])
        );
        assert_eq!(
            polynomial.nth_derivative(1),
            Polynomial::new(vec![2.0, 6.0, 12.0])
        );
        assert_eq!(
            polynomial.nth_derivative(2),
            Polynomial::new(vec![6.0, 24.0])
        );
        assert_eq!(polynomial.nth_derivative(4), Polynomial::zero());
    }

    #[test]
    fn computes_integral_with_constant() {
        let polynomial = Polynomial::new(vec![2.0, 6.0]);

        assert_eq!(
            polynomial.integral(5.0),
            Polynomial::new(vec![5.0, 2.0, 3.0])
        );
    }

    #[test]
    fn adds_polynomials() {
        let left = Polynomial::new(vec![1.0, 2.0, 3.0]);
        let right = Polynomial::new(vec![4.0, -2.0]);

        assert_eq!(left + right, Polynomial::new(vec![5.0, 0.0, 3.0]));
    }

    #[test]
    fn subtracts_polynomials() {
        let left = Polynomial::new(vec![5.0, 1.0]);
        let right = Polynomial::new(vec![2.0, 3.0, 4.0]);

        assert_eq!(left - right, Polynomial::new(vec![3.0, -2.0, -4.0]));
    }

    #[test]
    fn multiplies_polynomials() {
        let left = Polynomial::new(vec![1.0, 1.0]);
        let right = Polynomial::new(vec![1.0, -1.0]);

        assert_eq!(left * right, Polynomial::new(vec![1.0, 0.0, -1.0]));
    }

    #[test]
    fn negates_polynomials() {
        let polynomial = Polynomial::new(vec![1.0, -2.0, 3.0]);

        assert_eq!(-polynomial, Polynomial::new(vec![-1.0, 2.0, -3.0]));
    }

    #[test]
    fn multiplies_by_scalar() {
        let polynomial = Polynomial::new(vec![1.0, -2.0, 3.0]);

        assert_eq!(polynomial * 2.0, Polynomial::new(vec![2.0, -4.0, 6.0]));
        assert_eq!(Polynomial::new(vec![1.0, 2.0]) * 0.0, Polynomial::zero());
    }

    #[test]
    fn divides_by_scalar() {
        let polynomial = Polynomial::new(vec![2.0, -4.0, 6.0]);

        assert_eq!(polynomial / 2.0, Polynomial::new(vec![1.0, -2.0, 3.0]));
    }

    #[test]
    fn solves_linear_roots() {
        assert_eq!(linear_root(2.0, -4.0), Some(2.0));
        assert_eq!(linear_root(0.0, 3.0), None);
    }

    #[test]
    fn solves_quadratic_roots_with_two_real_roots() {
        assert_eq!(quadratic_roots(1.0, -3.0, 2.0), vec![1.0, 2.0]);
    }

    #[test]
    fn solves_quadratic_roots_with_one_repeated_root() {
        assert_eq!(quadratic_roots(1.0, -2.0, 1.0), vec![1.0]);
    }

    #[test]
    fn solves_quadratic_roots_with_no_real_roots() {
        assert!(quadratic_roots(1.0, 0.0, 1.0).is_empty());
    }

    #[test]
    fn quadratic_roots_fall_back_to_linear_case() {
        assert_eq!(quadratic_roots(0.0, 2.0, -4.0), vec![2.0]);
        assert!(quadratic_roots(0.0, 0.0, 1.0).is_empty());
    }

    #[test]
    fn low_degree_real_root_helper_handles_supported_cases() {
        assert_eq!(Polynomial::zero().real_roots_degree_1_or_2(), Some(vec![]));
        assert_eq!(
            Polynomial::constant(5.0).real_roots_degree_1_or_2(),
            Some(vec![])
        );
        assert_eq!(
            Polynomial::new(vec![-4.0, 2.0]).real_roots_degree_1_or_2(),
            Some(vec![2.0])
        );
        assert_eq!(
            Polynomial::new(vec![2.0, -3.0, 1.0]).real_roots_degree_1_or_2(),
            Some(vec![1.0, 2.0])
        );
        assert_eq!(
            Polynomial::new(vec![1.0, 0.0, 0.0, 1.0]).real_roots_degree_1_or_2(),
            None
        );
    }
}
