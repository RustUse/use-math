#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Small vector primitives and operations for `RustUse`.

#[doc(inline)]
pub use vector2::Vector2;
#[doc(inline)]
pub use vector3::Vector3;
#[doc(inline)]
pub use vector4::Vector4;

/// Two-dimensional vector primitives and operations.
pub mod vector2 {
    use core::ops::{Add, Div, Mul, Neg, Sub};

    #[inline]
    fn dot2(a0: f64, b0: f64, a1: f64, b1: f64) -> f64 {
        a0.mul_add(b0, a1 * b1)
    }

    /// A two-dimensional vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use use_vector::Vector2;
    ///
    /// let vector = Vector2::new(3.0, 4.0);
    ///
    /// assert_eq!(vector.magnitude(), 5.0);
    /// ```
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Vector2 {
        /// The x component.
        pub x: f64,
        /// The y component.
        pub y: f64,
    }

    impl Vector2 {
        /// The zero vector.
        pub const ZERO: Self = Self { x: 0.0, y: 0.0 };

        /// A vector with every component set to `1.0`.
        pub const ONE: Self = Self { x: 1.0, y: 1.0 };

        /// Creates a vector from its components.
        #[must_use]
        pub const fn new(x: f64, y: f64) -> Self {
            Self { x, y }
        }

        /// Returns the dot product with `other`.
        ///
        /// # Examples
        ///
        /// ```
        /// use use_vector::Vector2;
        ///
        /// let a = Vector2::new(1.0, 2.0);
        /// let b = Vector2::new(3.0, 4.0);
        ///
        /// assert_eq!(a.dot(b), 11.0);
        /// ```
        #[must_use]
        pub fn dot(self, other: Self) -> f64 {
            dot2(self.x, other.x, self.y, other.y)
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
        /// assert!((unit.x - 0.6).abs() < 1.0e-12);
        /// assert!((unit.y - 0.8).abs() < 1.0e-12);
        /// ```
        #[must_use]
        pub fn normalize(self) -> Option<Self> {
            let magnitude = self.magnitude();

            if magnitude == 0.0 || !magnitude.is_finite() {
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
            Self::new(self.x * rhs, self.y * rhs)
        }
    }

    impl Div<f64> for Vector2 {
        type Output = Self;

        fn div(self, rhs: f64) -> Self::Output {
            Self::new(self.x / rhs, self.y / rhs)
        }
    }

    impl Neg for Vector2 {
        type Output = Self;

        fn neg(self) -> Self::Output {
            Self::new(-self.x, -self.y)
        }
    }
}

/// Three-dimensional vector primitives and operations.
pub mod vector3 {
    use core::ops::{Add, Div, Mul, Neg, Sub};

    #[inline]
    fn dot3(a0: f64, b0: f64, a1: f64, b1: f64, a2: f64, b2: f64) -> f64 {
        a0.mul_add(b0, a1.mul_add(b1, a2 * b2))
    }

    #[inline]
    fn mul_sub(a: f64, b: f64, c: f64, d: f64) -> f64 {
        a.mul_add(b, -(c * d))
    }

    /// A three-dimensional vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use use_vector::Vector3;
    ///
    /// let vector = Vector3::new(2.0, 3.0, 6.0);
    ///
    /// assert_eq!(vector.magnitude(), 7.0);
    /// ```
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Vector3 {
        /// The x component.
        pub x: f64,
        /// The y component.
        pub y: f64,
        /// The z component.
        pub z: f64,
    }

    impl Vector3 {
        /// The zero vector.
        pub const ZERO: Self = Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };

        /// A vector with every component set to `1.0`.
        pub const ONE: Self = Self {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };

        /// Creates a vector from its components.
        #[must_use]
        pub const fn new(x: f64, y: f64, z: f64) -> Self {
            Self { x, y, z }
        }

