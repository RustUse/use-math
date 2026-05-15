#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Small matrix primitives and operations for `RustUse`.

#[doc(inline)]
pub use matrix2::Matrix2;
#[doc(inline)]
pub use matrix3::Matrix3;
#[doc(inline)]
pub use matrix4::Matrix4;

/// Two-dimensional matrix primitives and operations.
pub mod matrix2 {
    use core::num::FpCategory;
    use core::ops::{Add, Div, Mul, Neg, Sub};

    use use_vector::Vector2;

    #[inline]
    fn dot2(a0: f64, b0: f64, a1: f64, b1: f64) -> f64 {
        a0.mul_add(b0, a1 * b1)
    }

    /// A 2x2 matrix stored in row-major order.
    ///
    /// # Examples
    ///
    /// ```
    /// use use_matrix::Matrix2;
    ///
    /// let matrix = Matrix2::new(
    ///     1.0, 2.0,
    ///     3.0, 4.0,
    /// );
    ///
    /// assert_eq!(matrix.trace(), 5.0);
    /// assert_eq!(matrix.determinant(), -2.0);
    /// ```
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Matrix2 {
        pub m00: f64,
        pub m01: f64,
        pub m10: f64,
        pub m11: f64,
    }

    impl Matrix2 {
        /// The zero matrix.
        pub const ZERO: Self = Self::new(0.0, 0.0, 0.0, 0.0);

        /// The identity matrix.
        ///
        /// # Examples
        ///
        /// ```
        /// use use_matrix::Matrix2;
        ///
        /// let matrix = Matrix2::new(
        ///     4.0, 7.0,
        ///     2.0, 6.0,
        /// );
        ///
        /// assert_eq!(Matrix2::IDENTITY * matrix, matrix);
        /// assert_eq!(matrix * Matrix2::IDENTITY, matrix);
        /// ```
        pub const IDENTITY: Self = Self::new(1.0, 0.0, 0.0, 1.0);

        /// Creates a matrix from row-major entries.
        #[must_use]
        #[allow(clippy::similar_names, clippy::too_many_arguments)]
        pub const fn new(m00: f64, m01: f64, m10: f64, m11: f64) -> Self {
            Self { m00, m01, m10, m11 }
        }

        /// Creates a matrix from row arrays.
        #[must_use]
        pub const fn from_rows(rows: [[f64; 2]; 2]) -> Self {
            Self::new(rows[0][0], rows[0][1], rows[1][0], rows[1][1])
        }

        /// Creates a matrix from column arrays.
        #[must_use]
        pub const fn from_cols(cols: [[f64; 2]; 2]) -> Self {
            Self::new(cols[0][0], cols[1][0], cols[0][1], cols[1][1])
        }

        /// Returns the transpose.
        #[must_use]
        pub const fn transpose(self) -> Self {
            Self::new(self.m00, self.m10, self.m01, self.m11)
        }

        /// Returns the determinant.
        #[must_use]
        pub const fn determinant(self) -> f64 {
            (self.m00 * self.m11) - (self.m01 * self.m10)
        }

        /// Returns the trace.
        #[must_use]
        pub const fn trace(self) -> f64 {
            self.m00 + self.m11
        }

        /// Returns the inverse when the determinant is finite and non-zero.
        ///
        /// Returns `None` for singular matrices or matrices with non-finite
        /// determinants.
        ///
        /// # Examples
        ///
        /// ```
        /// use use_matrix::Matrix2;
        ///
        /// let matrix = Matrix2::new(
        ///     4.0, 7.0,
        ///     2.0, 6.0,
        /// );
        ///
        /// let inverse = matrix.inverse().expect("matrix should invert");
        ///
        /// assert!((inverse.m00 - 0.6).abs() < 1.0e-12);
        /// assert!((inverse.m11 - 0.4).abs() < 1.0e-12);
        /// ```
        #[must_use]
        pub fn inverse(self) -> Option<Self> {
            let determinant = self.determinant();

            if !determinant.is_finite() || matches!(determinant.classify(), FpCategory::Zero) {
                return None;
            }

            Some(Self::new(
                self.m11 / determinant,
                -self.m01 / determinant,
                -self.m10 / determinant,
                self.m00 / determinant,
            ))
        }
    }

