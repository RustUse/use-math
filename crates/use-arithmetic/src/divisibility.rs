/// Returns whether `value` is evenly divisible by `divisor`.
///
/// This plain helper returns `false` when `divisor` is zero.
/// Use [`checked_is_divisible_by`] when zero divisors should stay explicit.
///
/// # Examples
///
/// ```rust
/// use use_arithmetic::is_divisible_by;
///
/// assert!(is_divisible_by(84, 7));
/// assert!(!is_divisible_by(84, 0));
/// ```
#[must_use]
pub const fn is_divisible_by(value: u64, divisor: u64) -> bool {
    divisor != 0 && value.is_multiple_of(divisor)
}

/// Returns whether `value` is evenly divisible by `divisor`.
///
/// Returns `None` when `divisor` is zero.
///
/// # Examples
///
/// ```rust
/// use use_arithmetic::checked_is_divisible_by;
///
/// assert_eq!(checked_is_divisible_by(84, 7), Some(true));
/// assert_eq!(checked_is_divisible_by(84, 0), None);
/// ```
#[must_use]
pub const fn checked_is_divisible_by(value: u64, divisor: u64) -> Option<bool> {
    if divisor == 0 {
        None
    } else {
        Some(value.is_multiple_of(divisor))
    }
}

#[cfg(test)]
mod tests {
    use super::{checked_is_divisible_by, is_divisible_by};

    #[test]
    fn handles_positive_and_zero_divisors() {
        assert!(is_divisible_by(81, 9));
        assert!(!is_divisible_by(81, 8));
        assert!(!is_divisible_by(81, 0));
        assert_eq!(checked_is_divisible_by(81, 9), Some(true));
        assert_eq!(checked_is_divisible_by(81, 8), Some(false));
        assert_eq!(checked_is_divisible_by(81, 0), None);
    }
}
