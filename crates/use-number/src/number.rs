//! Raw-number classification helpers.

use core::num::FpCategory;

/// Broad categories for raw `f64` values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumberCategory {
    /// The value is not a number.
    Nan,
    /// The value is positive or negative infinity.
    Infinite,
    /// The value is positive or negative zero.
    Zero,
    /// The value is non-zero but below the normal minimum magnitude.
    Subnormal,
    /// The value is a finite normal floating-point number.
    Normal,
}

/// Sign classes for raw numeric values that are not `NaN`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumberSign {
    /// The value is below zero.
    Negative,
    /// The value is equal to zero, including `-0.0`.
    Zero,
    /// The value is above zero.
    Positive,
}

/// Classifies a raw `f64` by floating-point category.
#[must_use]
pub const fn classify_number(value: f64) -> NumberCategory {
    match value.classify() {
        FpCategory::Nan => NumberCategory::Nan,
        FpCategory::Infinite => NumberCategory::Infinite,
        FpCategory::Zero => NumberCategory::Zero,
        FpCategory::Subnormal => NumberCategory::Subnormal,
        FpCategory::Normal => NumberCategory::Normal,
    }
}

/// Classifies the sign of a raw `f64` while keeping `NaN` explicit.
#[must_use]
pub const fn classify_number_sign(value: f64) -> Option<NumberSign> {
    if value.is_nan() {
        None
    } else if matches!(value.classify(), FpCategory::Zero) {
        Some(NumberSign::Zero)
    } else if value.is_sign_negative() {
        Some(NumberSign::Negative)
    } else {
        Some(NumberSign::Positive)
    }
}

/// Returns whether a raw `f64` is finite.
#[must_use]
pub const fn is_finite_number(value: f64) -> bool {
    value.is_finite()
}

#[cfg(test)]
mod tests {
    use super::{
        NumberCategory, NumberSign, classify_number, classify_number_sign, is_finite_number,
    };

    #[test]
    fn classifies_floating_point_categories() {
        assert_eq!(classify_number(f64::NAN), NumberCategory::Nan);
        assert_eq!(classify_number(f64::INFINITY), NumberCategory::Infinite);
        assert_eq!(classify_number(0.0), NumberCategory::Zero);
        assert_eq!(
            classify_number(f64::from_bits(1)),
            NumberCategory::Subnormal
        );
        assert_eq!(classify_number(-4.0), NumberCategory::Normal);
    }

    #[test]
    fn classifies_signs_without_hiding_nan() {
        assert_eq!(classify_number_sign(-12.0), Some(NumberSign::Negative));
        assert_eq!(classify_number_sign(-0.0), Some(NumberSign::Zero));
        assert_eq!(classify_number_sign(18.0), Some(NumberSign::Positive));
        assert_eq!(classify_number_sign(f64::NAN), None);
    }

    #[test]
    fn reports_finiteness() {
        assert!(is_finite_number(1.25));
        assert!(!is_finite_number(f64::NEG_INFINITY));
        assert!(!is_finite_number(f64::NAN));
    }
}