    impl Add for Matrix2 {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            Self::new(
                self.m00 + rhs.m00,
                self.m01 + rhs.m01,
                self.m10 + rhs.m10,
                self.m11 + rhs.m11,
            )
        }
    }

    impl Sub for Matrix2 {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self::Output {
            Self::new(
                self.m00 - rhs.m00,
                self.m01 - rhs.m01,
                self.m10 - rhs.m10,
                self.m11 - rhs.m11,
            )
        }
    }

    impl Mul<f64> for Matrix2 {
        type Output = Self;

        fn mul(self, rhs: f64) -> Self::Output {
            Self::new(
                self.m00 * rhs,
                self.m01 * rhs,
                self.m10 * rhs,
                self.m11 * rhs,
            )
        }
    }

    impl Div<f64> for Matrix2 {
        type Output = Self;

        fn div(self, rhs: f64) -> Self::Output {
            Self::new(
                self.m00 / rhs,
                self.m01 / rhs,
                self.m10 / rhs,
                self.m11 / rhs,
            )
        }
    }

    impl Neg for Matrix2 {
        type Output = Self;

        fn neg(self) -> Self::Output {
            Self::new(-self.m00, -self.m01, -self.m10, -self.m11)
        }
    }

    impl Mul for Matrix2 {
        type Output = Self;

        fn mul(self, rhs: Self) -> Self::Output {
            Self::new(
                dot2(self.m00, rhs.m00, self.m01, rhs.m10),
                dot2(self.m00, rhs.m01, self.m01, rhs.m11),
                dot2(self.m10, rhs.m00, self.m11, rhs.m10),
                dot2(self.m10, rhs.m01, self.m11, rhs.m11),
            )
        }
    }

    impl Mul<Vector2> for Matrix2 {
        type Output = Vector2;

        fn mul(self, rhs: Vector2) -> Self::Output {
            Vector2::new(
                dot2(self.m00, rhs.x, self.m01, rhs.y),
                dot2(self.m10, rhs.x, self.m11, rhs.y),
            )
        }
    }
}

/// Three-dimensional matrix primitives and operations.
pub mod matrix3 {
    use core::num::FpCategory;
    use core::ops::{Add, Div, Mul, Neg, Sub};

    use use_vector::Vector3;

    #[inline]
    fn dot3(a0: f64, b0: f64, a1: f64, b1: f64, a2: f64, b2: f64) -> f64 {
        a0.mul_add(b0, a1.mul_add(b1, a2 * b2))
    }

    #[inline]
    fn mul_sub(a: f64, b: f64, c: f64, d: f64) -> f64 {
        a.mul_add(b, -(c * d))
    }

    /// A 3x3 matrix stored in row-major order.
    ///
    /// # Examples
    ///
    /// ```
    /// use use_matrix::Matrix3;
    ///
    /// let matrix = Matrix3::new(
    ///     1.0, 2.0, 3.0,
    ///     0.0, 1.0, 4.0,
    ///     5.0, 6.0, 0.0,
    /// );
    ///
    /// assert_eq!(matrix.determinant(), 1.0);
    /// ```
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Matrix3 {
        pub m00: f64,
        pub m01: f64,
        pub m02: f64,
        pub m10: f64,
        pub m11: f64,
        pub m12: f64,
        pub m20: f64,
        pub m21: f64,
        pub m22: f64,
    }

    impl Matrix3 {
        /// The zero matrix.
        pub const ZERO: Self = Self::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);

        /// The identity matrix.
        ///
        /// # Examples
        ///
        /// ```
        /// use use_matrix::Matrix3;
        ///
        /// let matrix = Matrix3::new(
        ///     2.0, 1.0, 0.0,
        ///     0.0, 3.0, 4.0,
        ///     0.0, 0.0, 5.0,
        /// );
        ///
        /// assert_eq!(Matrix3::IDENTITY * matrix, matrix);
        /// ```
        pub const IDENTITY: Self = Self::new(1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0);

        /// Creates a matrix from row-major entries.
        #[must_use]
        #[allow(clippy::similar_names, clippy::too_many_arguments)]
        pub const fn new(
            m00: f64,
            m01: f64,
            m02: f64,
            m10: f64,
            m11: f64,
            m12: f64,
            m20: f64,
            m21: f64,
            m22: f64,
        ) -> Self {
            Self {
                m00,
                m01,
                m02,
                m10,
                m11,
                m12,
                m20,
                m21,
                m22,
            }
        }

        /// Creates a matrix from row arrays.
        #[must_use]
        pub const fn from_rows(rows: [[f64; 3]; 3]) -> Self {
            Self::new(
                rows[0][0], rows[0][1], rows[0][2], rows[1][0], rows[1][1], rows[1][2], rows[2][0],
                rows[2][1], rows[2][2],
            )
        }

        /// Creates a matrix from column arrays.
        #[must_use]
        pub const fn from_cols(cols: [[f64; 3]; 3]) -> Self {
            Self::new(
                cols[0][0], cols[1][0], cols[2][0], cols[0][1], cols[1][1], cols[2][1], cols[0][2],
                cols[1][2], cols[2][2],
            )
        }

        /// Returns the transpose.
        #[must_use]
        pub const fn transpose(self) -> Self {
            Self::new(
                self.m00, self.m10, self.m20, self.m01, self.m11, self.m21, self.m02, self.m12,
                self.m22,
            )
        }

        /// Returns the determinant.
        #[must_use]
        pub const fn determinant(self) -> f64 {
            self.m00 * ((self.m11 * self.m22) - (self.m12 * self.m21))
                - self.m01 * ((self.m10 * self.m22) - (self.m12 * self.m20))
                + self.m02 * ((self.m10 * self.m21) - (self.m11 * self.m20))
        }

        /// Returns the trace.
        #[must_use]
        pub const fn trace(self) -> f64 {
            self.m00 + self.m11 + self.m22
        }

