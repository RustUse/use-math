use crate::error::CatalanError;

const fn gcd(mut left: u128, mut right: u128) -> u128 {
    while right != 0 {
        let remainder = left % right;
        left = right;
        right = remainder;
    }

    left
}

fn checked_binomial(total: u128, choose: u64) -> Option<u128> {
    let choose = choose.min(u64::try_from(total.saturating_sub(u128::from(choose))).ok()?);
    let mut result = 1_u128;
    let mut step = 1_u64;

    while step <= choose {
        let mut numerator = total - u128::from(choose) + u128::from(step);
        let mut denominator = u128::from(step);

        let numerator_gcd = gcd(numerator, denominator);
        numerator /= numerator_gcd;
        denominator /= numerator_gcd;

        let result_gcd = gcd(result, denominator);
        result /= result_gcd;
        denominator /= result_gcd;

        debug_assert_eq!(denominator, 1);

        result = result.checked_mul(numerator)?;
        step += 1;
    }

    Some(result)
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
    match fuss_catalan(2, n) {
        Ok(value) => Ok(value),
        Err(CatalanError::FussCatalanOverflow { .. }) => Err(CatalanError::CatalanOverflow(n)),
        Err(CatalanError::ZeroOrder | CatalanError::CatalanOverflow(_)) => {
            unreachable!("fuss_catalan(2, n) only reports overflow or success")
        },
    }
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

    let total = u128::from(order) * u128::from(n);
    let divisor = u128::from(order - 1) * u128::from(n) + 1;
    let binomial =
        checked_binomial(total, n).ok_or(CatalanError::FussCatalanOverflow { order, n })?;

    if binomial % divisor != 0 {
        return Err(CatalanError::FussCatalanOverflow { order, n });
    }

    Ok(binomial / divisor)
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
