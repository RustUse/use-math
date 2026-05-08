use crate::{aabb::Aabb2, error::GeometryError, point::Point2, vector::Vector2};

/// A finite line segment between two 2D points.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Segment2 {
    /// The segment start point.
    start: Point2,
    /// The segment end point.
    end: Point2,
}

impl Segment2 {
    /// Creates a segment from `start` to `end`.
    #[must_use]
    pub const fn new(start: Point2, end: Point2) -> Self {
        Self { start, end }
    }

    /// Creates a segment from `start` to `end` with finite point coordinates.
    ///
    /// # Errors
    ///
    /// Returns [`GeometryError::NonFiniteComponent`] when either input point
    /// contains a non-finite coordinate.
    ///
    /// # Examples
    ///
    /// ```
    /// use use_geometry::{GeometryError, Point2, Segment2};
    ///
    /// let segment = Segment2::try_new(Point2::new(0.0, 0.0), Point2::new(4.0, 2.0))?;
    /// assert_eq!(segment.midpoint(), Point2::new(2.0, 1.0));
    /// # Ok::<(), GeometryError>(())
    /// ```
    pub fn try_new(start: Point2, end: Point2) -> Result<Self, GeometryError> {
        Ok(Self::new(start.validate()?, end.validate()?))
    }

    /// Returns the segment start point.
    #[must_use]
    pub const fn start(self) -> Point2 {
        self.start
    }

    /// Returns the segment end point.
    #[must_use]
    pub const fn end(self) -> Point2 {
        self.end
    }

    /// Returns the segment length.
    #[must_use]
    pub fn length(self) -> f64 {
        self.start.distance_to(self.end)
    }

    /// Returns the squared segment length.
    #[must_use]
    pub fn length_squared(self) -> f64 {
        self.start.distance_squared_to(self.end)
    }

    /// Returns the segment midpoint.
    #[must_use]
    pub const fn midpoint(self) -> Point2 {
        self.start.midpoint(self.end)
    }

    /// Returns the segment vector from `start` to `end`.
    #[must_use]
    pub const fn vector(self) -> Vector2 {
        Vector2::from_points(self.start, self.end)
    }

    /// Returns the point at parameter `t` along the segment.
    ///
    /// # Examples
    ///
    /// ```
    /// use use_geometry::{Point2, Segment2};
    ///
    /// let segment = Segment2::new(Point2::new(0.0, 0.0), Point2::new(4.0, 2.0));
    /// assert_eq!(segment.point_at(0.25), Point2::new(1.0, 0.5));
    /// ```
    #[must_use]
    pub const fn point_at(self, t: f64) -> Point2 {
        self.start.lerp(self.end, t)
    }

    /// Returns the segment with its endpoints reversed.
    ///
    /// # Examples
    ///
    /// ```
    /// use use_geometry::{Point2, Segment2};
    ///
    /// let segment = Segment2::new(Point2::new(1.0, 2.0), Point2::new(4.0, 6.0));
    ///
    /// assert_eq!(segment.reversed().start(), Point2::new(4.0, 6.0));
    /// assert_eq!(segment.reversed().end(), Point2::new(1.0, 2.0));
    /// ```
    #[must_use]
    pub const fn reversed(self) -> Self {
        Self::new(self.end, self.start)
    }

    /// Returns `true` when the segment collapses to a single point.
    #[must_use]
    pub fn is_degenerate(self) -> bool {
        self.length_squared() == 0.0
    }

    /// Returns `true` when the segment length is within `tolerance` of zero.
    ///
    /// # Errors
    ///
    /// Returns [`GeometryError::NonFiniteTolerance`] when `tolerance` is `NaN`
    /// or infinite.
    ///
    /// Returns [`GeometryError::NegativeTolerance`] when `tolerance` is negative.
    ///
    /// # Examples
    ///
    /// ```
    /// use use_geometry::{GeometryError, Point2, Segment2};
    ///
    /// let segment = Segment2::new(Point2::new(2.0, 2.0), Point2::new(2.0, 2.0));
    /// assert!(segment.is_degenerate_with_tolerance(0.0)?);
    /// # Ok::<(), GeometryError>(())
    /// ```
    pub fn is_degenerate_with_tolerance(self, tolerance: f64) -> Result<bool, GeometryError> {
        let tolerance = GeometryError::validate_tolerance(tolerance)?;

        Ok(self.length_squared() <= tolerance * tolerance)
    }

    /// Returns the segment bounding box.
    #[must_use]
    pub const fn aabb(self) -> Aabb2 {
        Aabb2::from_points(self.start, self.end)
    }
}

#[cfg(test)]
mod tests {
    use super::Segment2;
    use crate::{error::GeometryError, point::Point2, vector::Vector2};

    fn approx_eq(left: f64, right: f64) -> bool {
        (left - right).abs() < 1.0e-10
    }

    #[test]
    fn constructs_segments() {
        let start = Point2::new(0.0, 0.0);
        let end = Point2::new(1.0, 1.0);

        assert_eq!(Segment2::new(start, end).start(), start);
        assert_eq!(Segment2::new(start, end).end(), end);
    }

    #[test]
    fn constructs_segments_with_try_new() {
        let start = Point2::new(0.0, 0.0);
        let end = Point2::new(1.0, 1.0);

        assert_eq!(Segment2::try_new(start, end), Ok(Segment2::new(start, end)));
    }

    #[test]
    fn rejects_non_finite_segment_points() {
        assert_eq!(
            Segment2::try_new(Point2::new(0.0, 0.0), Point2::new(1.0, f64::INFINITY)),
            Err(GeometryError::NonFiniteComponent {
                type_name: "Point2",
                component: "y",
                value: f64::INFINITY,
            })
        );
    }

    #[test]
    fn computes_length() {
        let segment = Segment2::new(Point2::new(0.0, 0.0), Point2::new(3.0, 4.0));

        assert!(approx_eq(segment.length(), 5.0));
        assert!(approx_eq(segment.length_squared(), 25.0));
    }

    #[test]
    fn computes_midpoint() {
        let segment = Segment2::new(Point2::new(0.0, 0.0), Point2::new(4.0, 2.0));

        assert_eq!(segment.midpoint(), Point2::new(2.0, 1.0));
        assert_eq!(segment.point_at(0.25), Point2::new(1.0, 0.5));
    }

    #[test]
    fn computes_vector() {
        let segment = Segment2::new(Point2::new(1.0, 2.0), Point2::new(4.0, 6.0));

        assert_eq!(segment.vector(), Vector2::new(3.0, 4.0));
        assert_eq!(segment.start(), Point2::new(1.0, 2.0));
        assert_eq!(segment.end(), Point2::new(4.0, 6.0));
        assert_eq!(
            segment.reversed(),
            Segment2::new(Point2::new(4.0, 6.0), Point2::new(1.0, 2.0))
        );
    }

    #[test]
    fn detects_degenerate_segments() {
        let segment = Segment2::new(Point2::new(2.0, 2.0), Point2::new(2.0, 2.0));

        assert!(segment.is_degenerate());
        assert_eq!(segment.is_degenerate_with_tolerance(0.0), Ok(true));
        assert_eq!(
            segment.is_degenerate_with_tolerance(-1.0),
            Err(GeometryError::NegativeTolerance(-1.0))
        );
    }

    #[test]
    fn computes_segment_bounds() {
        let segment = Segment2::new(Point2::new(4.0, 1.0), Point2::new(1.0, 3.0));

        assert_eq!(segment.aabb().min(), Point2::new(1.0, 1.0));
        assert_eq!(segment.aabb().max(), Point2::new(4.0, 3.0));
    }
}
