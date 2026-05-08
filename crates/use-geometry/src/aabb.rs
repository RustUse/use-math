use crate::{error::GeometryError, point::Point2};

/// An axis-aligned bounding box represented by inclusive minimum and maximum corners.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Aabb2 {
    min: Point2,
    max: Point2,
}

impl Aabb2 {
    /// Creates a validated axis-aligned bounding box from ordered corners.
    ///
    /// # Errors
    ///
    /// Returns [`GeometryError::NonFiniteComponent`] when either corner
    /// contains a non-finite coordinate.
    ///
    /// Returns [`GeometryError::InvalidBounds`] when `min` is greater than
    /// `max` on either axis.
    ///
    /// # Examples
    ///
    /// ```
    /// use use_geometry::{Aabb2, GeometryError, Point2};
    ///
    /// let bounds = Aabb2::try_new(Point2::new(1.0, 2.0), Point2::new(4.0, 6.0))?;
    /// assert_eq!(bounds.center(), Point2::new(2.5, 4.0));
    /// # Ok::<(), GeometryError>(())
    /// ```
    pub fn try_new(min: Point2, max: Point2) -> Result<Self, GeometryError> {
        let min = min.validate()?;
        let max = max.validate()?;

        if min.x() > max.x() || min.y() > max.y() {
            return Err(GeometryError::InvalidBounds {
                min_x: min.x(),
                min_y: min.y(),
                max_x: max.x(),
                max_y: max.y(),
            });
        }

        Ok(Self { min, max })
    }

    /// Creates a bounding box from any two corners, normalizing axis order.
    ///
    /// # Examples
    ///
    /// ```
    /// use use_geometry::{Aabb2, Point2};
    ///
    /// let bounds = Aabb2::from_points(Point2::new(4.0, 1.0), Point2::new(1.0, 3.0));
    ///
    /// assert_eq!(bounds.min(), Point2::new(1.0, 1.0));
    /// assert_eq!(bounds.max(), Point2::new(4.0, 3.0));
    /// ```
    #[must_use]
    pub const fn from_points(a: Point2, b: Point2) -> Self {
        Self {
            min: Point2::new(a.x().min(b.x()), a.y().min(b.y())),
            max: Point2::new(a.x().max(b.x()), a.y().max(b.y())),
        }
    }

    /// Returns the inclusive minimum corner.
    #[must_use]
    pub const fn min(&self) -> Point2 {
        self.min
    }

    /// Returns the inclusive maximum corner.
    #[must_use]
    pub const fn max(&self) -> Point2 {
        self.max
    }

    /// Returns the box width.
    #[must_use]
    pub fn width(&self) -> f64 {
        self.max.x() - self.min.x()
    }

    /// Returns the box height.
    #[must_use]
    pub fn height(&self) -> f64 {
        self.max.y() - self.min.y()
    }

    /// Returns the box center.
    #[must_use]
    pub const fn center(&self) -> Point2 {
        self.min.midpoint(self.max)
    }

    /// Returns the box area.
    #[must_use]
    pub fn area(&self) -> f64 {
        self.width() * self.height()
    }

    /// Returns `true` when `point` lies inside or on the boundary.
    #[must_use]
    pub fn contains_point(&self, point: Point2) -> bool {
        point.x() >= self.min.x()
            && point.x() <= self.max.x()
            && point.y() >= self.min.y()
            && point.y() <= self.max.y()
    }

    /// Returns `true` when `point` lies inside the box expanded by `tolerance`.
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
    /// use use_geometry::{Aabb2, GeometryError, Point2};
    ///
    /// let bounds = Aabb2::from_points(Point2::new(1.0, 1.0), Point2::new(4.0, 3.0));
    /// assert!(bounds.contains_point_with_tolerance(Point2::new(4.25, 3.0), 0.25)?);
    /// # Ok::<(), GeometryError>(())
    /// ```
    pub fn contains_point_with_tolerance(
        &self,
        point: Point2,
        tolerance: f64,
    ) -> Result<bool, GeometryError> {
        let tolerance = GeometryError::validate_tolerance(tolerance)?;

        Ok(point.x() >= self.min.x() - tolerance
            && point.x() <= self.max.x() + tolerance
            && point.y() >= self.min.y() - tolerance
            && point.y() <= self.max.y() + tolerance)
    }

    /// Returns `true` when the box has zero width or height.
    #[must_use]
    pub fn is_degenerate(&self) -> bool {
        self.width() == 0.0 || self.height() == 0.0
    }

    /// Returns `true` when the box width or height is within `tolerance` of zero.
    ///
    /// # Errors
    ///
    /// Returns [`GeometryError::NonFiniteTolerance`] when `tolerance` is `NaN`
    /// or infinite.
    ///
    /// Returns [`GeometryError::NegativeTolerance`] when `tolerance` is negative.
    pub fn is_degenerate_with_tolerance(&self, tolerance: f64) -> Result<bool, GeometryError> {
        let tolerance = GeometryError::validate_tolerance(tolerance)?;

        Ok(self.width() <= tolerance || self.height() <= tolerance)
    }
}

