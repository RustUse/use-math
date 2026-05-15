/// Computes the greatest common divisor of two unsigned integers.
///
/// This helper uses the Euclidean algorithm and keeps the required edge cases
/// explicit: `gcd(0, 0) == 0` and `gcd(a, 0) == a`.
///
/// # Examples
///
/// ```rust
/// use use_arithmetic::gcd;
///
/// assert_eq!(gcd(54, 24), 6);
/// assert_eq!(gcd(0, 0), 0);
/// assert_eq!(gcd(21, 0), 21);
/// ```
#[must_use]
pub const fn gcd(mut left: u64, mut right: u64) -> u64 {
    while right != 0 {
        let remainder = left % right;
        left = right;
        right = remainder;
    }

    left
}

#[cfg(test)]
mod tests {
    use super::gcd;

    #[test]
    fn handles_zero_and_positive_cases() {
        assert_eq!(gcd(0, 0), 0);
        assert_eq!(gcd(21, 0), 21);
        assert_eq!(gcd(0, 21), 21);
        assert_eq!(gcd(54, 24), 6);
        assert_eq!(gcd(48, 18), 6);
    }
}
