use crate::{
    error::GeometryError, orientation::signed_twice_area_2d, point::Point2, vector::Vector2,
};

/// An infinite 2D line represented by two sample points.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Line2 {
    /// The first point on the line.
    a: Point2,
    /// The second point on the line.
    b: Point2,
}

impl Line2 {
    /// Creates a line from two sample points.
    #[must_use]
    pub const fn new(a: Point2, b: Point2) -> Self {
        Self { a, b }
    }

    /// Creates a validated line from two distinct sample points with finite coordinates.
    ///
    /// # Errors
    ///
    /// Returns [`GeometryError::NonFiniteComponent`] when either input point
    /// contains a non-finite coordinate.
    ///
    /// Returns [`GeometryError::IdenticalPoints`] when `a == b`.
    pub fn try_new(a: Point2, b: Point2) -> Result<Self, GeometryError> {
        Self::try_from_points(a, b)
    }

    /// Creates a validated line from two distinct finite sample points.
    ///
    /// # Errors
    ///
    /// Returns [`GeometryError::NonFiniteComponent`] when either input point
    /// contains a non-finite coordinate.
    ///
    /// Returns [`GeometryError::IdenticalPoints`] when `a == b`.
    ///
    /// # Examples
    ///
    /// ```
    /// use use_geometry::{GeometryError, Line2, Point2};
    ///
    /// let line = Line2::try_from_points(Point2::new(0.0, 0.0), Point2::new(2.0, 2.0))?;
    /// assert!(line.contains_point(Point2::new(4.0, 4.0)));
    ///
    /// assert!(matches!(
    ///     Line2::try_from_points(Point2::new(1.0, 1.0), Point2::new(1.0, 1.0)),
    ///     Err(GeometryError::IdenticalPoints)
    /// ));
    /// # Ok::<(), GeometryError>(())
    /// ```
    pub fn try_from_points(a: Point2, b: Point2) -> Result<Self, GeometryError> {
        let a = a.validate()?;
        let b = b.validate()?;

        if a == b {
            return Err(GeometryError::IdenticalPoints);
        }

        Ok(Self::new(a, b))
    }

    /// Creates a validated line from a point and non-zero direction vector.
    ///
    /// # Errors
    ///
    /// Returns [`GeometryError::NonFiniteComponent`] when `point` or `direction`
    /// contains a non-finite value.
    ///
    /// Returns [`GeometryError::ZeroDirectionVector`] when `direction` is zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use use_geometry::{GeometryError, Line2, Point2, Vector2};
    ///
    /// let line = Line2::try_from_point_direction(Point2::new(1.0, 2.0), Vector2::new(3.0, 4.0))?;
    /// assert_eq!(line.b(), Point2::new(4.0, 6.0));
    /// # Ok::<(), GeometryError>(())
    /// ```
    pub fn try_from_point_direction(
        point: Point2,
        direction: Vector2,
    ) -> Result<Self, GeometryError> {
        let point = point.validate()?;
        let direction = direction.validate()?;

        if direction.length_squared() == 0.0 {
            return Err(GeometryError::ZeroDirectionVector);
        }

        Ok(Self::new(point, point + direction))
    }

    /// Returns the first sample point on the line.
    #[must_use]
    pub const fn a(self) -> Point2 {
        self.a
    }

    /// Returns the second sample point on the line.
    #[must_use]
    pub const fn b(self) -> Point2 {
        self.b
    }

    /// Returns one sample point on the line.
    #[must_use]
    pub const fn point(self) -> Point2 {
        self.a()
    }

    /// Returns the line direction from `a` to `b`.
    #[must_use]
    pub const fn direction(self) -> Vector2 {
        Vector2::from_points(self.a(), self.b())
    }

    /// Returns `true` when `point` lies on the infinite line.
    #[must_use]
    pub fn contains_point(self, point: Point2) -> bool {
        signed_twice_area_2d(self.a(), self.b(), point) == 0.0
    }

    /// Returns `true` when `point` lies within `tolerance` of the line.
    ///
    /// # Errors
    ///
    /// Returns [`GeometryError::NonFiniteTolerance`] when `tolerance` is `NaN`
    /// or infinite.
    ///
    /// Returns [`GeometryError::NegativeTolerance`] when `tolerance` is negative.
    pub fn contains_point_with_tolerance(
        self,
        point: Point2,
        tolerance: f64,
    ) -> Result<bool, GeometryError> {
        let tolerance = GeometryError::validate_tolerance(tolerance)?;
        let direction_length = self.direction().length();

        if direction_length == 0.0 {
            return Ok(self.a().distance_to(point) <= tolerance);
        }

        Ok(signed_twice_area_2d(self.a(), self.b(), point).abs() <= tolerance * direction_length)
    }

    /// Returns the slope, or `None` for a vertical line.
    #[must_use]
    pub fn slope(self) -> Option<f64> {
        slope(self.a(), self.b())
    }