        /// Returns the inverse when the determinant is finite and non-zero.
        ///
        /// Returns `None` for singular matrices or matrices with non-finite
        /// determinants.
        ///
        /// # Examples
        ///
        /// ```
        /// use use_matrix::Matrix3;
        ///
        /// let matrix = Matrix3::new(
        ///     1.0, 2.0, 3.0,
        ///     0.0, 1.0, 4.0,
        ///     5.0, 6.0, 0.0,
        /// );
        ///
        /// let inverse = matrix.inverse().expect("matrix should invert");
        ///
        /// assert_eq!(matrix * inverse, Matrix3::IDENTITY);
        /// ```
        #[must_use]
        pub fn inverse(self) -> Option<Self> {
            let determinant = self.determinant();

            if !determinant.is_finite() || matches!(determinant.classify(), FpCategory::Zero) {
                return None;
            }

            let c00 = mul_sub(self.m11, self.m22, self.m12, self.m21);
            let c01 = mul_sub(self.m12, self.m20, self.m10, self.m22);
            let c02 = mul_sub(self.m10, self.m21, self.m11, self.m20);
            let c10 = mul_sub(self.m02, self.m21, self.m01, self.m22);
            let c11 = mul_sub(self.m00, self.m22, self.m02, self.m20);
            let c12 = mul_sub(self.m01, self.m20, self.m00, self.m21);
            let c20 = mul_sub(self.m01, self.m12, self.m02, self.m11);
            let c21 = mul_sub(self.m02, self.m10, self.m00, self.m12);
            let c22 = mul_sub(self.m00, self.m11, self.m01, self.m10);

            Some(Self::new(c00, c10, c20, c01, c11, c21, c02, c12, c22) / determinant)
        }
    }

    impl Add for Matrix3 {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            Self::new(
                self.m00 + rhs.m00,
                self.m01 + rhs.m01,
                self.m02 + rhs.m02,
                self.m10 + rhs.m10,
                self.m11 + rhs.m11,
                self.m12 + rhs.m12,
                self.m20 + rhs.m20,
                self.m21 + rhs.m21,
                self.m22 + rhs.m22,
            )
        }
    }

    impl Sub for Matrix3 {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self::Output {
            Self::new(
                self.m00 - rhs.m00,
                self.m01 - rhs.m01,
                self.m02 - rhs.m02,
                self.m10 - rhs.m10,
                self.m11 - rhs.m11,
                self.m12 - rhs.m12,
                self.m20 - rhs.m20,
                self.m21 - rhs.m21,
                self.m22 - rhs.m22,
            )
        }
    }

    impl Mul<f64> for Matrix3 {
        type Output = Self;

        fn mul(self, rhs: f64) -> Self::Output {
            Self::new(
                self.m00 * rhs,
                self.m01 * rhs,
                self.m02 * rhs,
                self.m10 * rhs,
                self.m11 * rhs,
                self.m12 * rhs,
                self.m20 * rhs,
                self.m21 * rhs,
                self.m22 * rhs,
            )
        }
    }

    impl Div<f64> for Matrix3 {
        type Output = Self;

        fn div(self, rhs: f64) -> Self::Output {
            Self::new(
                self.m00 / rhs,
                self.m01 / rhs,
                self.m02 / rhs,
                self.m10 / rhs,
                self.m11 / rhs,
                self.m12 / rhs,
                self.m20 / rhs,
                self.m21 / rhs,
                self.m22 / rhs,
            )
        }
    }

    impl Neg for Matrix3 {
        type Output = Self;

        fn neg(self) -> Self::Output {
            Self::new(
                -self.m00, -self.m01, -self.m02, -self.m10, -self.m11, -self.m12, -self.m20,
                -self.m21, -self.m22,
            )
        }
    }

    impl Mul for Matrix3 {
        type Output = Self;

        fn mul(self, rhs: Self) -> Self::Output {
            Self::new(
                dot3(self.m00, rhs.m00, self.m01, rhs.m10, self.m02, rhs.m20),
                dot3(self.m00, rhs.m01, self.m01, rhs.m11, self.m02, rhs.m21),
                dot3(self.m00, rhs.m02, self.m01, rhs.m12, self.m02, rhs.m22),
                dot3(self.m10, rhs.m00, self.m11, rhs.m10, self.m12, rhs.m20),
                dot3(self.m10, rhs.m01, self.m11, rhs.m11, self.m12, rhs.m21),
                dot3(self.m10, rhs.m02, self.m11, rhs.m12, self.m12, rhs.m22),
                dot3(self.m20, rhs.m00, self.m21, rhs.m10, self.m22, rhs.m20),
                dot3(self.m20, rhs.m01, self.m21, rhs.m11, self.m22, rhs.m21),
                dot3(self.m20, rhs.m02, self.m21, rhs.m12, self.m22, rhs.m22),
            )
        }
    }

    impl Mul<Vector3> for Matrix3 {
        type Output = Vector3;

        fn mul(self, rhs: Vector3) -> Self::Output {
            Vector3::new(
                dot3(self.m00, rhs.x, self.m01, rhs.y, self.m02, rhs.z),
                dot3(self.m10, rhs.x, self.m11, rhs.y, self.m12, rhs.z),
                dot3(self.m20, rhs.x, self.m21, rhs.y, self.m22, rhs.z),
            )
        }
    }
}

