#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Const-generic vector primitives and operations for `RustUse`.

use core::num::FpCategory;
use core::ops::{Add, Div, Index, IndexMut, Mul, Neg, Sub};

/// A vector with `N` plain `f64` components.
///
/// # Examples
///
/// ```
/// use use_vector::Vector;
///
/// let vector = Vector::<3>::from_array([2.0, 3.0, 6.0]);
///
/// assert_eq!(vector.dimension(), 3);
/// assert_eq!(vector.magnitude(), 7.0);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector<const N: usize> {
    components: [f64; N],
}

/// A two-dimensional vector.
pub type Vector2 = Vector<2>;

/// A three-dimensional vector.
pub type Vector3 = Vector<3>;

/// A four-dimensional vector.
pub type Vector4 = Vector<4>;

/// Two-dimensional vector primitives and operations.
pub mod vector2 {
    pub use crate::Vector2;
}

/// Three-dimensional vector primitives and operations.
pub mod vector3 {
    pub use crate::Vector3;
}

/// Four-dimensional vector primitives and operations.
pub mod vector4 {
    pub use crate::Vector4;
}

impl<const N: usize> Vector<N> {
    /// The zero vector.
    pub const ZERO: Self = Self {
        components: [0.0; N],
    };

    /// A vector with every component set to `1.0`.
    pub const ONE: Self = Self {
        components: [1.0; N],
    };

    /// Creates a vector from its component array.
    #[must_use]
    pub const fn from_array(components: [f64; N]) -> Self {
        Self { components }
    }

    /// Returns a shared reference to the component array.
    #[must_use]
    pub const fn as_array(&self) -> &[f64; N] {
        &self.components
    }

    /// Returns the component array.
    #[must_use]
    pub const fn into_array(self) -> [f64; N] {
        self.components
    }

    /// Returns the vector dimension.
    #[must_use]
    pub const fn dimension(self) -> usize {
        self.components.len()
    }

    /// Returns the dot product with `other`.
    ///
    /// # Examples
    ///
    /// ```
    /// use use_vector::Vector;
    ///
    /// let a = Vector::<3>::from_array([1.0, 2.0, 3.0]);
    /// let b = Vector::<3>::from_array([4.0, 5.0, 6.0]);
    ///
    /// assert_eq!(a.dot(b), 32.0);
    /// ```
    #[must_use]
    pub fn dot(self, other: Self) -> f64 {
        self.components
            .into_iter()
            .zip(other.components)
            .fold(0.0, |sum, (left, right)| left.mul_add(right, sum))
    }

    /// Returns the squared Euclidean magnitude.
    #[must_use]
    pub fn magnitude_squared(self) -> f64 {
        self.dot(self)
    }

    /// Returns the Euclidean magnitude.
    #[must_use]
    pub fn magnitude(self) -> f64 {
        self.magnitude_squared().sqrt()
    }

    /// Returns a normalized vector when the magnitude is finite and non-zero.
    ///
    /// Returns `None` for zero vectors or vectors with non-finite magnitude.
    ///
    /// # Examples
    ///
    /// ```
    /// use use_vector::Vector2;
    ///
    /// let unit = Vector2::new(3.0, 4.0)
    ///     .normalize()
    ///     .expect("non-zero finite vector should normalize");
    ///
    /// assert!((unit.x() - 0.6).abs() < 1.0e-12);
    /// assert!((unit.y() - 0.8).abs() < 1.0e-12);
    /// ```
    #[must_use]
    pub fn normalize(self) -> Option<Self> {
        let magnitude = self.magnitude();

        if !magnitude.is_finite() || matches!(magnitude.classify(), FpCategory::Zero) {
            return None;
        }

        Some(self / magnitude)
    }

    /// Returns the vector scaled by `scalar`.
    #[must_use]
    pub fn scale(self, scalar: f64) -> Self {
        self * scalar
    }

    /// Returns the Euclidean distance to `other`.
    ///
    /// # Examples
    ///
    /// ```
    /// use use_vector::Vector2;
    ///
    /// let start = Vector2::ZERO;
    /// let end = Vector2::new(3.0, 4.0);
    ///
    /// assert_eq!(start.distance(end), 5.0);
    /// ```
    #[must_use]
    pub fn distance(self, other: Self) -> f64 {
        (other - self).magnitude()
    }

