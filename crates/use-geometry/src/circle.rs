use core::f64::consts::PI;

use crate::{aabb::Aabb2, error::GeometryError, point::Point2};

/// A circle in 2D Euclidean space.
///
/// `Circle` stores a center point and a radius. Construction validates the
/// radius so downstream methods can stay simple.
///
/// # Examples
///
/// ```rust
/// use use_geometry::{Circle, Point2};
///
/// let circle = Circle::try_new(Point2::new(1.0, 2.0), 3.0)?;
///
/// assert_eq!(circle.center(), Point2::new(1.0, 2.0));
/// assert_eq!(circle.radius(), 3.0);
/// assert_eq!(circle.diameter(), 6.0);
/// # Ok::<(), use_geometry::GeometryError>(())
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Circle {
    center: Point2,
    radius: f64,
}

impl Circle {
    /// Creates a circle from a center point and a finite, non-negative radius.
    ///
    /// A radius of `0.0` is valid and represents a degenerate circle centered
    /// at `center`.
    ///
    /// # Errors
    ///
    /// Returns [`GeometryError::NonFiniteComponent`] when `center` contains a
    /// non-finite coordinate.
    ///
    /// Returns [`GeometryError::NonFiniteRadius`] when `radius` is `NaN` or
    /// infinite.
    ///
    /// Returns [`GeometryError::NegativeRadius`] when `radius` is negative.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_geometry::{Circle, Point2};
    ///
    /// let circle = Circle::try_new(Point2::new(0.0, 0.0), 2.5)?;
    ///
    /// assert_eq!(circle.center(), Point2::new(0.0, 0.0));
    /// assert_eq!(circle.radius(), 2.5);
    /// # Ok::<(), use_geometry::GeometryError>(())
    /// ```
    pub fn try_new(center: Point2, radius: f64) -> Result<Self, GeometryError> {
        let center = center.validate()?;

        if !radius.is_finite() {
            return Err(GeometryError::NonFiniteRadius(radius));
        }

        if radius < 0.0 {
            return Err(GeometryError::NegativeRadius(radius));
        }

        Ok(Self { center, radius })
    }

    /// Returns the circle center.
    #[must_use]
    pub const fn center(&self) -> Point2 {
        self.center
    }

    /// Returns the circle radius.
    #[must_use]
    pub const fn radius(&self) -> f64 {
        self.radius
    }

    /// Returns the circle diameter.
    #[must_use]
    pub fn diameter(&self) -> f64 {
        self.radius * 2.0
    }

    /// Returns the circle area.
    #[must_use]
    pub fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }

    /// Returns the circle circumference.
    #[must_use]
    pub fn circumference(&self) -> f64 {
        2.0 * PI * self.radius
    }

    /// Returns `true` when `point` lies inside or on the circle boundary.
    #[must_use]
    pub fn contains_point(&self, point: Point2) -> bool {
        self.center.distance_squared_to(point) <= self.radius * self.radius
    }

    /// Returns `true` when `point` lies inside the circle expanded by `tolerance`.
    ///
    /// # Errors
    ///
    /// Returns [`GeometryError::NonFiniteTolerance`] when `tolerance` is `NaN`
    /// or infinite.
    ///
    /// Returns [`GeometryError::NegativeTolerance`] when `tolerance` is negative.
    pub fn contains_point_with_tolerance(
        &self,
        point: Point2,
        tolerance: f64,
    ) -> Result<bool, GeometryError> {
        let tolerance = GeometryError::validate_tolerance(tolerance)?;
        let radius = self.radius + tolerance;

        Ok(self.center.distance_squared_to(point) <= radius * radius)
    }

    /// Returns the circle bounding box.
    #[must_use]
    pub fn aabb(&self) -> Aabb2 {
        Aabb2::from_points(
            Point2::new(self.center.x() - self.radius, self.center.y() - self.radius),
            Point2::new(self.center.x() + self.radius, self.center.y() + self.radius),
        )
    }
}

#[cfg(test)]
mod tests {
    use core::f64::consts::PI;

    use super::Circle;
    use crate::{error::GeometryError, point::Point2};

    fn approx_eq(left: f64, right: f64) -> bool {
        (left - right).abs() < 1.0e-10
    }

    #[test]
    fn constructs_circle_with_valid_radius() {
        let circle = Circle::try_new(Point2::new(1.0, 2.0), 3.0).expect("valid circle");

        assert_eq!(circle.center(), Point2::new(1.0, 2.0));
        assert!(approx_eq(circle.radius(), 3.0));
    }

    #[test]
    fn constructs_circle_with_zero_radius() {
        let circle = Circle::try_new(Point2::origin(), 0.0).expect("zero radius should be valid");

        assert!(approx_eq(circle.radius(), 0.0));
    }

    #[test]
    fn rejects_negative_radius() {
        assert_eq!(
            Circle::try_new(Point2::origin(), -1.0),
            Err(GeometryError::NegativeRadius(-1.0))
        );
    }

    #[test]
    fn rejects_nan_radius() {
        let radius = f64::NAN;

        assert!(matches!(
            Circle::try_new(Point2::origin(), radius),
            Err(GeometryError::NonFiniteRadius(value)) if value.is_nan()
        ));
    }

    #[test]
    fn rejects_infinite_radius() {
        assert_eq!(
            Circle::try_new(Point2::origin(), f64::INFINITY),
            Err(GeometryError::NonFiniteRadius(f64::INFINITY))
        );
    }

    #[test]
    fn rejects_non_finite_center_coordinates() {
        assert!(matches!(
            Circle::try_new(Point2::new(f64::NAN, 0.0), 1.0),
            Err(GeometryError::NonFiniteComponent {
                type_name: "Point2",
                component: "x",
                value,
            }) if value.is_nan()
        ));
    }

    #[test]
    fn computes_area() {
        let circle = Circle::try_new(Point2::origin(), 3.0).expect("valid circle");

        assert!(approx_eq(circle.area(), PI * 9.0));
    }

    #[test]
    fn computes_diameter() {
        let circle = Circle::try_new(Point2::origin(), 3.0).expect("valid circle");

        assert!(approx_eq(circle.diameter(), 6.0));
    }

    #[test]
    fn computes_circumference() {
        let circle = Circle::try_new(Point2::origin(), 3.0).expect("valid circle");

        assert!(approx_eq(circle.circumference(), 2.0 * PI * 3.0));
    }

    #[test]
    fn contains_points() {
        let circle = Circle::try_new(Point2::origin(), 3.0).expect("valid circle");

        assert!(circle.contains_point(Point2::new(0.0, 0.0)));
        assert!(circle.contains_point(Point2::new(3.0, 0.0)));
        assert!(!circle.contains_point(Point2::new(3.1, 0.0)));
    }

    #[test]
    fn supports_tolerance_based_containment() {
        let circle = Circle::try_new(Point2::origin(), 3.0).expect("valid circle");

        assert_eq!(
            circle.contains_point_with_tolerance(Point2::new(3.1, 0.0), 0.1),
            Ok(true)
        );
        assert_eq!(
            circle.contains_point_with_tolerance(Point2::new(3.1, 0.0), -0.1),
            Err(GeometryError::NegativeTolerance(-0.1))
        );
    }

    #[test]
    fn computes_bounds() {
        let circle = Circle::try_new(Point2::new(2.0, 3.0), 1.5).expect("valid circle");

        assert_eq!(circle.aabb().min(), Point2::new(0.5, 1.5));
        assert_eq!(circle.aabb().max(), Point2::new(3.5, 4.5));
    }
}
