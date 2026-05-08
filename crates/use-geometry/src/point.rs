use core::ops::{Add, Sub};

use crate::{aabb::Aabb2, error::GeometryError, vector::Vector2};

/// A 2D point represented with `f64` coordinates.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point2 {
    /// The horizontal coordinate.
    x: f64,
    /// The vertical coordinate.
    y: f64,
}

impl Point2 {
    /// Creates a point from `x` and `y` coordinates.
    #[must_use]
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// Returns the horizontal coordinate.
    #[must_use]
    pub const fn x(&self) -> f64 {
        self.x
    }

    /// Returns the vertical coordinate.
    #[must_use]
    pub const fn y(&self) -> f64 {
        self.y
    }

    /// Creates a point from finite `x` and `y` coordinates.
    ///
    /// # Errors
    ///
    /// Returns [`GeometryError::NonFiniteComponent`] when `x` or `y` is `NaN`
    /// or infinite.
    ///
    /// # Examples
    ///
    /// ```
    /// use use_geometry::{GeometryError, Point2};
    ///
    /// let point = Point2::try_new(1.0, -2.0)?;
    /// assert_eq!(point, Point2::new(1.0, -2.0));
    ///
    /// assert!(matches!(
    ///     Point2::try_new(f64::NAN, 0.0),
    ///     Err(GeometryError::NonFiniteComponent { component: "x", .. })
    /// ));
    /// # Ok::<(), GeometryError>(())
    /// ```
    pub const fn try_new(x: f64, y: f64) -> Result<Self, GeometryError> {
        if !x.is_finite() {
            return Err(GeometryError::non_finite_component("Point2", "x", x));
        }

        if !y.is_finite() {
            return Err(GeometryError::non_finite_component("Point2", "y", y));
        }

        Ok(Self::new(x, y))
    }

    /// Validates that an existing point contains only finite coordinates.
    ///
    /// # Errors
    ///
    /// Returns [`GeometryError::NonFiniteComponent`] when `self.x` or
    /// `self.y` is `NaN` or infinite.
    ///
    /// # Examples
    ///
    /// ```
    /// use use_geometry::{GeometryError, Point2};
    ///
    /// let validated = Point2::new(3.0, 4.0).validate()?;
    /// assert_eq!(validated, Point2::new(3.0, 4.0));
    /// # Ok::<(), GeometryError>(())
    /// ```
    pub const fn validate(self) -> Result<Self, GeometryError> {
        Self::try_new(self.x, self.y)
    }

    /// Returns `true` when both coordinates are finite.
    #[must_use]
    pub const fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite()
    }

    /// Returns the origin `(0, 0)`.
    #[must_use]
    pub const fn origin() -> Self {
        Self::new(0.0, 0.0)
    }

    /// Returns the Euclidean distance to another point.
    ///
    /// # Examples
    ///
    /// ```
    /// use use_geometry::Point2;
    ///
    /// let origin = Point2::new(0.0, 0.0);
    /// let point = Point2::new(3.0, 4.0);
    ///
    /// assert_eq!(origin.distance_to(point), 5.0);
    /// ```
    #[must_use]
    pub fn distance_to(self, other: Self) -> f64 {
        self.distance_squared_to(other).sqrt()
    }

    /// Returns the squared Euclidean distance to another point.
    ///
    /// # Examples
    ///
    /// ```
    /// use use_geometry::Point2;
    ///
    /// let left = Point2::new(0.0, 0.0);
    /// let right = Point2::new(3.0, 4.0);
    ///
    /// assert_eq!(left.distance_squared_to(right), 25.0);
    /// ```
    #[must_use]
    pub fn distance_squared_to(self, other: Self) -> f64 {
        let delta_x = other.x - self.x;
        let delta_y = other.y - self.y;

        delta_x.mul_add(delta_x, delta_y * delta_y)
    }

    /// Returns the midpoint between this point and another point.
    ///
    /// # Examples
    ///
    /// ```
    /// use use_geometry::Point2;
    ///
    /// let left = Point2::new(-2.0, 1.0);
    /// let right = Point2::new(4.0, 5.0);
    ///
    /// assert_eq!(left.midpoint(right), Point2::new(1.0, 3.0));
    /// ```
    #[must_use]
    pub const fn midpoint(self, other: Self) -> Self {
        Self::new(self.x.midpoint(other.x), self.y.midpoint(other.y))
    }

    /// Returns a point interpolated between this point and `other`.
    ///
    /// # Examples
    ///
    /// ```
    /// use use_geometry::Point2;
    ///
    /// let start = Point2::new(0.0, 0.0);
    /// let end = Point2::new(8.0, 4.0);
    ///
    /// assert_eq!(start.lerp(end, 0.25), Point2::new(2.0, 1.0));
    /// ```
    #[must_use]
    pub const fn lerp(self, other: Self, t: f64) -> Self {
        Self::new(
            self.x + ((other.x - self.x) * t),
            self.y + ((other.y - self.y) * t),
        )
    }

    /// Returns a point translated by a vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use use_geometry::{Point2, Vector2};
    ///
    /// let point = Point2::new(1.5, -2.0);
    /// let offset = Vector2::new(2.0, 3.5);
    ///
    /// assert_eq!(point.translate(offset), Point2::new(3.5, 1.5));
    /// ```
    #[must_use]
    pub const fn translate(self, vector: Vector2) -> Self {
        Self::new(self.x + vector.x(), self.y + vector.y())
    }

    /// Returns the degenerate bounding box rooted at this point.
    #[must_use]
    pub const fn aabb(self) -> Aabb2 {
        Aabb2::from_points(self, self)
    }
}