/// Four-dimensional matrix primitives and operations.
pub mod matrix4 {
    use core::ops::{Add, Div, Mul, Neg, Sub};

    use use_vector::Vector4;

    #[inline]
    fn dot4(a0: f64, b0: f64, a1: f64, b1: f64, a2: f64, b2: f64, a3: f64, b3: f64) -> f64 {
        a0.mul_add(b0, a1.mul_add(b1, a2.mul_add(b2, a3 * b3)))
    }

    /// A 4x4 matrix stored in row-major order.
    ///
    /// # Examples
    ///
    /// ```
    /// use use_matrix::Matrix4;
    ///
    /// let matrix = Matrix4::from_rows([
    ///     [2.0, 1.0, 0.0, 0.0],
    ///     [0.0, 3.0, 4.0, 0.0],
    ///     [0.0, 0.0, 5.0, 6.0],
    ///     [0.0, 0.0, 0.0, 7.0],
    /// ]);
    ///
    /// assert_eq!(matrix.determinant(), 210.0);
    /// ```
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Matrix4 {
        pub m00: f64,
        pub m01: f64,
        pub m02: f64,
        pub m03: f64,
        pub m10: f64,
        pub m11: f64,
        pub m12: f64,
        pub m13: f64,
        pub m20: f64,
        pub m21: f64,
        pub m22: f64,
        pub m23: f64,
        pub m30: f64,
        pub m31: f64,
        pub m32: f64,
        pub m33: f64,
    }

    impl Matrix4 {
        /// The zero matrix.
        pub const ZERO: Self = Self::new(
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        );

        /// The identity matrix.
        ///
        /// # Examples
        ///
        /// ```
        /// use use_matrix::Matrix4;
        ///
        /// let matrix = Matrix4::from_rows([
        ///     [1.0, 2.0, 3.0, 4.0],
        ///     [5.0, 6.0, 7.0, 8.0],
        ///     [2.0, 0.0, 1.0, 3.0],
        ///     [4.0, 1.0, 0.0, 2.0],
        /// ]);
        ///
        /// assert_eq!(Matrix4::IDENTITY * matrix, matrix);
        /// ```
        pub const IDENTITY: Self = Self::new(
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        );

        /// Creates a matrix from row-major entries.
        #[must_use]
        #[allow(clippy::similar_names, clippy::too_many_arguments)]
        pub const fn new(
            m00: f64,
            m01: f64,
            m02: f64,
            m03: f64,
            m10: f64,
            m11: f64,
            m12: f64,
            m13: f64,
            m20: f64,
            m21: f64,
            m22: f64,
            m23: f64,
            m30: f64,
            m31: f64,
            m32: f64,
            m33: f64,
        ) -> Self {
            Self {
                m00,
                m01,
                m02,
                m03,
                m10,
                m11,
                m12,
                m13,
                m20,
                m21,
                m22,
                m23,
                m30,
                m31,
                m32,
                m33,
            }
        }

        /// Creates a matrix from row arrays.
        #[must_use]
        pub const fn from_rows(rows: [[f64; 4]; 4]) -> Self {
            Self::new(
                rows[0][0], rows[0][1], rows[0][2], rows[0][3], rows[1][0], rows[1][1], rows[1][2],
                rows[1][3], rows[2][0], rows[2][1], rows[2][2], rows[2][3], rows[3][0], rows[3][1],
                rows[3][2], rows[3][3],
            )
        }

        /// Creates a matrix from column arrays.
        #[must_use]
        pub const fn from_cols(cols: [[f64; 4]; 4]) -> Self {
            Self::new(
                cols[0][0], cols[1][0], cols[2][0], cols[3][0], cols[0][1], cols[1][1], cols[2][1],
                cols[3][1], cols[0][2], cols[1][2], cols[2][2], cols[3][2], cols[0][3], cols[1][3],
                cols[2][3], cols[3][3],
            )
        }

        /// Returns the transpose.
        #[must_use]
        pub const fn transpose(self) -> Self {
            Self::new(
                self.m00, self.m10, self.m20, self.m30, self.m01, self.m11, self.m21, self.m31,
                self.m02, self.m12, self.m22, self.m32, self.m03, self.m13, self.m23, self.m33,
            )
        }

        /// Returns the determinant.
        #[must_use]
        pub const fn determinant(self) -> f64 {
            let minor00 = determinant3([
                [self.m11, self.m12, self.m13],
                [self.m21, self.m22, self.m23],
                [self.m31, self.m32, self.m33],
            ]);
            let minor01 = determinant3([
                [self.m10, self.m12, self.m13],
                [self.m20, self.m22, self.m23],
                [self.m30, self.m32, self.m33],
            ]);
            let minor02 = determinant3([
                [self.m10, self.m11, self.m13],
                [self.m20, self.m21, self.m23],
                [self.m30, self.m31, self.m33],
            ]);
            let minor03 = determinant3([
                [self.m10, self.m11, self.m12],
                [self.m20, self.m21, self.m22],
                [self.m30, self.m31, self.m32],
            ]);

            (self.m00 * minor00) - (self.m01 * minor01) + (self.m02 * minor02)
                - (self.m03 * minor03)
        }

