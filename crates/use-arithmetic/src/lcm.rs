use crate::gcd;

/// Computes the least common multiple of two unsigned integers.
///
/// Returns `None` when the exact result does not fit in `u64`.
///
/// # Examples
///
/// ```rust
/// use use_arithmetic::checked_lcm;
///
/// assert_eq!(checked_lcm(6, 15), Some(30));
/// assert_eq!(checked_lcm(0, 15), Some(0));
/// ```
#[must_use]
pub const fn checked_lcm(left: u64, right: u64) -> Option<u64> {
    if left == 0 || right == 0 {
        return Some(0);
    }

    let divisor = gcd(left, right);
    (left / divisor).checked_mul(right)
}

/// Computes the least common multiple of two unsigned integers.
///
/// `lcm(a, 0)` returns `0`.
///
/// # Panics
///
/// Panics when the exact least common multiple does not fit in `u64`.
///
/// # Examples
///
/// ```rust
/// use use_arithmetic::lcm;
///
/// assert_eq!(lcm(6, 15), 30);
/// assert_eq!(lcm(0, 15), 0);
/// ```
#[must_use]
pub fn lcm(left: u64, right: u64) -> u64 {
    checked_lcm(left, right).unwrap_or_else(|| panic!("lcm overflowed u64"))
}

#[cfg(test)]
mod tests {
    use super::{checked_lcm, lcm};

    #[test]
    fn handles_zero_and_positive_cases() {
        assert_eq!(checked_lcm(0, 0), Some(0));
        assert_eq!(checked_lcm(0, 15), Some(0));
        assert_eq!(checked_lcm(6, 15), Some(30));
        assert_eq!(lcm(21, 6), 42);
    }

    #[test]
    fn reports_overflow_through_checked_variant() {
        assert_eq!(checked_lcm(u64::MAX, u64::MAX - 1), None);
    }

    #[test]
    #[should_panic(expected = "lcm overflowed u64")]
    fn plain_lcm_panics_on_overflow() {
        let _ = lcm(u64::MAX, u64::MAX - 1);
    }
}