    /// Returns the squared Euclidean distance to `other`.
    #[must_use]
    pub fn distance_squared(self, other: Self) -> f64 {
        (other - self).magnitude_squared()
    }

    /// Returns the linear interpolation between `self` and `other` for `t`.
    #[must_use]
    pub fn lerp(self, other: Self, t: f64) -> Self {
        self + (other - self) * t
    }

    /// Maps each component with `mapper`.
    #[must_use]
    pub fn map_components(self, mut mapper: impl FnMut(f64) -> f64) -> Self {
        let mut components = self.components;

        for component in &mut components {
            *component = mapper(*component);
        }

        Self::from_array(components)
    }

    /// Combines components from `self` and `other` with `mapper`.
    #[must_use]
    pub fn zip_components(self, other: Self, mut mapper: impl FnMut(f64, f64) -> f64) -> Self {
        let mut components = [0.0; N];

        for (index, component) in components.iter_mut().enumerate() {
            *component = mapper(self.components[index], other.components[index]);
        }

        Self::from_array(components)
    }

    /// Returns the component-wise minimum with `other`.
    #[must_use]
    pub fn component_min(self, other: Self) -> Self {
        self.zip_components(other, f64::min)
    }

    /// Returns the component-wise maximum with `other`.
    #[must_use]
    pub fn component_max(self, other: Self) -> Self {
        self.zip_components(other, f64::max)
    }

    /// Returns each component bounded by the matching `minimum` and `maximum` components.
    #[must_use]
    pub fn clamp_components(self, minimum: Self, maximum: Self) -> Self {
        self.zip_components(minimum, f64::max)
            .zip_components(maximum, f64::min)
    }

    /// Returns a vector with the absolute value of each component.
    #[must_use]
    pub fn abs(self) -> Self {
        self.map_components(f64::abs)
    }

    /// Returns `true` when every component is finite.
    #[must_use]
    pub fn is_finite(self) -> bool {
        self.components.into_iter().all(f64::is_finite)
    }

    /// Returns `true` when any component is `NaN`.
    #[must_use]
    pub fn is_nan(self) -> bool {
        self.components.into_iter().any(f64::is_nan)
    }
}

impl Vector<2> {
    /// Creates a two-dimensional vector from its components.
    #[must_use]
    pub const fn new(x: f64, y: f64) -> Self {
        Self::from_array([x, y])
    }

    /// Returns the x component.
    #[must_use]
    pub const fn x(self) -> f64 {
        self.components[0]
    }

    /// Returns the y component.
    #[must_use]
    pub const fn y(self) -> f64 {
        self.components[1]
    }
}

impl Vector<3> {
    /// Creates a three-dimensional vector from its components.
    #[must_use]
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self::from_array([x, y, z])
    }

    /// Returns the x component.
    #[must_use]
    pub const fn x(self) -> f64 {
        self.components[0]
    }

    /// Returns the y component.
    #[must_use]
    pub const fn y(self) -> f64 {
        self.components[1]
    }

    /// Returns the z component.
    #[must_use]
    pub const fn z(self) -> f64 {
        self.components[2]
    }

    /// Returns the cross product with `other`.
    ///
    /// # Examples
    ///
    /// ```
    /// use use_vector::Vector3;
    ///
    /// let x = Vector3::new(1.0, 0.0, 0.0);
    /// let y = Vector3::new(0.0, 1.0, 0.0);
    ///
    /// assert_eq!(x.cross(y), Vector3::new(0.0, 0.0, 1.0));
    /// ```
    #[must_use]
    pub fn cross(self, other: Self) -> Self {
        Self::new(
            mul_sub(self.y(), other.z(), self.z(), other.y()),
            mul_sub(self.z(), other.x(), self.x(), other.z()),
            mul_sub(self.x(), other.y(), self.y(), other.x()),
        )
    }
}

