use core::ops::{Add, Div, Mul, Sub};

use crate::{error::GeometryError, point::Point2};

/// A 2D vector represented with `f64` components.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector2 {
    /// The horizontal component.
    x: f64,
    /// The vertical component.
    y: f64,
}

impl Vector2 {
    /// Creates a vector from `x` and `y` components.
    #[must_use]
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// Returns the horizontal component.
    #[must_use]
    pub const fn x(&self) -> f64 {
        self.x
    }

    /// Returns the vertical component.
    #[must_use]
    pub const fn y(&self) -> f64 {
        self.y
    }

    /// Creates a vector from finite `x` and `y` components.
    ///
    /// # Errors
    ///
    /// Returns [`GeometryError::NonFiniteComponent`] when `x` or `y` is `NaN`
    /// or infinite.
    ///
    /// # Examples
    ///
    /// ```
    /// use use_geometry::{GeometryError, Vector2};
    ///
    /// let vector = Vector2::try_new(1.0, -2.0)?;
    /// assert_eq!(vector, Vector2::new(1.0, -2.0));
    ///
    /// assert!(matches!(
    ///     Vector2::try_new(1.0, f64::INFINITY),
    ///     Err(GeometryError::NonFiniteComponent { component: "y", .. })
    /// ));
    /// # Ok::<(), GeometryError>(())
    /// ```
    pub const fn try_new(x: f64, y: f64) -> Result<Self, GeometryError> {
        if !x.is_finite() {
            return Err(GeometryError::non_finite_component("Vector2", "x", x));
        }

        if !y.is_finite() {
            return Err(GeometryError::non_finite_component("Vector2", "y", y));
        }

        Ok(Self::new(x, y))
    }

    /// Validates that an existing vector contains only finite components.
    ///
    /// # Errors
    ///
    /// Returns [`GeometryError::NonFiniteComponent`] when `self.x` or
    /// `self.y` is `NaN` or infinite.
    ///
    /// # Examples
    ///
    /// ```
    /// use use_geometry::{GeometryError, Vector2};
    ///
    /// let validated = Vector2::new(3.0, 4.0).validate()?;
    /// assert_eq!(validated, Vector2::new(3.0, 4.0));
    /// # Ok::<(), GeometryError>(())
    /// ```
    pub const fn validate(self) -> Result<Self, GeometryError> {
        Self::try_new(self.x, self.y)
    }

