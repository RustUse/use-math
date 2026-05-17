#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Small equation-solving primitives for `RustUse`.

#[doc(inline)]
pub use linear::{LinearEquation, solve_linear};
#[cfg(feature = "polynomial")]
#[doc(inline)]
pub use polynomial::solve_polynomial_degree_1_or_2;
#[doc(inline)]
pub use quadratic::{QuadraticEquation, solve_quadratic};
#[doc(inline)]
pub use root::{RootSolver, Roots};
#[doc(inline)]
pub use system::{LinearSystem2, solve_2x2};

/// Root result values and shared solving traits.
pub mod root {
    /// Real-root outcomes for the small equation helpers in this crate.
    #[derive(Debug, Clone, PartialEq)]
    pub enum Roots {
        /// The equation has no finite real roots.
        None,
        /// The equation has exactly one finite real root.
        One(f64),
        /// The equation has exactly two distinct finite real roots.
        Two(f64, f64),
        /// The equation is satisfied by every real value.
        Infinite,
    }

    /// Shared trait for equation types that can solve themselves.
    pub trait RootSolver {
        /// The solved output type for the equation.
        type Output;

        /// Solves the equation and returns its concrete output.
        fn solve(&self) -> Self::Output;
    }
}

/// Linear equation helpers for equations of the form `ax + b = 0`.
pub mod linear {
    use crate::root::{RootSolver, Roots};

    /// A linear equation of the form `ax + b = 0`.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct LinearEquation {
        /// The coefficient for `x`.
        pub a: f64,
        /// The constant term.
        pub b: f64,
    }

    impl LinearEquation {
        /// Creates a linear equation of the form `ax + b = 0`.
        #[must_use]
        pub const fn new(a: f64, b: f64) -> Self {
            Self { a, b }
        }
    }

    impl RootSolver for LinearEquation {
        type Output = Roots;

        fn solve(&self) -> Self::Output {
            solve_linear(self.a, self.b)
        }
    }

    /// Solves the linear equation `ax + b = 0`.
    ///
    /// Returns [`Roots::Infinite`] when both coefficients are zero, and
    /// [`Roots::None`] when the equation has no finite real root.
    #[must_use]
    pub fn solve_linear(a: f64, b: f64) -> Roots {
        if a == 0.0 {
            if b == 0.0 {
                Roots::Infinite
            } else {
                Roots::None
            }
        } else {
            let root = -b / a;

            if root.is_finite() {
                Roots::One(root)
            } else {
                Roots::None
            }
        }
    }
}

/// Quadratic equation helpers for equations of the form `ax^2 + bx + c = 0`.
pub mod quadratic {
    use crate::{
        linear::solve_linear,
        root::{RootSolver, Roots},
    };

    /// A quadratic equation of the form `ax^2 + bx + c = 0`.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct QuadraticEquation {
        /// The coefficient for `x^2`.
        pub a: f64,
        /// The coefficient for `x`.
        pub b: f64,
        /// The constant term.
        pub c: f64,
    }

    impl QuadraticEquation {
        /// Creates a quadratic equation of the form `ax^2 + bx + c = 0`.
        #[must_use]
        pub const fn new(a: f64, b: f64, c: f64) -> Self {
            Self { a, b, c }
        }
    }

    impl RootSolver for QuadraticEquation {
        type Output = Roots;

        fn solve(&self) -> Self::Output {
            solve_quadratic(self.a, self.b, self.c)
        }
    }

    /// Solves the quadratic equation `ax^2 + bx + c = 0` over the real numbers.
    ///
    /// When `a == 0.0`, this falls back to solving `bx + c = 0` with
    /// [`solve_linear`]. Roots are returned in ascending order when two
    /// distinct real roots exist.
    #[must_use]
    pub fn solve_quadratic(a: f64, b: f64, c: f64) -> Roots {
        if a == 0.0 {
            return solve_linear(b, c);
        }

        let discriminant = b.mul_add(b, -4.0 * a * c);
        if !discriminant.is_finite() {
            return Roots::None;
        }

        if discriminant < 0.0 {
            return Roots::None;
        }

        if discriminant == 0.0 {
            let root = -b / (2.0 * a);
            return if root.is_finite() {
                Roots::One(root)
            } else {
                Roots::None
            };
        }

        let sqrt_discriminant = discriminant.sqrt();
        let first_root = (-b - sqrt_discriminant) / (2.0 * a);
        let second_root = (-b + sqrt_discriminant) / (2.0 * a);

        if !first_root.is_finite() || !second_root.is_finite() {
            return Roots::None;
        }

        if first_root <= second_root {
            Roots::Two(first_root, second_root)
        } else {
            Roots::Two(second_root, first_root)
        }
    }
}

/// Small `2x2` linear-system helpers.
pub mod system {
    use crate::root::RootSolver;

    /// A `2x2` linear system of the form:
    ///
    /// `a11*x + a12*y = b1`
    ///
    /// `a21*x + a22*y = b2`
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct LinearSystem2 {
        /// Row 1, column 1 coefficient.
        pub a11: f64,
        /// Row 1, column 2 coefficient.
        pub a12: f64,
        /// Row 1 right-hand side.
        pub b1: f64,
        /// Row 2, column 1 coefficient.
        pub a21: f64,
        /// Row 2, column 2 coefficient.
        pub a22: f64,
        /// Row 2 right-hand side.
        pub b2: f64,
    }

    impl LinearSystem2 {
        /// Creates a `2x2` linear system.
        #[must_use]
        pub const fn new(a11: f64, a12: f64, b1: f64, a21: f64, a22: f64, b2: f64) -> Self {
            Self {
                a11,
                a12,
                b1,
                a21,
                a22,
                b2,
            }
        }
    }

