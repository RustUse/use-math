#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Trigonometric utilities for `RustUse`.

use core::f64::consts::{PI, TAU};

/// Explicit angle value stored in radians.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Angle {
    radians: f64,
}

impl Angle {
    /// Creates an angle from a radians value.
    #[must_use]
    pub const fn from_radians(radians: f64) -> Self {
        Self { radians }
    }

    /// Creates an angle from a degrees value.
    #[must_use]
    pub const fn from_degrees(degrees: f64) -> Self {
        Self::from_radians(degrees.to_radians())
    }

    /// Returns the angle in radians.
    #[must_use]
    pub const fn radians(self) -> f64 {
        self.radians
    }

    /// Returns the angle in degrees.
    #[must_use]
    pub const fn degrees(self) -> f64 {
        self.radians.to_degrees()
    }

    /// Returns the angle normalized into the interval `[0, 2π)`.
    #[must_use]
    pub fn normalized(self) -> Self {
        Self::from_radians(self.radians.rem_euclid(TAU))
    }

    /// Returns the angle normalized into the interval `(-π, π]`.
    #[must_use]
    pub fn normalized_signed(self) -> Self {
        let radians = self.normalized().radians();

        if radians > PI {
            Self::from_radians(radians - TAU)
        } else {
            Self::from_radians(radians)
        }
    }

    /// Returns the sine of the angle.
    #[must_use]
    pub fn sin(self) -> f64 {
        self.radians.sin()
    }

    /// Returns the cosine of the angle.
    #[must_use]
    pub fn cos(self) -> f64 {
        self.radians.cos()
    }

    /// Returns the tangent of the angle.
    #[must_use]
    pub fn tan(self) -> f64 {
        self.radians.tan()
    }

    /// Returns the sine and cosine of the angle as a pair.
    #[must_use]
    pub fn sin_cos(self) -> (f64, f64) {
        self.radians.sin_cos()
    }
}

/// Converts a degrees value into radians.
#[must_use]
pub const fn degrees_to_radians(degrees: f64) -> f64 {
    degrees.to_radians()
}

/// Converts a radians value into degrees.
#[must_use]
pub const fn radians_to_degrees(radians: f64) -> f64 {
    radians.to_degrees()
}

/// Normalizes a degrees value into the interval `[0, 360)`.
#[must_use]
pub fn normalize_degrees(degrees: f64) -> f64 {
    degrees.rem_euclid(360.0)
}

/// Normalizes a radians value into the interval `[0, 2π)`.
#[must_use]
pub fn normalize_radians(radians: f64) -> f64 {
    radians.rem_euclid(TAU)
}

/// Evaluates `sin` for a degrees input.
#[must_use]
pub fn sin_deg(degrees: f64) -> f64 {
    degrees_to_radians(degrees).sin()
}

/// Evaluates `cos` for a degrees input.
#[must_use]
pub fn cos_deg(degrees: f64) -> f64 {
    degrees_to_radians(degrees).cos()
}

/// Evaluates `tan` for a degrees input.
#[must_use]
pub fn tan_deg(degrees: f64) -> f64 {
    degrees_to_radians(degrees).tan()
}

pub mod prelude;

#[cfg(test)]
mod tests {
    use super::{
        Angle, cos_deg, degrees_to_radians, normalize_degrees, normalize_radians,
        radians_to_degrees, sin_deg, tan_deg,
    };
    use core::f64::consts::{PI, TAU};

    fn assert_close(left: f64, right: f64) {
        assert!((left - right).abs() < 1.0e-12, "left={left}, right={right}");
    }

    #[test]
    fn converts_between_degrees_and_radians() {
        let straight = Angle::from_degrees(180.0);
        let right = Angle::from_radians(PI / 2.0);

        assert_close(straight.radians(), PI);
        assert_close(right.degrees(), 90.0);
        assert_close(degrees_to_radians(60.0), PI / 3.0);
        assert_close(radians_to_degrees(PI / 6.0), 30.0);
    }

    #[test]
    fn normalizes_angles_and_evaluates_trig_helpers() {
        let wrapped = Angle::from_degrees(-30.0).normalized();
        let signed = Angle::from_degrees(450.0).normalized_signed();
        let (sine, cosine) = Angle::from_degrees(30.0).sin_cos();

        assert_close(wrapped.degrees(), 330.0);
        assert_close(signed.degrees(), 90.0);
        assert_close(sine, 0.5);
        assert_close(cosine, (3.0_f64).sqrt() / 2.0);
        assert_close(sin_deg(30.0), 0.5);
        assert_close(cos_deg(60.0), 0.5);
        assert_close(tan_deg(45.0), 1.0);
        assert_close(normalize_degrees(-90.0), 270.0);
        assert_close(normalize_radians(-PI / 2.0), TAU - (PI / 2.0));
    }
}