        /// Returns the trace.
        #[must_use]
        pub const fn trace(self) -> f64 {
            self.m00 + self.m11 + self.m22 + self.m33
        }
    }

    impl Add for Matrix4 {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            Self::new(
                self.m00 + rhs.m00,
                self.m01 + rhs.m01,
                self.m02 + rhs.m02,
                self.m03 + rhs.m03,
                self.m10 + rhs.m10,
                self.m11 + rhs.m11,
                self.m12 + rhs.m12,
                self.m13 + rhs.m13,
                self.m20 + rhs.m20,
                self.m21 + rhs.m21,
                self.m22 + rhs.m22,
                self.m23 + rhs.m23,
                self.m30 + rhs.m30,
                self.m31 + rhs.m31,
                self.m32 + rhs.m32,
                self.m33 + rhs.m33,
            )
        }
    }

    impl Sub for Matrix4 {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self::Output {
            Self::new(
                self.m00 - rhs.m00,
                self.m01 - rhs.m01,
                self.m02 - rhs.m02,
                self.m03 - rhs.m03,
                self.m10 - rhs.m10,
                self.m11 - rhs.m11,
                self.m12 - rhs.m12,
                self.m13 - rhs.m13,
                self.m20 - rhs.m20,
                self.m21 - rhs.m21,
                self.m22 - rhs.m22,
                self.m23 - rhs.m23,
                self.m30 - rhs.m30,
                self.m31 - rhs.m31,
                self.m32 - rhs.m32,
                self.m33 - rhs.m33,
            )
        }
    }

    impl Mul<f64> for Matrix4 {
        type Output = Self;

        fn mul(self, rhs: f64) -> Self::Output {
            Self::new(
                self.m00 * rhs,
                self.m01 * rhs,
                self.m02 * rhs,
                self.m03 * rhs,
                self.m10 * rhs,
                self.m11 * rhs,
                self.m12 * rhs,
                self.m13 * rhs,
                self.m20 * rhs,
                self.m21 * rhs,
                self.m22 * rhs,
                self.m23 * rhs,
                self.m30 * rhs,
                self.m31 * rhs,
                self.m32 * rhs,
                self.m33 * rhs,
            )
        }
    }

    impl Div<f64> for Matrix4 {
        type Output = Self;

        fn div(self, rhs: f64) -> Self::Output {
            Self::new(
                self.m00 / rhs,
                self.m01 / rhs,
                self.m02 / rhs,
                self.m03 / rhs,
                self.m10 / rhs,
                self.m11 / rhs,
                self.m12 / rhs,
                self.m13 / rhs,
                self.m20 / rhs,
                self.m21 / rhs,
                self.m22 / rhs,
                self.m23 / rhs,
                self.m30 / rhs,
                self.m31 / rhs,
                self.m32 / rhs,
                self.m33 / rhs,
            )
        }
    }

    impl Neg for Matrix4 {
        type Output = Self;

        fn neg(self) -> Self::Output {
            Self::new(
                -self.m00, -self.m01, -self.m02, -self.m03, -self.m10, -self.m11, -self.m12,
                -self.m13, -self.m20, -self.m21, -self.m22, -self.m23, -self.m30, -self.m31,
                -self.m32, -self.m33,
            )
        }
    }

    impl Mul for Matrix4 {
        type Output = Self;

        fn mul(self, rhs: Self) -> Self::Output {
            Self::new(
                dot4(
                    self.m00, rhs.m00, self.m01, rhs.m10, self.m02, rhs.m20, self.m03, rhs.m30,
                ),
                dot4(
                    self.m00, rhs.m01, self.m01, rhs.m11, self.m02, rhs.m21, self.m03, rhs.m31,
                ),
                dot4(
                    self.m00, rhs.m02, self.m01, rhs.m12, self.m02, rhs.m22, self.m03, rhs.m32,
                ),
                dot4(
                    self.m00, rhs.m03, self.m01, rhs.m13, self.m02, rhs.m23, self.m03, rhs.m33,
                ),
                dot4(
                    self.m10, rhs.m00, self.m11, rhs.m10, self.m12, rhs.m20, self.m13, rhs.m30,
                ),
                dot4(
                    self.m10, rhs.m01, self.m11, rhs.m11, self.m12, rhs.m21, self.m13, rhs.m31,
                ),
                dot4(
                    self.m10, rhs.m02, self.m11, rhs.m12, self.m12, rhs.m22, self.m13, rhs.m32,
                ),
                dot4(
                    self.m10, rhs.m03, self.m11, rhs.m13, self.m12, rhs.m23, self.m13, rhs.m33,
                ),
                dot4(
                    self.m20, rhs.m00, self.m21, rhs.m10, self.m22, rhs.m20, self.m23, rhs.m30,
                ),
                dot4(
                    self.m20, rhs.m01, self.m21, rhs.m11, self.m22, rhs.m21, self.m23, rhs.m31,
                ),
                dot4(
                    self.m20, rhs.m02, self.m21, rhs.m12, self.m22, rhs.m22, self.m23, rhs.m32,
                ),
                dot4(
                    self.m20, rhs.m03, self.m21, rhs.m13, self.m22, rhs.m23, self.m23, rhs.m33,
                ),
                dot4(
                    self.m30, rhs.m00, self.m31, rhs.m10, self.m32, rhs.m20, self.m33, rhs.m30,
                ),
                dot4(
                    self.m30, rhs.m01, self.m31, rhs.m11, self.m32, rhs.m21, self.m33, rhs.m31,
                ),
                dot4(
                    self.m30, rhs.m02, self.m31, rhs.m12, self.m32, rhs.m22, self.m33, rhs.m32,
                ),
                dot4(
                    self.m30, rhs.m03, self.m31, rhs.m13, self.m32, rhs.m23, self.m33, rhs.m33,
                ),
            )
        }
    }

    impl Mul<Vector4> for Matrix4 {
        type Output = Vector4;

        fn mul(self, rhs: Vector4) -> Self::Output {
            Vector4::new(
                dot4(
                    self.m00, rhs.x, self.m01, rhs.y, self.m02, rhs.z, self.m03, rhs.w,
                ),
                dot4(
                    self.m10, rhs.x, self.m11, rhs.y, self.m12, rhs.z, self.m13, rhs.w,
                ),
                dot4(
                    self.m20, rhs.x, self.m21, rhs.y, self.m22, rhs.z, self.m23, rhs.w,
                ),
                dot4(
                    self.m30, rhs.x, self.m31, rhs.y, self.m32, rhs.z, self.m33, rhs.w,
                ),
            )
        }
    }

    const fn determinant3(rows: [[f64; 3]; 3]) -> f64 {
        (rows[0][0] * ((rows[1][1] * rows[2][2]) - (rows[1][2] * rows[2][1])))
            - (rows[0][1] * ((rows[1][0] * rows[2][2]) - (rows[1][2] * rows[2][0])))
            + (rows[0][2] * ((rows[1][0] * rows[2][1]) - (rows[1][1] * rows[2][0])))
    }
}