/// Creates a bounding box from any two corners.
#[must_use]
pub const fn aabb_from_points(a: Point2, b: Point2) -> Aabb2 {
    Aabb2::from_points(a, b)
}

#[cfg(test)]
mod tests {
    use super::{Aabb2, aabb_from_points};
    use crate::{Circle, GeometryError, Point2, Segment2, Triangle};

    fn approx_eq(left: f64, right: f64) -> bool {
        (left - right).abs() < 1.0e-10
    }

    #[test]
    fn constructs_valid_aabbs() {
        let bounds =
            Aabb2::try_new(Point2::new(1.0, 2.0), Point2::new(4.0, 6.0)).expect("valid bounds");

        assert_eq!(bounds.min(), Point2::new(1.0, 2.0));
        assert_eq!(bounds.max(), Point2::new(4.0, 6.0));
    }

    #[test]
    fn rejects_invalid_aabb_ordering() {
        assert_eq!(
            Aabb2::try_new(Point2::new(4.0, 1.0), Point2::new(1.0, 3.0)),
            Err(GeometryError::InvalidBounds {
                min_x: 4.0,
                min_y: 1.0,
                max_x: 1.0,
                max_y: 3.0,
            })
        );
    }

    #[test]
    fn normalizes_point_order() {
        let bounds = Aabb2::from_points(Point2::new(4.0, 1.0), Point2::new(1.0, 3.0));

        assert_eq!(bounds.min(), Point2::new(1.0, 1.0));
        assert_eq!(bounds.max(), Point2::new(4.0, 3.0));
        assert_eq!(
            aabb_from_points(Point2::new(4.0, 1.0), Point2::new(1.0, 3.0)),
            bounds
        );
    }

    #[test]
    fn computes_dimensions_center_and_area() {
        let bounds = Aabb2::from_points(Point2::new(1.0, 1.0), Point2::new(4.0, 3.0));

        assert!(approx_eq(bounds.width(), 3.0));
        assert!(approx_eq(bounds.height(), 2.0));
        assert_eq!(bounds.center(), Point2::new(2.5, 2.0));
        assert!(approx_eq(bounds.area(), 6.0));
    }

    #[test]
    fn contains_points_including_boundary() {
        let bounds = Aabb2::from_points(Point2::new(1.0, 1.0), Point2::new(4.0, 3.0));

        assert!(bounds.contains_point(Point2::new(2.0, 2.0)));
        assert!(bounds.contains_point(Point2::new(1.0, 3.0)));
        assert!(!bounds.contains_point(Point2::new(4.5, 3.0)));
    }

    #[test]
    fn supports_tolerance_based_containment() {
        let bounds = Aabb2::from_points(Point2::new(1.0, 1.0), Point2::new(4.0, 3.0));

        assert_eq!(
            bounds.contains_point_with_tolerance(Point2::new(4.25, 3.0), 0.25),
            Ok(true)
        );
        assert_eq!(
            bounds.contains_point_with_tolerance(Point2::new(4.25, 3.0), -0.25),
            Err(GeometryError::NegativeTolerance(-0.25))
        );
    }

    #[test]
    fn detects_degenerate_bounds() {
        let point_bounds = Aabb2::from_points(Point2::new(2.0, 2.0), Point2::new(2.0, 2.0));
        let line_bounds = Aabb2::from_points(Point2::new(2.0, 1.0), Point2::new(2.0, 3.0));

        assert!(point_bounds.is_degenerate());
        assert!(line_bounds.is_degenerate());
        assert_eq!(line_bounds.is_degenerate_with_tolerance(0.0), Ok(true));
    }

    #[test]
    fn builds_bounds_from_primitives() {
        let point = Point2::new(2.0, 3.0);
        let segment = Segment2::new(Point2::new(1.0, 4.0), Point2::new(3.0, 2.0));
        let circle = Circle::try_new(Point2::new(2.0, 2.0), 1.5).expect("valid circle");
        let triangle = Triangle::new(
            Point2::new(0.0, 1.0),
            Point2::new(4.0, 3.0),
            Point2::new(2.0, -1.0),
        );

        assert_eq!(point.aabb(), Aabb2::from_points(point, point));
        assert_eq!(
            segment.aabb(),
            Aabb2::from_points(Point2::new(1.0, 2.0), Point2::new(3.0, 4.0))
        );
        assert_eq!(
            circle.aabb(),
            Aabb2::from_points(Point2::new(0.5, 0.5), Point2::new(3.5, 3.5))
        );
        assert_eq!(
            triangle.aabb(),
            Aabb2::from_points(Point2::new(0.0, -1.0), Point2::new(4.0, 3.0))
        );
    }
}
