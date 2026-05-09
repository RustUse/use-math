use crate::error::IntegerError;

/// Sign classification for a signed integer value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntegerSign {
    Negative,
    Zero,
    Positive,
}

/// Classifies a signed integer as negative, zero, or positive.
#[must_use]
pub const fn classify_sign(value: i128) -> IntegerSign {
    if value < 0 {
        IntegerSign::Negative
    } else if value > 0 {
        IntegerSign::Positive
    } else {
        IntegerSign::Zero
    }
}

/// Returns whether an integer is evenly divisible by two.
#[must_use]
pub const fn is_even(value: i128) -> bool {
    value % 2 == 0
}

/// Returns whether an integer leaves a remainder of one when divided by two.
#[must_use]
pub const fn is_odd(value: i128) -> bool {
    !is_even(value)
}

/// Returns whether `value` is evenly divisible by `divisor`.
///
/// # Errors
///
/// Returns [`IntegerError::DivisionByZero`] when `divisor` is zero.
pub const fn is_divisible_by(value: i128, divisor: i128) -> Result<bool, IntegerError> {
    if divisor == 0 {
        Err(IntegerError::DivisionByZero)
    } else {
        Ok(value % divisor == 0)
    }
}

/// Computes the non-negative greatest common divisor of two signed integers.
#[must_use]
pub const fn gcd(left: i128, right: i128) -> u128 {
    gcd_u128(left.unsigned_abs(), right.unsigned_abs())
}

/// Returns whether two integers share no positive common divisor other than one.
#[must_use]
pub const fn are_coprime(left: i128, right: i128) -> bool {
    gcd(left, right) == 1
}

/// Computes the non-negative least common multiple of two signed integers.
///
/// # Errors
///
/// Returns [`IntegerError::ArithmeticOverflow`] when the least common multiple does not fit in `u128`.
pub const fn lcm(left: i128, right: i128) -> Result<u128, IntegerError> {
    if left == 0 || right == 0 {
        return Ok(0);
    }

    let divisor = gcd(left, right);
    let scaled = left.unsigned_abs() / divisor;

    match scaled.checked_mul(right.unsigned_abs()) {
        Some(result) => Ok(result),
        None => Err(IntegerError::ArithmeticOverflow { operation: "lcm" }),
    }
}

const fn gcd_u128(mut left: u128, mut right: u128) -> u128 {
    while right != 0 {
        let remainder = left % right;
        left = right;
        right = remainder;
    }

    left
}

#[cfg(test)]
mod tests {
    use super::{IntegerSign, are_coprime, classify_sign, gcd, is_divisible_by, is_even, is_odd, lcm};
    use crate::IntegerError;

    #[test]
    fn classifies_sign_and_parity() {
        assert_eq!(classify_sign(-1), IntegerSign::Negative);
        assert_eq!(classify_sign(0), IntegerSign::Zero);
        assert_eq!(classify_sign(1), IntegerSign::Positive);
        assert!(is_even(-8));
        assert!(is_odd(-7));
    }

    #[test]
    fn computes_divisibility_and_common_divisors() -> Result<(), IntegerError> {
        assert!(is_divisible_by(81, 9)?);
        assert!(!is_divisible_by(81, 8)?);
        assert_eq!(gcd(-54, 24), 6);
        assert_eq!(gcd(0, 0), 0);
        assert_eq!(lcm(-6, 15)?, 30);
        assert_eq!(lcm(0, 15)?, 0);
        assert!(are_coprime(35, 64));

        Ok(())
    }

    #[test]
    fn rejects_invalid_divisors_and_reports_overflow() {
        assert_eq!(is_divisible_by(10, 0), Err(IntegerError::DivisionByZero));
        assert!(matches!(
            lcm(i128::MAX, i128::MAX - 1),
            Err(IntegerError::ArithmeticOverflow { operation: "lcm" })
        ));
    }
}