#[cfg(test)]
mod tests {
    use super::{Matrix2, Matrix3, Matrix4};
    use use_vector::{Vector2, Vector3, Vector4};

    fn assert_close(left: f64, right: f64) {
        assert!((left - right).abs() < 1.0e-10, "left={left}, right={right}");
    }

    fn assert_matrix2_close(left: Matrix2, right: Matrix2) {
        assert_close(left.m00, right.m00);
        assert_close(left.m01, right.m01);
        assert_close(left.m10, right.m10);
        assert_close(left.m11, right.m11);
    }

    fn assert_matrix3_close(left: Matrix3, right: Matrix3) {
        assert_close(left.m00, right.m00);
        assert_close(left.m01, right.m01);
        assert_close(left.m02, right.m02);
        assert_close(left.m10, right.m10);
        assert_close(left.m11, right.m11);
        assert_close(left.m12, right.m12);
        assert_close(left.m20, right.m20);
        assert_close(left.m21, right.m21);
        assert_close(left.m22, right.m22);
    }

    fn assert_matrix4_close(left: Matrix4, right: Matrix4) {
        assert_close(left.m00, right.m00);
        assert_close(left.m01, right.m01);
        assert_close(left.m02, right.m02);
        assert_close(left.m03, right.m03);
        assert_close(left.m10, right.m10);
        assert_close(left.m11, right.m11);
        assert_close(left.m12, right.m12);
        assert_close(left.m13, right.m13);
        assert_close(left.m20, right.m20);
        assert_close(left.m21, right.m21);
        assert_close(left.m22, right.m22);
        assert_close(left.m23, right.m23);
        assert_close(left.m30, right.m30);
        assert_close(left.m31, right.m31);
        assert_close(left.m32, right.m32);
        assert_close(left.m33, right.m33);
    }

    #[test]
    fn matrix2_construction_and_constants_work() {
        let matrix = Matrix2::new(1.0, 2.0, 3.0, 4.0);

        assert_eq!(Matrix2::ZERO, Matrix2::new(0.0, 0.0, 0.0, 0.0));
        assert_eq!(Matrix2::IDENTITY, Matrix2::new(1.0, 0.0, 0.0, 1.0));
        assert_eq!(matrix, Matrix2::from_rows([[1.0, 2.0], [3.0, 4.0]]));
        assert_eq!(matrix, Matrix2::from_cols([[1.0, 3.0], [2.0, 4.0]]));
    }