    /// Returns `true` when both components are finite.
    #[must_use]
    pub const fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite()
    }

    /// Returns the zero vector.
    #[must_use]
    pub const fn zero() -> Self {
        Self::new(0.0, 0.0)
    }

    /// Returns the vector from point `a` to point `b`.
    ///
    /// # Examples
    ///
    /// ```
    /// use use_geometry::{Point2, Vector2};
    ///
    /// let start = Point2::new(1.0, 2.0);
    /// let end = Point2::new(4.0, 6.0);
    ///
    /// assert_eq!(Vector2::from_points(start, end), Vector2::new(3.0, 4.0));
    /// ```
    #[must_use]
    pub const fn from_points(a: Point2, b: Point2) -> Self {
        Self::new(b.x() - a.x(), b.y() - a.y())
    }

    /// Returns the vector from point `a` to point `b` when both points are finite.
    ///
    /// # Errors
    ///
    /// Returns [`GeometryError::NonFiniteComponent`] when either point contains
    /// a non-finite coordinate or the resulting vector contains a non-finite
    /// component.
    ///
    /// # Examples
    ///
    /// ```
    /// use use_geometry::{GeometryError, Point2, Vector2};
    ///
    /// let vector = Vector2::try_from_points(Point2::new(1.0, 2.0), Point2::new(4.0, 6.0))?;
    /// assert_eq!(vector, Vector2::new(3.0, 4.0));
    /// # Ok::<(), GeometryError>(())
    /// ```
    pub fn try_from_points(a: Point2, b: Point2) -> Result<Self, GeometryError> {
        let a = a.validate()?;
        let b = b.validate()?;

        Self::from_points(a, b).validate()
    }

    /// Returns the vector magnitude.
    ///
    /// # Examples
    ///
    /// ```
    /// use use_geometry::Vector2;
    ///
    /// let vector = Vector2::new(3.0, 4.0);
    /// assert_eq!(vector.magnitude(), 5.0);
    /// ```
    #[must_use]
    pub fn magnitude(self) -> f64 {
        self.x.hypot(self.y)
    }

    /// Returns the vector length.
    ///
    /// # Examples
    ///
    /// ```
    /// use use_geometry::Vector2;
    ///
    /// let vector = Vector2::new(5.0, 12.0);
    /// assert_eq!(vector.length(), 13.0);
    /// ```
    #[must_use]
    pub fn length(self) -> f64 {
        self.magnitude()
    }

    /// Returns the squared vector magnitude.
    ///
    /// # Examples
    ///
    /// ```
    /// use use_geometry::Vector2;
    ///
    /// let vector = Vector2::new(5.0, 12.0);
    /// assert_eq!(vector.magnitude_squared(), 169.0);
    /// ```
    #[must_use]
    pub fn magnitude_squared(self) -> f64 {
        self.x.mul_add(self.x, self.y * self.y)
    }

    /// Returns the squared vector length.
    ///
    /// # Examples
    ///
    /// ```
    /// use use_geometry::Vector2;
    ///
    /// let vector = Vector2::new(5.0, 12.0);
    /// assert_eq!(vector.length_squared(), 169.0);
    /// ```
    #[must_use]
    pub fn length_squared(self) -> f64 {
        self.magnitude_squared()
    }

    /// Returns the dot product with another vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use use_geometry::Vector2;
    ///
    /// let left = Vector2::new(1.0, 3.0);
    /// let right = Vector2::new(2.0, 4.0);
    ///
    /// assert_eq!(left.dot(right), 14.0);
    /// ```
    #[must_use]
    pub fn dot(self, other: Self) -> f64 {
        self.x.mul_add(other.x, self.y * other.y)
    }

    /// Returns the scalar z-component of the 2D cross product.
    ///
    /// # Examples
    ///
    /// ```
    /// use use_geometry::Vector2;
    ///
    /// let x_axis = Vector2::new(1.0, 0.0);
    /// let y_axis = Vector2::new(0.0, 1.0);
    ///
    /// assert_eq!(x_axis.cross(y_axis), 1.0);
    /// ```
    #[must_use]
    pub fn cross(self, other: Self) -> f64 {
        self.x.mul_add(other.y, -(self.y * other.x))
    }

    /// Returns a scaled vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use use_geometry::Vector2;
    ///
    /// let vector = Vector2::new(2.0, -3.0);
    /// assert_eq!(vector.scale(0.5), Vector2::new(1.0, -1.5));
    /// ```
    #[must_use]
    pub const fn scale(self, factor: f64) -> Self {
        Self::new(self.x * factor, self.y * factor)
    }

    /// Returns a unit-length vector when normalization succeeds.
    ///
    /// Returns `None` for the zero vector and for vectors whose length is not
    /// finite.
    ///
    /// # Examples
    ///
    /// ```
    /// use use_geometry::Vector2;
    ///
    /// let unit = Vector2::new(3.0, 4.0).try_normalize().expect("unit vector");
    ///
    /// assert!((unit.length() - 1.0).abs() < 1.0e-10);
    /// assert!(Vector2::zero().try_normalize().is_none());
    /// ```
    #[must_use]
    pub fn try_normalize(self) -> Option<Self> {
        let length = self.length();

        if length == 0.0 || !length.is_finite() {
            None
        } else {
            Some(self / length)
        }
    }

    /// Returns a unit-length vector, or zero when normalization fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use use_geometry::Vector2;
    ///
    /// let unit = Vector2::new(3.0, 4.0).normalize_or_zero();
    ///
    /// assert!((unit.length() - 1.0).abs() < 1.0e-10);
    /// assert_eq!(Vector2::zero().normalize_or_zero(), Vector2::zero());
    /// ```
    #[must_use]
    pub fn normalize_or_zero(self) -> Self {
        self.try_normalize().unwrap_or_else(Self::zero)
    }
}

/// Returns the dot product of two vectors.
#[must_use]
pub fn dot(left: Vector2, right: Vector2) -> f64 {
    left.dot(right)
}

/// Returns the 2D cross product magnitude of two vectors.
#[must_use]
pub fn cross(left: Vector2, right: Vector2) -> f64 {
    left.cross(right)
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

impl Mul<f64> for Vector2 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        self.scale(rhs)
    }
}

