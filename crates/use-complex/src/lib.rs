#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Complex-number primitives for `RustUse`.

use core::fmt;
use core::ops::{Add, Div, Mul, Neg, Sub};

/// A complex number stored in rectangular form.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Complex<T> {
    /// The real component.
    pub re: T,
    /// The imaginary component.
    pub im: T,
}

/// A standalone imaginary value.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Imaginary<T>(pub T);

impl<T> Complex<T> {
    /// Creates a complex number from real and imaginary parts.
    #[must_use]
    pub const fn new(re: T, im: T) -> Self {
        Self { re, im }
    }

    /// Returns the real component.
    #[must_use]
    pub const fn real(&self) -> &T {
        &self.re
    }

    /// Returns the imaginary component.
    #[must_use]
    pub const fn imaginary(&self) -> &T {
        &self.im
    }
}

impl<T> Imaginary<T> {
    /// Creates a new imaginary value.
    #[must_use]
    pub const fn new(value: T) -> Self {
        Self(value)
    }

    /// Returns the stored coefficient.
    #[must_use]
    pub const fn value(&self) -> &T {
        &self.0
    }
}

impl<T> Complex<T>
where
    T: Default,
{
    /// Creates a real-only complex value.
    #[must_use]
    pub fn from_real(re: T) -> Self {
        Self::new(re, T::default())
    }

    /// Creates an imaginary-only complex value.
    #[must_use]
    pub fn from_imaginary(im: T) -> Self {
        Self::new(T::default(), im)
    }
}

impl<T> Complex<T>
where
    T: Default + From<u8>,
{
    /// Returns `0 + 0i`.
    #[must_use]
    pub fn zero() -> Self {
        Self::new(T::default(), T::default())
    }

    /// Returns `1 + 0i`.
    #[must_use]
    pub fn one() -> Self {
        Self::from_real(T::from(1_u8))
    }

    /// Returns `0 + 1i`.
    #[must_use]
    pub fn i() -> Self {
        Self::from_imaginary(T::from(1_u8))
    }
}

impl<T> Complex<T>
where
    T: Copy + Neg<Output = T>,
{
    /// Returns the complex conjugate.
    #[must_use]
    pub fn conjugate(&self) -> Self {
        Self::new(self.re, -self.im)
    }
}

impl<T> Complex<T>
where
    T: Copy + Add<Output = T> + Mul<Output = T>,
{
    /// Returns the squared magnitude, `re^2 + im^2`.
    #[must_use]
    pub fn magnitude_squared(&self) -> T {
        (self.re * self.re) + (self.im * self.im)
    }
}

macro_rules! impl_float_methods {
	($($ty:ty),* $(,)?) => {
		$(
			impl Complex<$ty> {
				/// Returns the magnitude, `sqrt(re^2 + im^2)`.
				#[must_use]
				pub fn magnitude(&self) -> $ty {
					self.magnitude_squared().sqrt()
				}

				/// Returns the argument in radians.
				#[must_use]
				pub fn argument(&self) -> $ty {
					self.im.atan2(self.re)
				}

				/// Builds a complex value from polar coordinates.
				#[must_use]
				pub fn from_polar(magnitude: $ty, argument: $ty) -> Self {
					Self::new(magnitude * argument.cos(), magnitude * argument.sin())
				}

				/// Returns `(magnitude, argument)` in polar form.
				#[must_use]
				pub fn to_polar(&self) -> ($ty, $ty) {
					(self.magnitude(), self.argument())
				}
			}
		)*
	};
}

impl_float_methods!(f32, f64);

impl<T> From<T> for Complex<T>
where
    T: Default,
{
    fn from(value: T) -> Self {
        Self::from_real(value)
    }
}

impl<T> From<Imaginary<T>> for Complex<T>
where
    T: Default,
{
    fn from(value: Imaginary<T>) -> Self {
        Self::from_imaginary(value.0)
    }
}

impl<T> From<T> for Imaginary<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<T> Add for Complex<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.re + rhs.re, self.im + rhs.im)
    }
}

impl<T> Sub for Complex<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.re - rhs.re, self.im - rhs.im)
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl<T> Mul for Complex<T>
where
    T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let re = (self.re * rhs.re) - (self.im * rhs.im);
        let im = (self.re * rhs.im) + (self.im * rhs.re);

        Self::new(re, im)
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl<T> Div for Complex<T>
where
    T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let denominator = (rhs.re * rhs.re) + (rhs.im * rhs.im);
        let re = ((self.re * rhs.re) + (self.im * rhs.im)) / denominator;
        let im = ((self.im * rhs.re) - (self.re * rhs.im)) / denominator;

        Self::new(re, im)
    }
}

impl<T> Neg for Complex<T>
where
    T: Neg<Output = T>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.re, -self.im)
    }
}

impl<T> fmt::Display for Complex<T>
where
    T: Copy + Default + fmt::Display + Neg<Output = T> + PartialEq + PartialOrd,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let zero = T::default();

        if self.im == zero {
            return write!(f, "{}", self.re);
        }

        if self.re == zero {
            if self.im < zero {
                return write!(f, "-{}i", -self.im);
            }

            return write!(f, "{}i", self.im);
        }

        if self.im < zero {
            return write!(f, "{} - {}i", self.re, -self.im);
        }

        write!(f, "{} + {}i", self.re, self.im)
    }
}