    #[test]
    fn matrix2_arithmetic_and_products_work() {
        let left = Matrix2::new(1.0, 2.0, 3.0, 4.0);
        let right = Matrix2::new(5.0, 6.0, 7.0, 8.0);
        let scale = Matrix2::from_rows([[2.0, 0.0], [0.0, 3.0]]);

        assert_eq!(left + right, Matrix2::new(6.0, 8.0, 10.0, 12.0));
        assert_eq!(left - right, Matrix2::new(-4.0, -4.0, -4.0, -4.0));
        assert_eq!(left * 2.0, Matrix2::new(2.0, 4.0, 6.0, 8.0));
        assert_eq!(left / 2.0, Matrix2::new(0.5, 1.0, 1.5, 2.0));
        assert_eq!(-left, Matrix2::new(-1.0, -2.0, -3.0, -4.0));
        assert_eq!(left * scale, Matrix2::new(2.0, 6.0, 6.0, 12.0));
        assert_eq!(left * Vector2::new(1.0, -1.0), Vector2::new(-1.0, -1.0));
        assert_eq!(Matrix2::IDENTITY * left, left);
        assert_eq!(left * Matrix2::IDENTITY, left);
    }

    #[test]
    fn matrix2_transpose_determinant_trace_and_inverse_work() {
        let matrix = Matrix2::new(4.0, 7.0, 2.0, 6.0);

        assert_eq!(matrix.transpose(), Matrix2::new(4.0, 2.0, 7.0, 6.0));
        assert_eq!(matrix.transpose().transpose(), matrix);
        assert_close(matrix.determinant(), 10.0);
        assert_close(matrix.trace(), 10.0);

        let inverse = matrix.inverse().expect("matrix should invert");

        assert_matrix2_close(inverse, Matrix2::new(0.6, -0.7, -0.2, 0.4));
        assert_matrix2_close(matrix * inverse, Matrix2::IDENTITY);
        assert!(Matrix2::new(1.0, 2.0, 2.0, 4.0).inverse().is_none());
        assert!(
            Matrix2::new(f64::INFINITY, 0.0, 0.0, 1.0)
                .inverse()
                .is_none()
        );
    }

    #[test]
    fn matrix3_construction_and_constants_work() {
        let matrix = Matrix3::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);