impl Vector<4> {
    /// Creates a four-dimensional vector from its components.
    #[must_use]
    pub const fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self::from_array([x, y, z, w])
    }

    /// Returns the x component.
    #[must_use]
    pub const fn x(self) -> f64 {
        self.components[0]
    }

    /// Returns the y component.
    #[must_use]
    pub const fn y(self) -> f64 {
        self.components[1]
    }

    /// Returns the z component.
    #[must_use]
    pub const fn z(self) -> f64 {
        self.components[2]
    }

    /// Returns the w component.
    #[must_use]
    pub const fn w(self) -> f64 {
        self.components[3]
    }
}

impl<const N: usize> Add for Vector<N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.zip_components(rhs, |left, right| left + right)
    }
}

impl<const N: usize> Sub for Vector<N> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.zip_components(rhs, |left, right| left - right)
    }
}

impl<const N: usize> Mul<f64> for Vector<N> {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        self.map_components(|component| component * rhs)
    }
}

impl<const N: usize> Div<f64> for Vector<N> {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        self.map_components(|component| component / rhs)
    }
}

impl<const N: usize> Neg for Vector<N> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self.map_components(|component| -component)
    }
}

impl<const N: usize> Index<usize> for Vector<N> {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.components[index]
    }
}

impl<const N: usize> IndexMut<usize> for Vector<N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.components[index]
    }
}

impl<const N: usize> AsRef<[f64; N]> for Vector<N> {
    fn as_ref(&self) -> &[f64; N] {
        self.as_array()
    }
}

impl<const N: usize> From<[f64; N]> for Vector<N> {
    fn from(components: [f64; N]) -> Self {
        Self::from_array(components)
    }
}

impl<const N: usize> From<Vector<N>> for [f64; N] {
    fn from(vector: Vector<N>) -> Self {
        vector.into_array()
    }
}

#[inline]
fn mul_sub(a: f64, b: f64, c: f64, d: f64) -> f64 {
    a.mul_add(b, -(c * d))
}

#[cfg(test)]
mod tests {
    use super::{Vector, Vector2, Vector3, Vector4};

    fn approx_eq(left: f64, right: f64) -> bool {
        (left - right).abs() < 1.0e-12
    }

    fn assert_vector_close<const N: usize>(actual: Vector<N>, expected: [f64; N]) {
        let actual_components = actual.into_array();

        for (actual_component, expected_component) in actual_components.iter().zip(expected) {
            assert!(
                approx_eq(*actual_component, expected_component),
                "expected {expected:?}, got {actual_components:?}"
            );
        }
    }

    #[test]
    fn generic_vectors_support_component_workflows() {
        let mut vector = Vector::<3>::from_array([1.0, -2.0, 3.0]);

        assert_eq!(vector.dimension(), 3);
        assert_vector_close(Vector::from_array(*vector.as_array()), [1.0, -2.0, 3.0]);
        assert_vector_close(
            Vector::from_array(<[f64; 3]>::from(vector)),
            [1.0, -2.0, 3.0],
        );
        assert_vector_close(Vector::<3>::from([1.0, 2.0, 3.0]), [1.0, 2.0, 3.0]);
        assert!(approx_eq(vector[0], 1.0));

        vector[1] = 5.0;

        assert_vector_close(Vector::from_array(vector.into_array()), [1.0, 5.0, 3.0]);
        assert_vector_close(Vector::<4>::ZERO, [0.0, 0.0, 0.0, 0.0]);
        assert_vector_close(Vector::<4>::ONE, [1.0, 1.0, 1.0, 1.0]);
    }

    #[test]
    fn generic_vectors_support_arithmetic_and_norms() {
        let a = Vector::<3>::from_array([1.0, 2.0, 3.0]);
        let b = Vector::<3>::from_array([4.0, -2.0, 0.5]);

        assert_vector_close(a + b, [5.0, 0.0, 3.5]);
        assert_vector_close(a - b, [-3.0, 4.0, 2.5]);
        assert_vector_close(a * 2.0, [2.0, 4.0, 6.0]);
        assert_vector_close(a / 2.0, [0.5, 1.0, 1.5]);
        assert_vector_close(-a, [-1.0, -2.0, -3.0]);
        assert!(approx_eq(a.dot(b), 1.5));
        assert!(approx_eq(a.magnitude_squared(), 14.0));
        assert!(approx_eq(a.magnitude(), 14.0_f64.sqrt()));
        assert_vector_close(Vector::<3>::ZERO.lerp(a, 0.25), [0.25, 0.5, 0.75]);
        assert!(approx_eq(Vector::<3>::ZERO.distance(a), a.magnitude()));
        assert!(approx_eq(Vector::<3>::ZERO.distance_squared(a), 14.0));
    }

