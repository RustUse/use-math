/// Returns whether `value` is evenly divisible by two.
///
/// # Examples
///
/// ```rust
/// use use_arithmetic::is_even;
///
/// assert!(is_even(12));
/// assert!(!is_even(7));
/// ```
#[must_use]
pub const fn is_even(value: u64) -> bool {
    value.is_multiple_of(2)
}

/// Returns whether `value` leaves a remainder of one when divided by two.
///
/// # Examples
///
/// ```rust
/// use use_arithmetic::is_odd;
///
/// assert!(is_odd(7));
/// assert!(!is_odd(12));
/// ```
#[must_use]
pub const fn is_odd(value: u64) -> bool {
    !is_even(value)
}

#[cfg(test)]
mod tests {
    use super::{is_even, is_odd};

    #[test]
    fn classifies_zero_and_positive_values() {
        assert!(is_even(0));
        assert!(is_even(12));
        assert!(!is_even(7));
        assert!(is_odd(7));
        assert!(!is_odd(12));
    }
}