impl<T> fmt::Display for Imaginary<T>
where
    T: Copy + Default + fmt::Display + Neg<Output = T> + PartialEq + PartialOrd,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let zero = T::default();

        if self.0 < zero {
            return write!(f, "-{}i", -self.0);
        }

        if self.0 == zero {
            return write!(f, "0i");
        }

        write!(f, "{}i", self.0)
    }
}

pub mod prelude;

#[cfg(test)]
mod tests {
    use super::{Complex, Imaginary};
    use core::f64::consts::{FRAC_PI_2, FRAC_PI_3};

    fn assert_close(lhs: f64, rhs: f64) {
        let difference = (lhs - rhs).abs();

        assert!(
            difference <= 1.0e-10,
            "left={lhs}, right={rhs}, diff={difference}"
        );
    }

    #[test]
    fn constructs_complex_values_and_accessors() {
        let value = Complex::new(3_i32, 4_i32);

        assert_eq!(value.re, 3);
        assert_eq!(value.im, 4);
        assert_eq!(value.real(), &3);
        assert_eq!(value.imaginary(), &4);
    }

    #[test]
    fn supports_imaginary_only_values() {
        let imaginary = Imaginary::new(4_i32);
        let complex = Complex::from(imaginary);

        assert_eq!(imaginary.value(), &4);
        assert_eq!(imaginary.to_string(), "4i");
        assert_eq!(complex, Complex::new(0, 4));
        assert_eq!(Complex::<i32>::from_imaginary(7), Complex::new(0, 7));
    }

    #[test]
    fn provides_zero_one_and_i() {
        assert_eq!(Complex::<i32>::zero(), Complex::new(0, 0));
        assert_eq!(Complex::<i32>::one(), Complex::new(1, 0));
        assert_eq!(Complex::<i32>::i(), Complex::new(0, 1));
        assert_eq!(Complex::<i32>::from_real(9), Complex::new(9, 0));
    }

    #[test]
    fn adds_and_subtracts_complex_values() {
        let lhs = Complex::new(3_i32, 4_i32);
        let rhs = Complex::new(1_i32, -2_i32);

        assert_eq!(lhs + rhs, Complex::new(4, 2));
        assert_eq!(lhs - rhs, Complex::new(2, 6));
    }

    #[test]
    fn multiplies_complex_values() {
        let lhs = Complex::new(3_i32, 4_i32);
        let rhs = Complex::new(1_i32, -2_i32);

        assert_eq!(lhs * rhs, Complex::new(11, -2));
    }

    #[test]
    fn divides_complex_values() {
        let lhs = Complex::new(3.0_f64, 4.0_f64);
        let rhs = Complex::new(1.0_f64, -2.0_f64);
        let quotient = lhs / rhs;

        assert_close(quotient.re, -1.0);
        assert_close(quotient.im, 2.0);
    }

    #[test]
    fn negates_and_conjugates() {
        let value = Complex::new(3_i32, -4_i32);

        assert_eq!(-value, Complex::new(-3, 4));
        assert_eq!(value.conjugate(), Complex::new(3, 4));
    }

    #[test]
    fn computes_magnitude_squared_for_integer_values() {
        let value = Complex::new(3_i32, 4_i32);

        assert_eq!(value.magnitude_squared(), 25);
    }

    #[test]
    fn computes_magnitude_and_argument_for_floats() {
        let value = Complex::new(3.0_f64, 4.0_f64);

        assert_close(value.magnitude(), 5.0);
        assert_close(Complex::new(0.0_f64, 4.0_f64).argument(), FRAC_PI_2);
    }

    #[test]
    fn converts_to_and_from_polar_form() {
        let value = Complex::<f64>::from_polar(5.0_f64, FRAC_PI_3);
        let (magnitude, argument) = value.to_polar();

        assert_close(magnitude, 5.0);
        assert_close(argument, FRAC_PI_3);
    }

    #[test]
    fn formats_complex_and_imaginary_values() {
        assert_eq!(Complex::new(3_i32, 4_i32).to_string(), "3 + 4i");
        assert_eq!(Complex::new(3_i32, -4_i32).to_string(), "3 - 4i");
        assert_eq!(Complex::new(0_i32, 4_i32).to_string(), "4i");
        assert_eq!(Complex::new(3_i32, 0_i32).to_string(), "3");
        assert_eq!(Imaginary::new(-4_i32).to_string(), "-4i");
    }

    #[test]
    fn integer_division_works_when_components_divide_evenly() {
        let lhs = Complex::new(4_i32, 2_i32);
        let rhs = Complex::new(2_i32, 0_i32);

        assert_eq!(lhs / rhs, Complex::new(2, 1));
    }

    #[test]
    fn floating_point_values_round_trip_through_helpers() {
        let from_real = Complex::<f64>::from(3.0);
        let from_imaginary = Complex::<f64>::from(Imaginary::new(2.5));

        assert_close(from_real.re, 3.0);
        assert_close(from_real.im, 0.0);
        assert_close(from_imaginary.re, 0.0);
        assert_close(from_imaginary.im, 2.5);
    }
}
