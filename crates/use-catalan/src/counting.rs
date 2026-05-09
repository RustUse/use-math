use crate::error::CatalanError;

const fn gcd(mut left: u128, mut right: u128) -> u128 {
    while right != 0 {
        let remainder = left % right;
        left = right;
        right = remainder;
    }

    left
}

/// Returns the `n`th Catalan number using checked `u128` arithmetic.
///
/// # Errors
///
/// Returns [`CatalanError::CatalanOverflow`] when the result no longer fits in
/// `u128`.
///
/// # Examples
///
/// ```rust
/// use use_catalan::catalan;
///
/// assert_eq!(catalan(4)?, 14);
/// # Ok::<(), use_catalan::CatalanError>(())
/// ```
pub fn catalan(n: u64) -> Result<u128, CatalanError> {
    if n <= 1 {
        return Ok(1);
    }

    let mut result = 1_u128;
    let mut step = 2_u64;

    while step <= n {
        let mut numerator = u128::from(n) + u128::from(step);
        let mut denominator = u128::from(step);

        let numerator_gcd = gcd(numerator, denominator);
        numerator /= numerator_gcd;
        denominator /= numerator_gcd;

        let result_gcd = gcd(result, denominator);
        result /= result_gcd;
        denominator /= result_gcd;

        debug_assert_eq!(denominator, 1);

        result = match result.checked_mul(numerator) {
            Some(value) => value,
            None => return Err(CatalanError::CatalanOverflow(n)),
        };

        step += 1;
    }

    Ok(result)
}

/// Returns the `n`th Fuss-Catalan number for a positive `order`.
///
/// `order = 2` matches the standard Catalan sequence.
///
/// # Errors
///
/// Returns [`CatalanError::ZeroOrder`] when `order == 0`.
///
/// Returns [`CatalanError::FussCatalanOverflow`] when the result no longer fits
/// in `u128`.
///
/// # Examples
///
/// ```rust
/// use use_catalan::fuss_catalan;
///
/// assert_eq!(fuss_catalan(3, 3)?, 12);
/// # Ok::<(), use_catalan::CatalanError>(())
/// ```
pub fn fuss_catalan(order: u64, n: u64) -> Result<u128, CatalanError> {
    if order == 0 {
        return Err(CatalanError::ZeroOrder);
    }

    if n == 0 || order == 1 {
        return Ok(1);
    }

    let width = u128::from(order - 1) * u128::from(n);
    let mut remaining_divisor = width + 1;
    let mut result = 1_u128;
    let mut step = 1_u64;

    while step <= n {
        let mut numerator = width + u128::from(step);
        let mut denominator = u128::from(step);

        let numerator_gcd = gcd(numerator, denominator);
        numerator /= numerator_gcd;
        denominator /= numerator_gcd;

        let result_gcd = gcd(result, denominator);
        result /= result_gcd;
        denominator /= result_gcd;

        let divisor_numerator_gcd = gcd(numerator, remaining_divisor);
        numerator /= divisor_numerator_gcd;
        remaining_divisor /= divisor_numerator_gcd;

        let divisor_result_gcd = gcd(result, remaining_divisor);
        result /= divisor_result_gcd;
        remaining_divisor /= divisor_result_gcd;

        debug_assert_eq!(denominator, 1);

        result = match result.checked_mul(numerator) {
            Some(value) => value,
            None => return Err(CatalanError::FussCatalanOverflow { order, n }),
        };

        step += 1;
    }

    debug_assert_eq!(remaining_divisor, 1);

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::{catalan, fuss_catalan};
    use crate::error::CatalanError;

    #[test]
    fn computes_catalan_numbers() {
        assert_eq!(catalan(0), Ok(1));
        assert_eq!(catalan(1), Ok(1));
        assert_eq!(catalan(4), Ok(14));
        assert_eq!(catalan(10), Ok(16_796));
    }

    #[test]
    fn computes_fuss_catalan_numbers() {
        assert_eq!(fuss_catalan(1, 5), Ok(1));
        assert_eq!(fuss_catalan(2, 4), Ok(14));
        assert_eq!(fuss_catalan(3, 3), Ok(12));
        assert_eq!(fuss_catalan(4, 2), Ok(4));
    }

    #[test]
    fn rejects_invalid_orders_and_reports_overflow() {
        assert_eq!(fuss_catalan(0, 3), Err(CatalanError::ZeroOrder));
        assert!(matches!(
            catalan(100),
            Err(CatalanError::CatalanOverflow(100))
        ));
        assert!(matches!(
            fuss_catalan(5, 60),
            Err(CatalanError::FussCatalanOverflow { order: 5, n: 60 })
        ));
    }
}
