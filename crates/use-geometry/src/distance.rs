//! Distance and interpolation helpers for 2D points.

use crate::point::Point2;

/// Returns the Euclidean distance between two 2D points.
///
/// # Examples
///
/// ```
/// use use_geometry::{Point2, distance_2d};
///
/// let left = Point2::new(0.0, 0.0);
/// let right = Point2::new(3.0, 4.0);
///
/// assert_eq!(distance_2d(left, right), 5.0);
/// ```
#[must_use]
pub fn distance_2d(left: Point2, right: Point2) -> f64 {
    left.distance_to(right)
}

/// Returns the squared Euclidean distance between two 2D points.
///
/// # Examples
///
/// ```
/// use use_geometry::{Point2, distance_squared_2d};
///
/// let left = Point2::new(0.0, 0.0);
/// let right = Point2::new(3.0, 4.0);
///
/// assert_eq!(distance_squared_2d(left, right), 25.0);
/// ```
#[must_use]
pub fn distance_squared_2d(left: Point2, right: Point2) -> f64 {
    left.distance_squared_to(right)
}

/// Returns the midpoint between two 2D points.
///
/// # Examples
///
/// ```
/// use use_geometry::{Point2, midpoint_2d};
///
/// let left = Point2::new(-2.0, 1.0);
/// let right = Point2::new(4.0, 5.0);
///
/// assert_eq!(midpoint_2d(left, right), Point2::new(1.0, 3.0));
/// ```
#[must_use]
pub const fn midpoint_2d(left: Point2, right: Point2) -> Point2 {
    left.midpoint(right)
}

#[cfg(test)]
mod tests {
    use super::{distance_2d, distance_squared_2d, midpoint_2d};
    use crate::point::Point2;

    fn approx_eq(left: f64, right: f64) -> bool {
        (left - right).abs() < 1.0e-10
    }

    #[test]
    fn computes_distance() {
        let left = Point2::new(0.0, 0.0);
        let right = Point2::new(3.0, 4.0);

        assert!(approx_eq(distance_2d(left, right), 5.0));
    }

    #[test]
    fn computes_squared_distance() {
        let left = Point2::new(-1.0, 2.0);
        let right = Point2::new(2.0, 6.0);

        assert!(approx_eq(distance_squared_2d(left, right), 25.0));
    }

    #[test]
    fn computes_midpoints() {
        let left = Point2::new(0.0, 0.0);
        let right = Point2::new(4.0, 2.0);

        assert_eq!(midpoint_2d(left, right), Point2::new(2.0, 1.0));
    }
}