impl Div<f64> for Vector2 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::{Vector2, cross, dot};
    use crate::{error::GeometryError, point::Point2};

    fn approx_eq(left: f64, right: f64) -> bool {
        (left - right).abs() < 1.0e-10
    }

    #[test]
    fn constructs_vectors() {
        assert_eq!(
            Vector2::new(1.0, -2.0),
            Vector2::try_new(1.0, -2.0).expect("valid vector")
        );
    }

    #[test]
    fn constructs_vectors_with_try_new() {
        assert_eq!(Vector2::try_new(1.0, -2.0), Ok(Vector2::new(1.0, -2.0)));
    }

    #[test]
    fn rejects_non_finite_vector_components() {
        assert_eq!(
            Vector2::try_new(1.0, f64::NEG_INFINITY),
            Err(GeometryError::NonFiniteComponent {
                type_name: "Vector2",
                component: "y",
                value: f64::NEG_INFINITY,
            })
        );
    }

    #[test]
    fn returns_zero_vector() {
        assert_eq!(Vector2::zero(), Vector2::new(0.0, 0.0));
    }

    #[test]
    fn computes_magnitudes() {
        let vector = Vector2::new(3.0, 4.0);

        assert!(approx_eq(vector.magnitude(), 5.0));
        assert!(approx_eq(vector.magnitude_squared(), 25.0));
        assert!(approx_eq(vector.length(), 5.0));
        assert!(approx_eq(vector.length_squared(), 25.0));
    }

    #[test]
    fn constructs_vectors_from_points() {
        let start = Point2::new(1.0, 2.0);
        let end = Point2::new(4.0, 6.0);

        assert_eq!(Vector2::from_points(start, end), Vector2::new(3.0, 4.0));
    }

    #[test]
    fn constructs_vectors_from_finite_points() {
        let start = Point2::new(1.0, 2.0);
        let end = Point2::new(4.0, 6.0);

        assert_eq!(
            Vector2::try_from_points(start, end),
            Ok(Vector2::new(3.0, 4.0))
        );
    }

    #[test]
    fn rejects_vectors_from_non_finite_points() {
        assert!(matches!(
            Vector2::try_from_points(Point2::new(f64::NAN, 0.0), Point2::new(1.0, 1.0)),
            Err(GeometryError::NonFiniteComponent {
                type_name: "Point2",
                component: "x",
                value,
            }) if value.is_nan()
        ));
    }

    #[test]
    fn computes_dot_products() {
        let left = Vector2::new(1.0, 2.0);
        let right = Vector2::new(3.0, 4.0);

        assert!(approx_eq(left.dot(right), 11.0));
        assert!(approx_eq(dot(left, right), 11.0));
    }

    #[test]
    fn computes_cross_products() {
        let left = Vector2::new(1.0, 2.0);
        let right = Vector2::new(3.0, 4.0);

        assert!(approx_eq(cross(left, right), -2.0));
    }

    #[test]
    fn scales_vectors() {
        let vector = Vector2::new(1.5, -2.0);

        assert_eq!(vector.scale(2.0), Vector2::new(3.0, -4.0));
        assert_eq!(vector * 2.0, Vector2::new(3.0, -4.0));
        assert_eq!(vector / 2.0, Vector2::new(0.75, -1.0));
    }

    #[test]
    fn adds_and_subtracts_vectors() {
        let left = Vector2::new(2.0, 5.0);
        let right = Vector2::new(-1.0, 3.0);

        assert_eq!(left + right, Vector2::new(1.0, 8.0));
        assert_eq!(left - right, Vector2::new(3.0, 2.0));
    }

    #[test]
    fn exposes_accessors_and_finite_checks() {
        let vector = Vector2::new(1.5, -2.0);

        assert!(approx_eq(vector.x(), 1.5));
        assert!(approx_eq(vector.y(), -2.0));
        assert!(vector.is_finite());
        assert!(!Vector2::new(0.0, f64::NEG_INFINITY).is_finite());
    }

    #[test]
    fn normalizes_vectors() {
        let vector = Vector2::new(3.0, 4.0);

        assert_eq!(vector.try_normalize(), Some(Vector2::new(0.6, 0.8)));
        assert_eq!(Vector2::zero().try_normalize(), None);
        assert_eq!(Vector2::zero().normalize_or_zero(), Vector2::zero());
    }
}