    impl RootSolver for LinearSystem2 {
        type Output = Option<(f64, f64)>;

        fn solve(&self) -> Self::Output {
            solve_2x2(self.a11, self.a12, self.b1, self.a21, self.a22, self.b2)
        }
    }

    /// Solves a `2x2` linear system with Cramer's rule.
    ///
    /// Returns `None` when the determinant is zero, non-finite, or when the
    /// computed solution is not finite.
    #[must_use]
    pub fn solve_2x2(
        a11: f64,
        a12: f64,
        b1: f64,
        a21: f64,
        a22: f64,
        b2: f64,
    ) -> Option<(f64, f64)> {
        let determinant = a11.mul_add(a22, -(a12 * a21));
        if determinant == 0.0 || !determinant.is_finite() {
            return None;
        }

        let x = b1.mul_add(a22, -(a12 * b2)) / determinant;
        let y = a11.mul_add(b2, -(b1 * a21)) / determinant;

        if x.is_finite() && y.is_finite() {
            Some((x, y))
        } else {
            None
        }
    }
}

#[cfg(feature = "polynomial")]
/// Low-degree polynomial bridge helpers.
pub mod polynomial {
    use crate::{linear::solve_linear, quadratic::solve_quadratic, root::Roots};
    use use_polynomial::Polynomial;

    /// Solves a polynomial when its degree is 0, 1, or 2.
    ///
    /// Returns `Some(Roots)` for low-degree polynomials and `None` for higher
    /// degrees. The zero polynomial returns [`Roots::Infinite`].
    #[must_use]
    pub fn solve_polynomial_degree_1_or_2(polynomial: &Polynomial) -> Option<Roots> {
        match polynomial.degree() {
            None => Some(Roots::Infinite),
            Some(0) => {
                let constant = polynomial.coefficients().first().copied().unwrap_or(0.0);
                Some(if constant == 0.0 {
                    Roots::Infinite
                } else {
                    Roots::None
                })
            },
            Some(1) => Some(solve_linear(
                polynomial.coefficients()[1],
                polynomial.coefficients()[0],
            )),
            Some(2) => Some(solve_quadratic(
                polynomial.coefficients()[2],
                polynomial.coefficients()[1],
                polynomial.coefficients()[0],
            )),
            Some(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        LinearEquation, LinearSystem2, QuadraticEquation, RootSolver, Roots, solve_2x2,
        solve_linear, solve_quadratic,
    };

    #[test]
    fn solves_linear_equation_with_one_root() {
        assert_eq!(solve_linear(2.0, -4.0), Roots::One(2.0));
    }

    #[test]
    fn solves_linear_equation_with_no_roots() {
        assert_eq!(solve_linear(0.0, 5.0), Roots::None);
    }

    #[test]
    fn solves_linear_equation_with_infinite_roots() {
        assert_eq!(solve_linear(0.0, 0.0), Roots::Infinite);
    }

    #[test]
    fn solves_quadratic_equation_with_two_real_roots() {
        assert_eq!(solve_quadratic(1.0, -3.0, 2.0), Roots::Two(1.0, 2.0));
    }

    #[test]
    fn solves_quadratic_equation_with_one_repeated_root() {
        assert_eq!(solve_quadratic(1.0, -2.0, 1.0), Roots::One(1.0));
    }

    #[test]
    fn solves_quadratic_equation_with_no_real_roots() {
        assert_eq!(solve_quadratic(1.0, 0.0, 1.0), Roots::None);
    }

    #[test]
    fn quadratic_falls_back_to_linear_solving() {
        assert_eq!(solve_quadratic(0.0, 2.0, -4.0), Roots::One(2.0));
    }

    #[test]
    fn solves_2x2_system_with_one_solution() {
        assert_eq!(solve_2x2(2.0, 1.0, 5.0, 1.0, -1.0, 1.0), Some((2.0, 1.0)));
    }

    #[test]
    fn rejects_singular_2x2_system() {
        assert_eq!(solve_2x2(1.0, 2.0, 3.0, 2.0, 4.0, 6.0), None);
    }

    #[test]
    fn rejects_non_finite_determinant() {
        assert_eq!(solve_2x2(f64::INFINITY, 0.0, 1.0, 0.0, 1.0, 2.0), None);
    }

    #[test]
    fn linear_equation_struct_solves() {
        let equation = LinearEquation::new(2.0, -4.0);

        assert_eq!(equation.solve(), Roots::One(2.0));
    }

    #[test]
    fn quadratic_equation_struct_solves() {
        let equation = QuadraticEquation::new(1.0, -3.0, 2.0);

        assert_eq!(equation.solve(), Roots::Two(1.0, 2.0));
    }

    #[test]
    fn linear_system_struct_solves() {
        let system = LinearSystem2::new(2.0, 1.0, 5.0, 1.0, -1.0, 1.0);

        assert_eq!(system.solve(), Some((2.0, 1.0)));
    }

    #[cfg(feature = "polynomial")]
    #[test]
    fn solves_supported_polynomials() {
        use super::solve_polynomial_degree_1_or_2;
        use use_polynomial::Polynomial;

        assert_eq!(
            solve_polynomial_degree_1_or_2(&Polynomial::new(vec![2.0, -3.0, 1.0])),
            Some(Roots::Two(1.0, 2.0))
        );
        assert_eq!(
            solve_polynomial_degree_1_or_2(&Polynomial::zero()),
            Some(Roots::Infinite)
        );
        assert_eq!(
            solve_polynomial_degree_1_or_2(&Polynomial::new(vec![1.0, 0.0, 0.0, 1.0])),
            None
        );
    }
}
