use crate::{
    aabb::Aabb2,
    distance::distance_2d,
    error::GeometryError,
    orientation::{Orientation2, orientation_2d, signed_twice_area_2d},
    point::Point2,
};

/// A constructed 2D triangle represented by three vertices.
///
/// Use [`Triangle::try_new`] when the vertices come from user input, files,
/// or other external sources. Use [`Triangle::new`] when the points are
/// already trusted.
///
/// # Examples
///
/// ```rust
/// use use_geometry::{
///     Orientation2, Point2, Triangle, triangle_area,
///     triangle_twice_signed_area,
/// };
///
/// let a = Point2::try_new(0.0, 0.0)?;
/// let b = Point2::try_new(4.0, 0.0)?;
/// let c = Point2::try_new(0.0, 3.0)?;
/// let triangle = Triangle::try_new(a, b, c)?;
///
/// assert_eq!(triangle.orientation(), Orientation2::CounterClockwise);
/// assert_eq!(triangle.twice_signed_area(), triangle_twice_signed_area(a, b, c));
/// assert_eq!(triangle.area(), triangle_area(a, b, c));
/// assert_eq!(triangle.sides(), [4.0, 5.0, 3.0]);
/// # Ok::<(), use_geometry::GeometryError>(())
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Triangle {
    /// The first vertex.
    a: Point2,
    /// The second vertex.
    b: Point2,
    /// The third vertex.
    c: Point2,
}

impl Triangle {
    /// Creates a triangle from three points.
    #[must_use]
    pub const fn new(a: Point2, b: Point2, c: Point2) -> Self {
        Self { a, b, c }
    }

    /// Creates a triangle from three points with finite coordinates.
    ///
    /// Use this constructor at API boundaries where coordinates may still need
    /// validation. [`Triangle::new`] remains available for already-validated
    /// points.
    ///
    /// # Errors
    ///
    /// Returns [`GeometryError::NonFiniteComponent`] when any vertex contains a
    /// non-finite coordinate.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_geometry::{Point2, Triangle};
    ///
    /// let triangle = Triangle::try_new(
    ///     Point2::try_new(0.0, 0.0)?,
    ///     Point2::try_new(4.0, 0.0)?,
    ///     Point2::try_new(0.0, 3.0)?,
    /// )?;
    ///
    /// assert_eq!(triangle.area(), 6.0);
    /// # Ok::<(), use_geometry::GeometryError>(())
    /// ```
    pub fn try_new(a: Point2, b: Point2, c: Point2) -> Result<Self, GeometryError> {
        Ok(Self::new(a.validate()?, b.validate()?, c.validate()?))
    }

    /// Returns the first vertex.
    #[must_use]
    pub const fn a(self) -> Point2 {
        self.a
    }

    /// Returns the second vertex.
    #[must_use]
    pub const fn b(self) -> Point2 {
        self.b
    }

    /// Returns the third vertex.
    #[must_use]
    pub const fn c(self) -> Point2 {
        self.c
    }

    /// Returns the triangle vertices in `[a, b, c]` order.
    #[must_use]
    pub const fn vertices(self) -> [Point2; 3] {
        [self.a(), self.b(), self.c()]
    }

    /// Returns twice the signed area of the triangle.
    ///
    /// The sign depends on the vertex winding order.
    #[must_use]
    pub fn twice_signed_area(self) -> f64 {
        triangle_twice_signed_area(self.a(), self.b(), self.c())
    }

    /// Returns twice the unsigned area of the triangle.
    #[must_use]
    pub fn twice_area(self) -> f64 {
        triangle_twice_area(self.a(), self.b(), self.c())
    }

    /// Returns the triangle orientation implied by the vertex winding order.
    #[must_use]
    pub fn orientation(self) -> Orientation2 {
        orientation_2d(self.a(), self.b(), self.c())
    }

    /// Returns the triangle area.
    #[must_use]
    pub fn area(self) -> f64 {
        self.twice_area() * 0.5
    }

    /// Returns the triangle side lengths in `[ab, bc, ca]` order.
    #[must_use]
    pub fn sides(self) -> [f64; 3] {
        [
            distance_2d(self.a(), self.b()),
            distance_2d(self.b(), self.c()),
            distance_2d(self.c(), self.a()),
        ]
    }