    /// Returns the slope when both line points contain only finite coordinates.
    ///
    /// # Errors
    ///
    /// Returns [`GeometryError::NonFiniteComponent`] when either point contains
    /// a non-finite coordinate.
    ///
    /// # Examples
    ///
    /// ```
    /// use use_geometry::{GeometryError, Line2, Point2};
    ///
    /// let diagonal = Line2::try_new(Point2::new(1.0, 1.0), Point2::new(3.0, 5.0))?;
    /// let vertical = Line2::try_new(Point2::new(2.0, 1.0), Point2::new(2.0, 5.0))?;
    ///
    /// assert_eq!(diagonal.try_slope()?, Some(2.0));
    /// assert_eq!(vertical.try_slope()?, None);
    /// # Ok::<(), GeometryError>(())
    /// ```
    pub fn try_slope(self) -> Result<Option<f64>, GeometryError> {
        try_slope(self.a(), self.b())
    }
}

/// Returns the slope between two points, or `None` for a vertical line.
#[must_use]
pub fn slope(left: Point2, right: Point2) -> Option<f64> {
    let delta_x = right.x() - left.x();
    if delta_x == 0.0 {
        None
    } else {
        Some((right.y() - left.y()) / delta_x)
    }
}

/// Returns the slope between two points when both points contain only finite coordinates.
///
/// # Errors
///
/// Returns [`GeometryError::NonFiniteComponent`] when either point contains a
/// non-finite coordinate.
pub fn try_slope(left: Point2, right: Point2) -> Result<Option<f64>, GeometryError> {
    let left = left.validate()?;
    let right = right.validate()?;

    Ok(slope(left, right))
}

#[cfg(test)]
mod tests {
    use super::{Line2, slope, try_slope};
    use crate::{error::GeometryError, point::Point2, vector::Vector2};

    #[test]
    fn constructs_lines() {
        let a = Point2::new(0.0, 0.0);
        let b = Point2::new(1.0, 1.0);

        assert_eq!(Line2::new(a, b).a(), a);
        assert_eq!(Line2::new(a, b).b(), b);
    }

    #[test]
    fn constructs_lines_with_try_new() {
        let a = Point2::new(0.0, 0.0);
        let b = Point2::new(1.0, 1.0);

        assert_eq!(Line2::try_new(a, b), Ok(Line2::new(a, b)));
        assert_eq!(Line2::try_from_points(a, b), Ok(Line2::new(a, b)));
    }

    #[test]
    fn computes_direction() {
        let line = Line2::new(Point2::new(0.0, 0.0), Point2::new(3.0, 4.0));

        assert_eq!(line.direction(), Vector2::new(3.0, 4.0));
        assert_eq!(line.point(), Point2::new(0.0, 0.0));
    }

    #[test]
    fn computes_slope() {
        let line = Line2::new(Point2::new(1.0, 1.0), Point2::new(3.0, 5.0));

        assert_eq!(line.slope(), Some(2.0));
        assert_eq!(slope(line.a(), line.b()), Some(2.0));
    }

    #[test]
    fn vertical_lines_have_no_slope() {
        let line = Line2::new(Point2::new(2.0, 1.0), Point2::new(2.0, 5.0));

        assert_eq!(line.slope(), None);
    }

    #[test]
    fn computes_try_slope_for_finite_lines() {
        let line = Line2::new(Point2::new(1.0, 1.0), Point2::new(3.0, 5.0));

        assert_eq!(line.try_slope(), Ok(Some(2.0)));
        assert_eq!(try_slope(line.a(), line.b()), Ok(Some(2.0)));
    }

    #[test]
    fn rejects_try_slope_for_non_finite_points() {
        assert!(matches!(
            try_slope(Point2::new(f64::NAN, 1.0), Point2::new(3.0, 5.0)),
            Err(GeometryError::NonFiniteComponent {
                type_name: "Point2",
                component: "x",
                value,
            }) if value.is_nan()
        ));
    }

    #[test]
    fn rejects_identical_points_for_validated_lines() {
        assert_eq!(
            Line2::try_new(Point2::new(1.0, 1.0), Point2::new(1.0, 1.0)),
            Err(GeometryError::IdenticalPoints)
        );
    }

    #[test]
    fn constructs_lines_from_point_and_direction() {
        let line = Line2::try_from_point_direction(Point2::new(1.0, 2.0), Vector2::new(3.0, 4.0))
            .expect("valid line");

        assert_eq!(
            line,
            Line2::new(Point2::new(1.0, 2.0), Point2::new(4.0, 6.0))
        );
    }

    #[test]
    fn rejects_zero_direction_vectors() {
        assert_eq!(
            Line2::try_from_point_direction(Point2::new(1.0, 2.0), Vector2::zero()),
            Err(GeometryError::ZeroDirectionVector)
        );
    }

    #[test]
    fn checks_line_containment() {
        let line =
            Line2::try_new(Point2::new(0.0, 0.0), Point2::new(2.0, 2.0)).expect("valid line");

        assert!(line.contains_point(Point2::new(4.0, 4.0)));
        assert!(!line.contains_point(Point2::new(4.0, 4.1)));
        assert_eq!(
            line.contains_point_with_tolerance(Point2::new(4.0, 4.1), 0.1),
            Ok(true)
        );
    }
}