        /// Returns the dot product with `other`.
        #[must_use]
        pub fn dot(self, other: Self) -> f64 {
            dot3(self.x, other.x, self.y, other.y, self.z, other.z)
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
        #[must_use]
        pub fn normalize(self) -> Option<Self> {
            let magnitude = self.magnitude();

            if magnitude == 0.0 || !magnitude.is_finite() {
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
                mul_sub(self.y, other.z, self.z, other.y),
                mul_sub(self.z, other.x, self.x, other.z),
                mul_sub(self.x, other.y, self.y, other.x),
            )
        }
    }

    impl Add for Vector3 {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
        }
    }

    impl Sub for Vector3 {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self::Output {
            Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
        }
    }

    impl Mul<f64> for Vector3 {
        type Output = Self;

        fn mul(self, rhs: f64) -> Self::Output {
            Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
        }
    }

    impl Div<f64> for Vector3 {
        type Output = Self;

        fn div(self, rhs: f64) -> Self::Output {
            Self::new(self.x / rhs, self.y / rhs, self.z / rhs)
        }
    }

    impl Neg for Vector3 {
        type Output = Self;

        fn neg(self) -> Self::Output {
            Self::new(-self.x, -self.y, -self.z)
        }
    }
}

/// Four-dimensional vector primitives and operations.
pub mod vector4 {
    use core::ops::{Add, Div, Mul, Neg, Sub};

    #[inline]
    fn dot4(a0: f64, b0: f64, a1: f64, b1: f64, a2: f64, b2: f64, a3: f64, b3: f64) -> f64 {
        a0.mul_add(b0, a1.mul_add(b1, a2.mul_add(b2, a3 * b3)))
    }

    /// A four-dimensional vector.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Vector4 {
        /// The x component.
        pub x: f64,
        /// The y component.
        pub y: f64,
        /// The z component.
        pub z: f64,
        /// The w component.
        pub w: f64,
    }

    impl Vector4 {
        /// The zero vector.
        pub const ZERO: Self = Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        };

        /// A vector with every component set to `1.0`.
        pub const ONE: Self = Self {
            x: 1.0,
            y: 1.0,
            z: 1.0,
            w: 1.0,
        };

