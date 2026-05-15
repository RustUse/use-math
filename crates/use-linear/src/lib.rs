#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Linear-algebra utilities for `RustUse`.

use core::fmt;

use use_matrix::Matrix2;
use use_vector::Vector2;

/// Errors returned by linear helpers when a system cannot be solved.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LinearError {
    /// The matrix is singular or non-finite and does not have a usable inverse.
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

/// Solves `matrix * x = rhs` for `x`.
///
/// # Errors
///
/// Returns [`LinearError::SingularMatrix`] when the determinant is zero or
/// non-finite.
///
/// # Examples
///
/// ```
/// use use_linear::solve_2x2;
/// use use_matrix::Matrix2;
/// use use_vector::Vector2;
///
/// let matrix = Matrix2::new(2.0, 1.0, 5.0, 3.0);
/// let rhs = Vector2::new(1.0, 2.0);
///
/// assert_eq!(solve_2x2(matrix, rhs)?, Vector2::new(1.0, -1.0));
/// # Ok::<(), use_linear::LinearError>(())
/// ```
pub fn solve_2x2(matrix: Matrix2, rhs: Vector2) -> Result<Vector2, LinearError> {
    let determinant = matrix.determinant();

    matrix
        .inverse()
        .map(|inverse| inverse * rhs)
        .ok_or(LinearError::SingularMatrix { determinant })
}

pub mod prelude;

#[cfg(test)]
mod tests {
    use super::{LinearError, solve_2x2};
    use use_matrix::Matrix2;
    use use_vector::Vector2;

    #[test]
    fn solves_nonsingular_systems_and_rejects_unusable_ones() {
        let matrix = Matrix2::new(2.0, 1.0, 5.0, 3.0);
        let rhs = Vector2::new(1.0, 2.0);
        let solution = solve_2x2(matrix, rhs).expect("system should solve");

        assert_eq!(solution, Vector2::new(1.0, -1.0));
        assert_eq!(matrix * solution, rhs);
        assert_eq!(
            solve_2x2(Matrix2::new(1.0, 2.0, 2.0, 4.0), Vector2::new(1.0, 2.0)),
            Err(LinearError::SingularMatrix { determinant: 0.0 })
        );
        assert!(matches!(
            solve_2x2(
                Matrix2::new(f64::INFINITY, 0.0, 0.0, 1.0),
                Vector2::new(1.0, 2.0)
            ),
            Err(LinearError::SingularMatrix { determinant }) if !determinant.is_finite()
        ));
    }
}