        assert_eq!(
            Matrix3::ZERO,
            Matrix3::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0)
        );
        assert_eq!(
            Matrix3::IDENTITY,
            Matrix3::new(1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0)
        );
        assert_eq!(
            matrix,
            Matrix3::from_rows([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0],])
        );
        assert_eq!(
            matrix,
            Matrix3::from_cols([[1.0, 4.0, 7.0], [2.0, 5.0, 8.0], [3.0, 6.0, 9.0],])
        );
    }

    #[test]
    fn matrix3_arithmetic_and_products_work() {
        let left = Matrix3::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 10.0);
        let right = Matrix3::new(9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0);
        let scale = Matrix3::from_rows([[2.0, 0.0, 0.0], [0.0, 3.0, 0.0], [0.0, 0.0, 4.0]]);

        assert_eq!(
            left + right,
            Matrix3::new(10.0, 10.0, 10.0, 10.0, 10.0, 10.0, 10.0, 10.0, 11.0)
        );
        assert_eq!(
            left - right,
            Matrix3::new(-8.0, -6.0, -4.0, -2.0, 0.0, 2.0, 4.0, 6.0, 9.0)
        );
        assert_eq!(
            left * 2.0,
            Matrix3::new(2.0, 4.0, 6.0, 8.0, 10.0, 12.0, 14.0, 16.0, 20.0)
        );
        assert_eq!(
            left / 2.0,
            Matrix3::new(0.5, 1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0, 5.0)
        );
        assert_eq!(
            -left,
            Matrix3::new(-1.0, -2.0, -3.0, -4.0, -5.0, -6.0, -7.0, -8.0, -10.0)
        );
        assert_eq!(
            left * scale,
            Matrix3::new(2.0, 6.0, 12.0, 8.0, 15.0, 24.0, 14.0, 24.0, 40.0)
        );
        assert_eq!(
            left * Vector3::new(1.0, 0.0, -1.0),
            Vector3::new(-2.0, -2.0, -3.0)
        );
        assert_eq!(Matrix3::IDENTITY * left, left);
        assert_eq!(left * Matrix3::IDENTITY, left);
    }

    #[test]
    fn matrix3_transpose_determinant_trace_and_inverse_work() {
        let matrix = Matrix3::new(1.0, 2.0, 3.0, 0.0, 1.0, 4.0, 5.0, 6.0, 0.0);

        assert_eq!(
            matrix.transpose(),
            Matrix3::new(1.0, 0.0, 5.0, 2.0, 1.0, 6.0, 3.0, 4.0, 0.0)
        );
        assert_eq!(matrix.transpose().transpose(), matrix);
        assert_close(matrix.determinant(), 1.0);
        assert_close(matrix.trace(), 2.0);

        let inverse = matrix.inverse().expect("matrix should invert");

        assert_eq!(
            inverse,
            Matrix3::new(-24.0, 18.0, 5.0, 20.0, -15.0, -4.0, -5.0, 4.0, 1.0)
        );
        assert_matrix3_close(matrix * inverse, Matrix3::IDENTITY);
        assert!(
            Matrix3::new(1.0, 2.0, 3.0, 2.0, 4.0, 6.0, 7.0, 8.0, 9.0)
                .inverse()
                .is_none()
        );
        assert!(
            Matrix3::new(f64::INFINITY, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0)
                .inverse()
                .is_none()
        );
    }

    #[test]
    fn matrix4_construction_and_constants_work() {
        let matrix = Matrix4::from_rows([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [2.0, 0.0, 1.0, 3.0],
            [4.0, 1.0, 0.0, 2.0],
        ]);

        assert_eq!(
            Matrix4::ZERO,
            Matrix4::new(
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            )
        );
        assert_eq!(
            Matrix4::IDENTITY,
            Matrix4::new(
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            )
        );
        assert_eq!(
            matrix,
            Matrix4::from_cols([
                [1.0, 5.0, 2.0, 4.0],
                [2.0, 6.0, 0.0, 1.0],
                [3.0, 7.0, 1.0, 0.0],
                [4.0, 8.0, 3.0, 2.0],
            ])
        );
    }

    #[test]
    fn matrix4_arithmetic_and_products_work() {
        let left = Matrix4::from_rows([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [2.0, 0.0, 1.0, 3.0],
            [4.0, 1.0, 0.0, 2.0],
        ]);
        let right = Matrix4::from_rows([
            [4.0, 3.0, 2.0, 1.0],
            [0.0, 1.0, 2.0, 3.0],
            [1.0, 0.0, 1.0, 0.0],
            [2.0, 1.0, 0.0, 2.0],
        ]);
        let scale = Matrix4::from_rows([
            [2.0, 0.0, 0.0, 0.0],
            [0.0, 3.0, 0.0, 0.0],
            [0.0, 0.0, 4.0, 0.0],
            [0.0, 0.0, 0.0, 5.0],
        ]);

        assert_eq!(
            left + right,
            Matrix4::from_rows([
                [5.0, 5.0, 5.0, 5.0],
                [5.0, 7.0, 9.0, 11.0],
                [3.0, 0.0, 2.0, 3.0],
                [6.0, 2.0, 0.0, 4.0],
            ])
        );
        assert_eq!(
            left - right,
            Matrix4::from_rows([
                [-3.0, -1.0, 1.0, 3.0],
                [5.0, 5.0, 5.0, 5.0],
                [1.0, 0.0, 0.0, 3.0],
                [2.0, 0.0, 0.0, 0.0],
            ])
        );
        assert_eq!(
            left * 2.0,
            Matrix4::from_rows([
                [2.0, 4.0, 6.0, 8.0],
                [10.0, 12.0, 14.0, 16.0],
                [4.0, 0.0, 2.0, 6.0],
                [8.0, 2.0, 0.0, 4.0],
            ])
        );
        assert_eq!(
            left / 2.0,
            Matrix4::from_rows([
                [0.5, 1.0, 1.5, 2.0],
                [2.5, 3.0, 3.5, 4.0],
                [1.0, 0.0, 0.5, 1.5],
                [2.0, 0.5, 0.0, 1.0],
            ])
        );
        assert_eq!(
            -left,
            Matrix4::from_rows([
                [-1.0, -2.0, -3.0, -4.0],
                [-5.0, -6.0, -7.0, -8.0],
                [-2.0, 0.0, -1.0, -3.0],
                [-4.0, -1.0, 0.0, -2.0],
            ])
        );
        assert_eq!(
            left * scale,
            Matrix4::from_rows([
                [2.0, 6.0, 12.0, 20.0],
                [10.0, 18.0, 28.0, 40.0],
                [4.0, 0.0, 4.0, 15.0],
                [8.0, 3.0, 0.0, 10.0],
            ])
        );
        assert_eq!(
            left * Vector4::new(1.0, 0.0, -1.0, 2.0),
            Vector4::new(6.0, 14.0, 7.0, 8.0)
        );
        assert_eq!(Matrix4::IDENTITY * left, left);
        assert_eq!(left * Matrix4::IDENTITY, left);
    }

    #[test]
    fn matrix4_transpose_determinant_and_trace_work() {
        let matrix = Matrix4::from_rows([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [2.0, 0.0, 1.0, 3.0],
            [4.0, 1.0, 0.0, 2.0],
        ]);
        let triangular = Matrix4::from_rows([
            [2.0, 1.0, 0.0, 0.0],
            [0.0, 3.0, 4.0, 0.0],
            [0.0, 0.0, 5.0, 6.0],
            [0.0, 0.0, 0.0, 7.0],
        ]);

        assert_eq!(
            matrix.transpose(),
            Matrix4::from_rows([
                [1.0, 5.0, 2.0, 4.0],
                [2.0, 6.0, 0.0, 1.0],
                [3.0, 7.0, 1.0, 0.0],
                [4.0, 8.0, 3.0, 2.0],
            ])
        );
        assert_eq!(matrix.transpose().transpose(), matrix);
        assert_close(matrix.trace(), 10.0);
        assert_close(triangular.determinant(), 210.0);
        assert_matrix4_close(Matrix4::IDENTITY * triangular, triangular);
    }
}