        /// Creates a vector from its components.
        #[must_use]
        pub const fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
            Self { x, y, z, w }
        }

        /// Returns the dot product with `other`.
        #[must_use]
        pub fn dot(self, other: Self) -> f64 {
            dot4(
                self.x, other.x, self.y, other.y, self.z, other.z, self.w, other.w,
            )
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
        #[must_use]
        pub fn normalize(self) -> Option<Self> {
            let magnitude = self.magnitude();

            if magnitude == 0.0 || !magnitude.is_finite() {
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
    }

    impl Add for Vector4 {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            Self::new(
                self.x + rhs.x,
                self.y + rhs.y,
                self.z + rhs.z,
                self.w + rhs.w,
            )
        }
    }

    impl Sub for Vector4 {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self::Output {
            Self::new(
                self.x - rhs.x,
                self.y - rhs.y,
                self.z - rhs.z,
                self.w - rhs.w,
            )
        }
    }

    impl Mul<f64> for Vector4 {
        type Output = Self;

        fn mul(self, rhs: f64) -> Self::Output {
            Self::new(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs)
        }
    }

    impl Div<f64> for Vector4 {
        type Output = Self;

        fn div(self, rhs: f64) -> Self::Output {
            Self::new(self.x / rhs, self.y / rhs, self.z / rhs, self.w / rhs)
        }
    }

    impl Neg for Vector4 {
        type Output = Self;

        fn neg(self) -> Self::Output {
            Self::new(-self.x, -self.y, -self.z, -self.w)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Vector2, Vector3, Vector4};

    fn approx_eq(left: f64, right: f64) -> bool {
        (left - right).abs() < 1.0e-12
    }

    fn assert_vector2_close(actual: Vector2, expected: Vector2) {
        assert!(
            approx_eq(actual.x, expected.x) && approx_eq(actual.y, expected.y),
            "expected {expected:?}, got {actual:?}"
        );
    }

    fn assert_vector3_close(actual: Vector3, expected: Vector3) {
        assert!(
            approx_eq(actual.x, expected.x)
                && approx_eq(actual.y, expected.y)
                && approx_eq(actual.z, expected.z),
            "expected {expected:?}, got {actual:?}"
        );
    }

    fn assert_vector4_close(actual: Vector4, expected: Vector4) {
        assert!(
            approx_eq(actual.x, expected.x)
                && approx_eq(actual.y, expected.y)
                && approx_eq(actual.z, expected.z)
                && approx_eq(actual.w, expected.w),
            "expected {expected:?}, got {actual:?}"
        );
    }

    #[test]
    fn vector2_supports_core_operations() {
        let constructed = Vector2::new(3.0, 4.0);

        assert_vector2_close(constructed, Vector2 { x: 3.0, y: 4.0 });
        assert_vector2_close(Vector2::ZERO, Vector2::new(0.0, 0.0));
        assert_vector2_close(Vector2::ONE, Vector2::new(1.0, 1.0));
        assert_vector2_close(constructed + Vector2::ONE, Vector2::new(4.0, 5.0));
        assert_vector2_close(constructed - Vector2::ONE, Vector2::new(2.0, 3.0));
        assert_vector2_close(constructed * 2.0, Vector2::new(6.0, 8.0));
        assert_vector2_close(constructed / 2.0, Vector2::new(1.5, 2.0));
        assert_vector2_close(-constructed, Vector2::new(-3.0, -4.0));
        assert!(approx_eq(constructed.dot(Vector2::new(-2.0, 1.0)), -2.0));
        assert!(approx_eq(constructed.magnitude_squared(), 25.0));
        assert!(approx_eq(constructed.magnitude(), 5.0));
        assert_vector2_close(constructed.scale(0.5), Vector2::new(1.5, 2.0));
        assert!(approx_eq(Vector2::ZERO.distance(constructed), 5.0));
        assert!(approx_eq(Vector2::ZERO.distance_squared(constructed), 25.0));
        assert_vector2_close(
            Vector2::ZERO.lerp(Vector2::new(8.0, 4.0), 0.25),
            Vector2::new(2.0, 1.0),
        );
        assert!(Vector2::ZERO.normalize().is_none());

        let normalized = constructed
            .normalize()
            .expect("non-zero finite vector should normalize");

        assert_vector2_close(normalized, Vector2::new(0.6, 0.8));
    }

    #[test]
    fn vector3_supports_core_operations() {
        let constructed = Vector3::new(2.0, 3.0, 6.0);

        assert_vector3_close(
            constructed,
            Vector3 {
                x: 2.0,
                y: 3.0,
                z: 6.0,
            },
        );
        assert_vector3_close(Vector3::ZERO, Vector3::new(0.0, 0.0, 0.0));
        assert_vector3_close(Vector3::ONE, Vector3::new(1.0, 1.0, 1.0));
        assert_vector3_close(constructed + Vector3::ONE, Vector3::new(3.0, 4.0, 7.0));
        assert_vector3_close(constructed - Vector3::ONE, Vector3::new(1.0, 2.0, 5.0));
        assert_vector3_close(constructed * 2.0, Vector3::new(4.0, 6.0, 12.0));
        assert_vector3_close(constructed / 2.0, Vector3::new(1.0, 1.5, 3.0));
        assert_vector3_close(-constructed, Vector3::new(-2.0, -3.0, -6.0));
        assert!(approx_eq(
            constructed.dot(Vector3::new(-1.0, 2.0, 0.5)),
            7.0
        ));
        assert_vector3_close(
            Vector3::new(1.0, 0.0, 0.0).cross(Vector3::new(0.0, 1.0, 0.0)),
            Vector3::new(0.0, 0.0, 1.0),
        );
        assert!(approx_eq(constructed.magnitude_squared(), 49.0));
        assert!(approx_eq(constructed.magnitude(), 7.0));
        assert_vector3_close(constructed.scale(0.5), Vector3::new(1.0, 1.5, 3.0));
        assert!(approx_eq(Vector3::ZERO.distance(constructed), 7.0));
        assert!(approx_eq(Vector3::ZERO.distance_squared(constructed), 49.0));
        assert_vector3_close(
            Vector3::ZERO.lerp(Vector3::new(4.0, 8.0, 12.0), 0.25),
            Vector3::new(1.0, 2.0, 3.0),
        );
        assert!(Vector3::ZERO.normalize().is_none());

        let normalized = Vector3::new(0.0, 3.0, 4.0)
            .normalize()
            .expect("non-zero finite vector should normalize");

        assert_vector3_close(normalized, Vector3::new(0.0, 0.6, 0.8));
    }

    #[test]
    fn vector4_supports_core_operations() {
        let constructed = Vector4::new(2.0, 4.0, 4.0, 4.0);

        assert_vector4_close(
            constructed,
            Vector4 {
                x: 2.0,
                y: 4.0,
                z: 4.0,
                w: 4.0,
            },
        );
        assert_vector4_close(Vector4::ZERO, Vector4::new(0.0, 0.0, 0.0, 0.0));
        assert_vector4_close(Vector4::ONE, Vector4::new(1.0, 1.0, 1.0, 1.0));
        assert_vector4_close(constructed + Vector4::ONE, Vector4::new(3.0, 5.0, 5.0, 5.0));
        assert_vector4_close(constructed - Vector4::ONE, Vector4::new(1.0, 3.0, 3.0, 3.0));
        assert_vector4_close(constructed * 2.0, Vector4::new(4.0, 8.0, 8.0, 8.0));
        assert_vector4_close(constructed / 2.0, Vector4::new(1.0, 2.0, 2.0, 2.0));
        assert_vector4_close(-constructed, Vector4::new(-2.0, -4.0, -4.0, -4.0));
        assert!(approx_eq(
            constructed.dot(Vector4::new(1.0, 0.5, 0.5, 0.5)),
            8.0
        ));
        assert!(approx_eq(constructed.magnitude_squared(), 52.0));
        assert!(approx_eq(constructed.magnitude(), 52.0_f64.sqrt()));
        assert_vector4_close(constructed.scale(0.5), Vector4::new(1.0, 2.0, 2.0, 2.0));
        assert!(approx_eq(
            Vector4::ZERO.distance(Vector4::new(4.0, 4.0, 4.0, 4.0)),
            8.0
        ));
        assert!(approx_eq(
            Vector4::ZERO.distance_squared(Vector4::new(4.0, 4.0, 4.0, 4.0)),
            64.0,
        ));
        assert_vector4_close(
            Vector4::ZERO.lerp(Vector4::new(4.0, 8.0, 12.0, 16.0), 0.25),
            Vector4::new(1.0, 2.0, 3.0, 4.0),
        );
        assert!(Vector4::ZERO.normalize().is_none());

        let normalized = Vector4::new(2.0, 0.0, 0.0, 0.0)
            .normalize()
            .expect("non-zero finite vector should normalize");

        assert_vector4_close(normalized, Vector4::new(1.0, 0.0, 0.0, 0.0));
    }

    #[test]
    fn normalize_rejects_non_finite_magnitudes() {
        assert!(Vector2::new(f64::INFINITY, 1.0).normalize().is_none());
        assert!(Vector3::new(f64::NAN, 1.0, 2.0).normalize().is_none());
        assert!(
            Vector4::new(f64::MAX, f64::MAX, f64::MAX, f64::MAX)
                .normalize()
                .is_none()
        );
    }
}