    /// Returns the triangle perimeter.
    #[must_use]
    pub fn perimeter(self) -> f64 {
        self.sides().into_iter().sum()
    }

    /// Returns the triangle centroid.
    #[must_use]
    pub fn centroid(self) -> Point2 {
        let [a, b, c] = self.vertices();

        Point2::new((a.x() + b.x() + c.x()) / 3.0, (a.y() + b.y() + c.y()) / 3.0)
    }

    /// Returns `true` when the triangle is exactly degenerate.
    ///
    /// Exact degeneracy means the signed twice-area is exactly zero, which in
    /// turn means the vertices are collinear.
    #[must_use]
    pub fn is_degenerate(self) -> bool {
        self.twice_signed_area() == 0.0
    }

    /// Returns `true` when the triangle's unsigned twice-area is within `tolerance` of zero.
    ///
    /// Use this when you care about practical collapse in measured or generated
    /// geometry rather than exact arithmetic collapse.
    ///
    /// # Errors
    ///
    /// Returns [`GeometryError::NonFiniteTolerance`] when `tolerance` is `NaN`
    /// or infinite.
    ///
    /// Returns [`GeometryError::NegativeTolerance`] when `tolerance` is negative.
    pub fn is_degenerate_with_tolerance(self, tolerance: f64) -> Result<bool, GeometryError> {
        let tolerance = GeometryError::validate_tolerance(tolerance)?;

        Ok(self.twice_signed_area().abs() <= tolerance)
    }

    /// Returns the triangle bounding box.
    #[must_use]
    pub const fn aabb(self) -> Aabb2 {
        let [a, b, c] = self.vertices();
        let min_x = a.x().min(b.x()).min(c.x());
        let min_y = a.y().min(b.y()).min(c.y());
        let max_x = a.x().max(b.x()).max(c.x());
        let max_y = a.y().max(b.y()).max(c.y());

        Aabb2::from_points(Point2::new(min_x, min_y), Point2::new(max_x, max_y))
    }
}

/// Returns twice the signed 2D triangle area using the shoelace formula.
#[must_use]
pub fn triangle_twice_signed_area(a: Point2, b: Point2, c: Point2) -> f64 {
    signed_twice_area_2d(a, b, c)
}

/// Returns twice the unsigned 2D triangle area.
#[must_use]
pub fn triangle_twice_area(a: Point2, b: Point2, c: Point2) -> f64 {
    triangle_twice_signed_area(a, b, c).abs()
}

/// Returns the 2D triangle area.
#[must_use]
pub fn triangle_area(a: Point2, b: Point2, c: Point2) -> f64 {
    triangle_twice_area(a, b, c) * 0.5
}

#[cfg(test)]
mod tests {
    use super::{Triangle, triangle_area, triangle_twice_area, triangle_twice_signed_area};
    use crate::{error::GeometryError, orientation::Orientation2, point::Point2};

    fn approx_eq(left: f64, right: f64) -> bool {
        (left - right).abs() < 1.0e-10
    }

    fn approx_eq_slice(left: [f64; 3], right: [f64; 3]) -> bool {
        left.into_iter()
            .zip(right)
            .all(|(left_value, right_value)| approx_eq(left_value, right_value))
    }

    #[test]
    fn constructs_triangles() {
        let triangle = Triangle::new(
            Point2::new(0.0, 0.0),
            Point2::new(4.0, 0.0),
            Point2::new(0.0, 3.0),
        );

        assert_eq!(triangle.a(), Point2::new(0.0, 0.0));
    }

    #[test]
    fn constructs_triangles_with_try_new() {
        assert_eq!(
            Triangle::try_new(
                Point2::new(0.0, 0.0),
                Point2::new(4.0, 0.0),
                Point2::new(0.0, 3.0),
            ),
            Ok(Triangle::new(
                Point2::new(0.0, 0.0),
                Point2::new(4.0, 0.0),
                Point2::new(0.0, 3.0),
            ))
        );
    }

    #[test]
    fn rejects_non_finite_triangle_vertices() {
        assert!(matches!(
            Triangle::try_new(
                Point2::new(0.0, 0.0),
                Point2::new(4.0, 0.0),
                Point2::new(0.0, f64::NAN),
            ),
            Err(GeometryError::NonFiniteComponent {
                type_name: "Point2",
                component: "y",
                value,
            }) if value.is_nan()
        ));
    }