impl Add<Vector2> for Point2 {
    type Output = Self;

    fn add(self, rhs: Vector2) -> Self::Output {
        Self::new(self.x + rhs.x(), self.y + rhs.y())
    }
}

impl Sub<Vector2> for Point2 {
    type Output = Self;

    fn sub(self, rhs: Vector2) -> Self::Output {
        Self::new(self.x - rhs.x(), self.y - rhs.y())
    }
}

impl Sub<Self> for Point2 {
    type Output = Vector2;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

#[cfg(test)]
mod tests {
    use super::Point2;
    use crate::{Aabb2, distance::midpoint_2d, error::GeometryError, vector::Vector2};

    fn approx_eq(left: f64, right: f64) -> bool {
        (left - right).abs() < 1.0e-10
    }

    #[test]
    fn constructs_points() {
        assert_eq!(
            Point2::new(1.0, 2.0),
            Point2::try_new(1.0, 2.0).expect("valid point")
        );
    }

    #[test]
    fn constructs_points_with_try_new() {
        assert_eq!(Point2::try_new(1.0, 2.0), Ok(Point2::new(1.0, 2.0)));
    }

    #[test]
    fn rejects_non_finite_x_coordinates() {
        assert!(matches!(
            Point2::try_new(f64::NAN, 2.0),
            Err(GeometryError::NonFiniteComponent {
                type_name: "Point2",
                component: "x",
                value,
            }) if value.is_nan()
        ));
    }

    #[test]
    fn rejects_non_finite_y_coordinates() {
        assert_eq!(
            Point2::try_new(1.0, f64::INFINITY),
            Err(GeometryError::NonFiniteComponent {
                type_name: "Point2",
                component: "y",
                value: f64::INFINITY,
            })
        );
    }

    #[test]
    fn validates_existing_points() {
        assert_eq!(Point2::new(1.0, 2.0).validate(), Ok(Point2::new(1.0, 2.0)));
    }

    #[test]
    fn returns_origin() {
        assert_eq!(Point2::origin(), Point2::new(0.0, 0.0));
    }

    #[test]
    fn computes_distance_to_other_point() {
        let left = Point2::new(0.0, 0.0);
        let right = Point2::new(3.0, 4.0);

        assert!(approx_eq(left.distance_to(right), 5.0));
        assert!(approx_eq(left.distance_squared_to(right), 25.0));
    }

    #[test]
    fn computes_midpoints() {
        let left = Point2::new(0.0, 0.0);
        let right = Point2::new(4.0, 2.0);

        assert_eq!(left.midpoint(right), Point2::new(2.0, 1.0));
        assert_eq!(midpoint_2d(left, right), Point2::new(2.0, 1.0));
        assert_eq!(left.lerp(right, 0.25), Point2::new(1.0, 0.5));
    }

    #[test]
    fn translates_by_vector() {
        let point = Point2::new(1.5, -2.0);
        let offset = Vector2::new(2.0, 3.5);

        assert_eq!(point.translate(offset), Point2::new(3.5, 1.5));
        assert_eq!(point + offset, Point2::new(3.5, 1.5));
        assert_eq!(point + offset - offset, point);
    }

    #[test]
    fn exposes_accessors_and_finite_checks() {
        let point = Point2::new(1.5, -2.0);

        assert!(approx_eq(point.x(), 1.5));
        assert!(approx_eq(point.y(), -2.0));
        assert!(point.is_finite());
        assert!(!Point2::new(f64::NAN, 0.0).is_finite());
    }

    #[test]
    fn converts_points_to_vectors_and_bounds() {
        let start = Point2::new(1.0, 2.0);
        let end = Point2::new(4.0, 6.0);

        assert_eq!(end - start, Vector2::new(3.0, 4.0));
        assert_eq!(start.aabb(), Aabb2::from_points(start, start));
    }
}
