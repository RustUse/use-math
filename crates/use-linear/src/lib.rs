#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Linear-algebra utilities for `RustUse`.

use core::fmt;
use core::ops::{Add, Mul, Neg, Sub};

/// Errors returned by linear helpers when a system cannot be solved.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LinearError {
    /// The matrix is singular and does not have an inverse.
    SingularMatrix { determinant: f64 },
}

impl fmt::Display for LinearError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SingularMatrix { determinant } => {
                write!(
                    formatter,
                    "matrix is singular with determinant {determinant}"
                )
            },
        }
    }
}

impl std::error::Error for LinearError {}

/// A 2D column vector.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vector2 {
    /// The first component.
    pub x: f64,
    /// The second component.
    pub y: f64,
}

impl Vector2 {
    /// Creates a vector from two components.
    #[must_use]
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// Returns the squared Euclidean norm.
    #[must_use]
    pub const fn magnitude_squared(self) -> f64 {
        dot(self, self)
    }

    /// Returns the Euclidean norm.
    #[must_use]
    pub fn magnitude(self) -> f64 {
        self.magnitude_squared().sqrt()
    }

    /// Returns the dot product with `other`.
    #[must_use]
    pub const fn dot(self, other: Self) -> f64 {
        dot(self, other)
    }
}

impl Add for Vector2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Vector2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Neg for Vector2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y)
    }
}

impl Mul<f64> for Vector2 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

/// A 2×2 matrix stored in row-major order.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Matrix2 {
    /// Row 1, column 1.
    pub m11: f64,
    /// Row 1, column 2.
    pub m12: f64,
    /// Row 2, column 1.
    pub m21: f64,
    /// Row 2, column 2.
    pub m22: f64,
}

impl Matrix2 {
    /// Creates a 2×2 matrix from row-major entries.
    #[must_use]
    pub const fn new(m11: f64, m12: f64, m21: f64, m22: f64) -> Self {
        Self { m11, m12, m21, m22 }
    }

    /// Returns the identity matrix.
    #[must_use]
    pub const fn identity() -> Self {
        Self::new(1.0, 0.0, 0.0, 1.0)
    }

    /// Returns the transpose.
    #[must_use]
    pub const fn transpose(self) -> Self {
        Self::new(self.m11, self.m21, self.m12, self.m22)
    }

    /// Returns the determinant.
    #[must_use]
    pub const fn determinant(self) -> f64 {
        (self.m11 * self.m22) - (self.m12 * self.m21)
    }

    /// Returns the trace.
    #[must_use]
    pub const fn trace(self) -> f64 {
        self.m11 + self.m22
    }

    /// Returns the matrix-vector product.
    #[must_use]
    pub const fn mul_vector(self, vector: Vector2) -> Vector2 {
        Vector2::new(
            (self.m11 * vector.x) + (self.m12 * vector.y),
            (self.m21 * vector.x) + (self.m22 * vector.y),
        )
    }

    /// Returns the matrix-matrix product.
    #[must_use]
    pub const fn mul_matrix(self, rhs: Self) -> Self {
        let first_row = Vector2::new(self.m11, self.m12);
        let second_row = Vector2::new(self.m21, self.m22);
        let first_column = Vector2::new(rhs.m11, rhs.m21);
        let second_column = Vector2::new(rhs.m12, rhs.m22);

        Self::new(
            dot(first_row, first_column),
            dot(first_row, second_column),
            dot(second_row, first_column),
            dot(second_row, second_column),
        )
    }

    /// Solves `self * x = rhs` for `x`.
    ///
    /// # Errors
    ///
    /// Returns [`LinearError::SingularMatrix`] when the determinant is zero.
    pub fn solve(self, rhs: Vector2) -> Result<Vector2, LinearError> {
        let determinant = self.determinant();

        if determinant == 0.0 {
            return Err(LinearError::SingularMatrix { determinant });
        }

        Ok(Vector2::new(
            self.m22.mul_add(rhs.x, -(self.m12 * rhs.y)) / determinant,
            self.m11.mul_add(rhs.y, -(self.m21 * rhs.x)) / determinant,
        ))
    }
}

impl Mul<Vector2> for Matrix2 {
    type Output = Vector2;

    fn mul(self, rhs: Vector2) -> Self::Output {
        self.mul_vector(rhs)
    }
}

impl Mul for Matrix2 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self.mul_matrix(rhs)
    }
}

/// Returns the dot product of two vectors.
#[must_use]
pub const fn dot(left: Vector2, right: Vector2) -> f64 {
    (left.x * right.x) + (left.y * right.y)
}

/// Solves `matrix * x = rhs` for `x`.
///
/// # Errors
///
/// Returns [`LinearError::SingularMatrix`] when the determinant is zero.
pub fn solve_2x2(matrix: Matrix2, rhs: Vector2) -> Result<Vector2, LinearError> {
    matrix.solve(rhs)
}

pub mod prelude;

#[cfg(test)]
mod tests {
    use super::{LinearError, Matrix2, Vector2, dot, solve_2x2};

    fn assert_close(left: f64, right: f64) {
        assert!((left - right).abs() < 1.0e-12, "left={left}, right={right}");
    }

    #[test]
    fn computes_vector_and_matrix_products() {
        let left = Vector2::new(3.0, 4.0);
        let right = Vector2::new(-2.0, 1.0);
        let matrix = Matrix2::new(2.0, 1.0, 5.0, 3.0);

        assert_eq!(left + right, Vector2::new(1.0, 5.0));
        assert_eq!(left - right, Vector2::new(5.0, 3.0));
        assert_eq!(-right, Vector2::new(2.0, -1.0));
        assert_eq!(left * 2.0, Vector2::new(6.0, 8.0));
        assert_close(dot(left, right), -2.0);
        assert_close(left.dot(right), -2.0);
        assert_close(left.magnitude_squared(), 25.0);
        assert_close(left.magnitude(), 5.0);
        assert_eq!(
            matrix.mul_vector(Vector2::new(1.0, -1.0)),
            Vector2::new(1.0, 2.0)
        );
        assert_eq!(matrix * Vector2::new(1.0, -1.0), Vector2::new(1.0, 2.0));
        assert_eq!(matrix.transpose(), Matrix2::new(2.0, 5.0, 1.0, 3.0));
        assert_close(matrix.trace(), 5.0);
        assert_close(matrix.determinant(), 1.0);
        assert_eq!(matrix * Matrix2::identity(), matrix);
    }

    #[test]
    fn solves_nonsingular_systems_and_rejects_singular_ones() {
        let matrix = Matrix2::new(2.0, 1.0, 5.0, 3.0);
        let rhs = Vector2::new(1.0, 2.0);

        assert_eq!(
            matrix.solve(rhs).expect("system should solve"),
            Vector2::new(1.0, -1.0)
        );
        assert_eq!(
            solve_2x2(matrix, rhs).expect("system should solve"),
            Vector2::new(1.0, -1.0)
        );
        assert_eq!(
            Matrix2::new(1.0, 2.0, 2.0, 4.0).solve(Vector2::new(1.0, 2.0)),
            Err(LinearError::SingularMatrix { determinant: 0.0 })
        );
    }
}
