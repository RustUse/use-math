/// Returns the mathematical floor of `dividend / divisor`.
///
/// This helper follows floor semantics over signed integers instead of Rust's
/// truncating division semantics.
///
/// Returns `None` when `divisor` is zero or when the exact quotient does not
/// fit in `i64`.
///
/// # Examples
///
/// ```rust
/// use use_arithmetic::checked_div_floor;
///
/// assert_eq!(checked_div_floor(-7, 3), Some(-3));
/// assert_eq!(checked_div_floor(7, -3), Some(-3));
/// assert_eq!(checked_div_floor(7, 0), None);
/// ```
#[must_use]
pub fn checked_div_floor(dividend: i64, divisor: i64) -> Option<i64> {
    let quotient = floor_quotient(dividend, divisor)?;

    i64::try_from(quotient).ok()
}

/// Returns the mathematical ceiling of `dividend / divisor`.
///
/// Returns `None` when `divisor` is zero or when the exact quotient does not
/// fit in `i64`.
///
/// # Examples
///
/// ```rust
/// use use_arithmetic::checked_div_ceil;
///
/// assert_eq!(checked_div_ceil(-7, 3), Some(-2));
/// assert_eq!(checked_div_ceil(7, -3), Some(-2));
/// assert_eq!(checked_div_ceil(7, 0), None);
/// ```
#[must_use]
pub fn checked_div_ceil(dividend: i64, divisor: i64) -> Option<i64> {
    let quotient = ceil_quotient(dividend, divisor)?;

    i64::try_from(quotient).ok()
}

/// Returns the mathematical floor-style remainder of `dividend / divisor`.
///
/// The result satisfies `dividend = divisor * div_floor(dividend, divisor) + mod_floor(dividend, divisor)`.
///
/// Returns `None` when `divisor` is zero or when the matching quotient does
/// not fit in `i64`.
///
/// # Examples
///
/// ```rust
/// use use_arithmetic::checked_mod_floor;
///
/// assert_eq!(checked_mod_floor(-7, 3), Some(2));
/// assert_eq!(checked_mod_floor(7, -3), Some(-2));
/// assert_eq!(checked_mod_floor(7, 0), None);
/// ```
#[must_use]
pub fn checked_mod_floor(dividend: i64, divisor: i64) -> Option<i64> {
    let quotient = i128::from(checked_div_floor(dividend, divisor)?);
    let remainder = i128::from(dividend) - (i128::from(divisor) * quotient);

    i64::try_from(remainder).ok()
}

/// Returns the mathematical floor of `dividend / divisor`.
///
/// # Panics
///
/// Panics when `divisor` is zero or when the exact quotient does not fit in
/// `i64`.
#[must_use]
pub fn div_floor(dividend: i64, divisor: i64) -> i64 {
    checked_div_floor(dividend, divisor)
        .unwrap_or_else(|| panic!("div_floor requires a non-zero divisor and an in-range quotient"))
}

/// Returns the mathematical ceiling of `dividend / divisor`.
///
/// # Panics
///
/// Panics when `divisor` is zero or when the exact quotient does not fit in
/// `i64`.
#[must_use]
pub fn div_ceil(dividend: i64, divisor: i64) -> i64 {
    checked_div_ceil(dividend, divisor)
        .unwrap_or_else(|| panic!("div_ceil requires a non-zero divisor and an in-range quotient"))
}

/// Returns the mathematical floor-style remainder of `dividend / divisor`.
///
/// # Panics
///
/// Panics when `divisor` is zero or when the matching quotient does not fit in
/// `i64`.
#[must_use]
pub fn mod_floor(dividend: i64, divisor: i64) -> i64 {
    checked_mod_floor(dividend, divisor)
        .unwrap_or_else(|| panic!("mod_floor requires a non-zero divisor and an in-range quotient"))
}

fn floor_quotient(dividend: i64, divisor: i64) -> Option<i128> {
    if divisor == 0 {
        return None;
    }

    let dividend = i128::from(dividend);
    let divisor = i128::from(divisor);
    let quotient = dividend / divisor;
    let remainder = dividend % divisor;

    if remainder != 0 && ((remainder > 0) != (divisor > 0)) {
        Some(quotient - 1)
    } else {
        Some(quotient)
    }
}

fn ceil_quotient(dividend: i64, divisor: i64) -> Option<i128> {
    if divisor == 0 {
        return None;
    }

    let dividend = i128::from(dividend);
    let divisor = i128::from(divisor);
    let quotient = dividend / divisor;
    let remainder = dividend % divisor;

    if remainder != 0 && ((remainder > 0) == (divisor > 0)) {
        Some(quotient + 1)
    } else {
        Some(quotient)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        checked_div_ceil, checked_div_floor, checked_mod_floor, div_ceil, div_floor, mod_floor,
    };

    #[test]
    fn matches_floor_semantics_for_negative_values() {
        assert_eq!(checked_div_floor(-7, 3), Some(-3));
        assert_eq!(checked_div_floor(7, -3), Some(-3));
        assert_eq!(checked_div_floor(-7, -3), Some(2));
        assert_eq!(checked_div_ceil(-7, 3), Some(-2));
        assert_eq!(checked_div_ceil(7, -3), Some(-2));
        assert_eq!(checked_mod_floor(-7, 3), Some(2));
        assert_eq!(checked_mod_floor(7, -3), Some(-2));
        assert_eq!(div_floor(-7, 3), -3);
        assert_eq!(div_ceil(-7, 3), -2);
        assert_eq!(mod_floor(-7, 3), 2);
    }

    #[test]
    fn rejects_zero_divisors_and_overflowing_quotients() {
        assert_eq!(checked_div_floor(7, 0), None);
        assert_eq!(checked_div_ceil(7, 0), None);
        assert_eq!(checked_mod_floor(7, 0), None);
        assert_eq!(checked_div_floor(i64::MIN, -1), None);
        assert_eq!(checked_div_ceil(i64::MIN, -1), None);
        assert_eq!(checked_mod_floor(i64::MIN, -1), None);
    }

    #[test]
    #[should_panic(expected = "div_floor requires a non-zero divisor and an in-range quotient")]
    fn plain_div_floor_panics_on_zero_divisor() {
        let _ = div_floor(7, 0);
    }

    #[test]
    #[should_panic(expected = "div_ceil requires a non-zero divisor and an in-range quotient")]
    fn plain_div_ceil_panics_on_zero_divisor() {
        let _ = div_ceil(7, 0);
    }

    #[test]
    #[should_panic(expected = "mod_floor requires a non-zero divisor and an in-range quotient")]
    fn plain_mod_floor_panics_on_zero_divisor() {
        let _ = mod_floor(7, 0);
    }
}
