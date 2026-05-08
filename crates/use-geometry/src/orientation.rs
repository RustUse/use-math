use crate::{error::GeometryError, point::Point2, vector::Vector2};

/// The winding order of three 2D points.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Orientation2 {
    /// Points wind in clockwise order.
    Clockwise,
    /// Points wind in counterclockwise order.
    CounterClockwise,
    /// Points are collinear.
    Collinear,
}

/// Returns twice the signed area of the triangle formed by three 2D points.
#[must_use]
pub fn signed_twice_area_2d(a: Point2, b: Point2, c: Point2) -> f64 {
    Vector2::from_points(a, b).cross(Vector2::from_points(a, c))
}

/// Returns the winding order of three 2D points.
#[must_use]
pub fn orientation_2d(a: Point2, b: Point2, c: Point2) -> Orientation2 {
    let signed_area = signed_twice_area_2d(a, b, c);

    if signed_area > 0.0 {
        Orientation2::CounterClockwise
    } else if signed_area < 0.0 {
        Orientation2::Clockwise
    } else {
        Orientation2::Collinear
    }
}

/// Returns the winding order of three 2D points using an explicit area tolerance.
///
/// # Errors
///
/// Returns [`GeometryError::NonFiniteTolerance`] when `tolerance` is `NaN` or
/// infinite.
///
/// Returns [`GeometryError::NegativeTolerance`] when `tolerance` is negative.
pub fn orientation_2d_with_tolerance(
    a: Point2,
    b: Point2,
    c: Point2,
    tolerance: f64,
) -> Result<Orientation2, GeometryError> {
    let tolerance = GeometryError::validate_tolerance(tolerance)?;
    let signed_area = signed_twice_area_2d(a, b, c);

    Ok(if signed_area > tolerance {
        Orientation2::CounterClockwise
    } else if signed_area < -tolerance {
        Orientation2::Clockwise
    } else {
        Orientation2::Collinear
    })
}

/// Returns the winding order of three 2D points with finite coordinates.
///
/// # Errors
///
/// Returns [`GeometryError::NonFiniteComponent`] when any input point contains
/// a non-finite coordinate.
pub fn try_orientation_2d(a: Point2, b: Point2, c: Point2) -> Result<Orientation2, GeometryError> {
    let a = a.validate()?;
    let b = b.validate()?;
    let c = c.validate()?;

    Ok(orientation_2d(a, b, c))
}

/// Returns the winding order of three finite 2D points using an explicit area tolerance.
///
/// # Errors
///
/// Returns [`GeometryError::NonFiniteComponent`] when any input point contains
/// a non-finite coordinate.
///
/// Returns [`GeometryError::NonFiniteTolerance`] when `tolerance` is `NaN` or
/// infinite.
///
/// Returns [`GeometryError::NegativeTolerance`] when `tolerance` is negative.
pub fn try_orientation_2d_with_tolerance(
    a: Point2,
    b: Point2,
    c: Point2,
    tolerance: f64,
) -> Result<Orientation2, GeometryError> {
    let a = a.validate()?;
    let b = b.validate()?;
    let c = c.validate()?;

    orientation_2d_with_tolerance(a, b, c, tolerance)
}

#[cfg(test)]
mod tests {
    use super::{
        Orientation2, orientation_2d, orientation_2d_with_tolerance, signed_twice_area_2d,
        try_orientation_2d, try_orientation_2d_with_tolerance,
    };
    use crate::{error::GeometryError, point::Point2};

    fn approx_eq(left: f64, right: f64) -> bool {
        (left - right).abs() < 1.0e-10
    }

    #[test]
    fn computes_counterclockwise_orientation() {
        assert_eq!(
            orientation_2d(
                Point2::new(0.0, 0.0),
                Point2::new(4.0, 0.0),
                Point2::new(0.0, 3.0)
            ),
            Orientation2::CounterClockwise
        );
    }

    #[test]
    fn computes_clockwise_orientation() {
        assert_eq!(
            orientation_2d(
                Point2::new(0.0, 0.0),
                Point2::new(0.0, 3.0),
                Point2::new(4.0, 0.0)
            ),
            Orientation2::Clockwise
        );
    }

    #[test]
    fn computes_collinear_orientation() {
        assert_eq!(
            orientation_2d(
                Point2::new(0.0, 0.0),
                Point2::new(1.0, 1.0),
                Point2::new(2.0, 2.0)
            ),
            Orientation2::Collinear
        );
    }

    #[test]
    fn computes_signed_twice_area() {
        assert!(approx_eq(
            signed_twice_area_2d(
                Point2::new(0.0, 0.0),
                Point2::new(4.0, 0.0),
                Point2::new(0.0, 3.0)
            ),
            12.0
        ));
    }

    #[test]
    fn computes_try_orientation_for_finite_points() {
        assert_eq!(
            try_orientation_2d(
                Point2::new(0.0, 0.0),
                Point2::new(4.0, 0.0),
                Point2::new(0.0, 3.0)
            ),
            Ok(Orientation2::CounterClockwise)
        );
    }

    #[test]
    fn rejects_try_orientation_for_non_finite_points() {
        assert_eq!(
            try_orientation_2d(
                Point2::new(0.0, 0.0),
                Point2::new(4.0, 0.0),
                Point2::new(0.0, f64::INFINITY)
            ),
            Err(GeometryError::NonFiniteComponent {
                type_name: "Point2",
                component: "y",
                value: f64::INFINITY,
            })
        );
    }

    #[test]
    fn computes_tolerance_based_orientation() {
        assert_eq!(
            orientation_2d_with_tolerance(
                Point2::new(0.0, 0.0),
                Point2::new(1.0, 1.0),
                Point2::new(2.0, 2.0 + 1.0e-12),
                1.0e-11
            ),
            Ok(Orientation2::Collinear)
        );
        assert_eq!(
            try_orientation_2d_with_tolerance(
                Point2::new(0.0, 0.0),
                Point2::new(4.0, 0.0),
                Point2::new(0.0, 3.0),
                0.0
            ),
            Ok(Orientation2::CounterClockwise)
        );
    }

    #[test]
    fn rejects_invalid_orientation_tolerance() {
        assert_eq!(
            orientation_2d_with_tolerance(
                Point2::new(0.0, 0.0),
                Point2::new(1.0, 0.0),
                Point2::new(0.0, 1.0),
                -1.0
            ),
            Err(GeometryError::NegativeTolerance(-1.0))
        );
    }
}