    #[test]
    fn generic_vectors_support_component_helpers() {
        let a = Vector::<3>::from_array([-1.0, 2.0, 5.0]);
        let b = Vector::<3>::from_array([3.0, -4.0, 4.0]);
        let minimum = Vector::<3>::from_array([0.0, 0.0, 0.0]);
        let maximum = Vector::<3>::from_array([2.0, 3.0, 4.0]);

        assert_vector_close(
            a.map_components(|component| component * 2.0),
            [-2.0, 4.0, 10.0],
        );
        assert_vector_close(
            a.zip_components(b, |left, right| left - right),
            [-4.0, 6.0, 1.0],
        );
        assert_vector_close(a.component_min(b), [-1.0, -4.0, 4.0]);
        assert_vector_close(a.component_max(b), [3.0, 2.0, 5.0]);
        assert_vector_close(a.clamp_components(minimum, maximum), [0.0, 2.0, 4.0]);
        assert_vector_close(a.abs(), [1.0, 2.0, 5.0]);
        assert!(a.is_finite());
        assert!(!Vector::<3>::from_array([f64::INFINITY, 1.0, 2.0]).is_finite());
        assert!(Vector::<3>::from_array([f64::NAN, 1.0, 2.0]).is_nan());
    }

    #[test]
    fn vector2_alias_supports_specialized_construction_and_accessors() {
        let constructed = Vector2::new(3.0, 4.0);

        assert!(approx_eq(constructed.x(), 3.0));
        assert!(approx_eq(constructed.y(), 4.0));
        assert_vector_close(constructed.scale(0.5), [1.5, 2.0]);
        assert!(approx_eq(constructed.dot(Vector2::new(-2.0, 1.0)), -2.0));
        assert!(approx_eq(constructed.magnitude(), 5.0));
        assert!(Vector2::ZERO.normalize().is_none());

        let Some(normalized) = constructed.normalize() else {
            panic!("non-zero finite vector should normalize");
        };

        assert_vector_close(normalized, [0.6, 0.8]);
    }

    #[test]
    fn vector3_alias_supports_cross_product() {
        let constructed = Vector3::new(2.0, 3.0, 6.0);

        assert!(approx_eq(constructed.x(), 2.0));
        assert!(approx_eq(constructed.y(), 3.0));
        assert!(approx_eq(constructed.z(), 6.0));
        assert_vector_close(constructed + Vector3::ONE, [3.0, 4.0, 7.0]);
        assert_vector_close(
            Vector3::new(1.0, 0.0, 0.0).cross(Vector3::new(0.0, 1.0, 0.0)),
            [0.0, 0.0, 1.0],
        );
        assert!(approx_eq(constructed.magnitude(), 7.0));
    }

    #[test]
    fn vector4_alias_supports_specialized_construction_and_accessors() {
        let constructed = Vector4::new(2.0, 4.0, 4.0, 4.0);

        assert!(approx_eq(constructed.x(), 2.0));
        assert!(approx_eq(constructed.y(), 4.0));
        assert!(approx_eq(constructed.z(), 4.0));
        assert!(approx_eq(constructed.w(), 4.0));
        assert_vector_close(constructed + Vector4::ONE, [3.0, 5.0, 5.0, 5.0]);
        assert!(approx_eq(constructed.magnitude_squared(), 52.0));
    }

    #[test]
    fn normalize_rejects_zero_and_non_finite_magnitudes() {
        assert!(Vector2::ZERO.normalize().is_none());
        assert!(Vector2::new(f64::INFINITY, 1.0).normalize().is_none());
        assert!(Vector3::new(f64::NAN, 1.0, 2.0).normalize().is_none());
        assert!(
            Vector4::new(f64::MAX, f64::MAX, f64::MAX, f64::MAX)
                .normalize()
                .is_none()
        );
    }
}