    #[test]
    fn computes_triangle_area() {
        let triangle = Triangle::new(
            Point2::new(0.0, 0.0),
            Point2::new(4.0, 0.0),
            Point2::new(0.0, 3.0),
        );

        assert!(approx_eq(triangle.twice_signed_area(), 12.0));
        assert!(approx_eq(triangle.twice_area(), 12.0));
        assert!(approx_eq(triangle.area(), 6.0));
        assert!(approx_eq(
            triangle_twice_signed_area(triangle.a(), triangle.b(), triangle.c()),
            12.0
        ));
        assert!(approx_eq(
            triangle_twice_area(triangle.a(), triangle.b(), triangle.c()),
            12.0
        ));
        assert!(approx_eq(
            triangle_area(triangle.a(), triangle.b(), triangle.c()),
            6.0
        ));
    }

    #[test]
    fn signed_area_tracks_orientation() {
        let counter_clockwise = Triangle::new(
            Point2::new(0.0, 0.0),
            Point2::new(4.0, 0.0),
            Point2::new(0.0, 3.0),
        );
        let clockwise = Triangle::new(
            Point2::new(0.0, 0.0),
            Point2::new(0.0, 3.0),
            Point2::new(4.0, 0.0),
        );

        assert!(approx_eq(counter_clockwise.twice_signed_area(), 12.0));
        assert!(approx_eq(clockwise.twice_signed_area(), -12.0));
        assert_eq!(
            counter_clockwise.orientation(),
            Orientation2::CounterClockwise
        );
        assert_eq!(clockwise.orientation(), Orientation2::Clockwise);
        assert_eq!(
            crate::orientation::orientation_2d(
                Point2::new(0.0, 0.0),
                Point2::new(1.0, 1.0),
                Point2::new(2.0, 2.0)
            ),
            Orientation2::Collinear
        );
    }

    #[test]
    fn computes_triangle_perimeter() {
        let triangle = Triangle::new(
            Point2::new(0.0, 0.0),
            Point2::new(4.0, 0.0),
            Point2::new(0.0, 3.0),
        );

        assert!(approx_eq_slice(triangle.sides(), [4.0, 5.0, 3.0]));
        assert!(approx_eq(triangle.perimeter(), 12.0));
        assert_eq!(
            triangle.vertices(),
            [triangle.a(), triangle.b(), triangle.c()]
        );
        assert_eq!(triangle.centroid(), Point2::new(4.0 / 3.0, 1.0));
    }

    #[test]
    fn detects_degenerate_triangles() {
        let triangle = Triangle::new(
            Point2::new(0.0, 0.0),
            Point2::new(1.0, 1.0),
            Point2::new(2.0, 2.0),
        );

        assert!(triangle.is_degenerate());
        assert_eq!(triangle.is_degenerate_with_tolerance(0.0), Ok(true));
    }

    #[test]
    fn detects_near_degenerate_triangles_with_tolerance() {
        let triangle = Triangle::new(
            Point2::new(0.0, 0.0),
            Point2::new(1.0, 1.0),
            Point2::new(2.0, 2.0 + 1.0e-12),
        );

        assert!(!triangle.is_degenerate());
        assert_eq!(triangle.is_degenerate_with_tolerance(1.0e-11), Ok(true));
    }

    #[test]
    fn rejects_negative_degeneracy_tolerance() {
        let triangle = Triangle::new(
            Point2::new(0.0, 0.0),
            Point2::new(1.0, 1.0),
            Point2::new(2.0, 2.0),
        );

        assert_eq!(
            triangle.is_degenerate_with_tolerance(-1.0),
            Err(GeometryError::NegativeTolerance(-1.0))
        );
    }

    #[test]
    fn computes_triangle_bounds() {
        let triangle = Triangle::new(
            Point2::new(4.0, 1.0),
            Point2::new(1.0, 3.0),
            Point2::new(2.0, -1.0),
        );

        assert_eq!(triangle.aabb().min(), Point2::new(1.0, -1.0));
        assert_eq!(triangle.aabb().max(), Point2::new(4.0, 3.0));
    }
}
