/// Computes the binomial coefficient C(2n, n) using checked integer arithmetic.
///
/// Returns `None` on overflow.
fn checked_binomial_2n_n(n: u32) -> Option<u128> {
    // C(2n, n) = product_{i=1}^{n} (n + i) / i
    // We cancel factors incrementally to stay within u128 as long as possible.
    let mut result: u128 = 1;

    for i in 1_u32..=n {
        let numerator = u128::from(n + i);
        let denominator = u128::from(i);

        // Cancel the GCD of result and denominator first, then of numerator and
        // the remaining denominator, so that we keep intermediate values small.
        let g1 = gcd(result, denominator);
        result /= g1;
        let d1 = denominator / g1;

        let g2 = gcd(numerator, d1);
        let num = numerator / g2;
        // d1 / g2 must be 1 at this point for the exact binomial coefficient.
        debug_assert_eq!(d1 / g2, 1);

        result = result.checked_mul(num)?;
    }

    Some(result)
}

const fn gcd(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

/// Returns the n-th Catalan number, or `None` if the result overflows `u128`.
///
/// Uses the recurrence relation
/// `C(n+1) = C(n) * 2 * (2n + 1) / (n + 2)`
/// with fully checked integer arithmetic.
///
/// # Examples
///
/// ```rust
/// use use_catalan::catalan;
///
/// assert_eq!(catalan(0), Some(1));
/// assert_eq!(catalan(5), Some(42));
/// assert_eq!(catalan(10), Some(16796));
/// ```
#[must_use]
pub fn catalan(n: u32) -> Option<u128> {
    let mut c: u128 = 1; // C(0) = 1

    for k in 0..n {
        // C(k+1) = C(k) * 2 * (2k+1) / (k+2)
        // The numerator factor is 2*(2k+1) and the denominator is (k+2).
        // Because C(k+1) is always an integer, the division is exact.
        // We compute it as: c = c * 2 * (2k+1) / (k+2).
        // To avoid overflow, cancel the GCD between c and (k+2) before
        // multiplying by the numerator factor.
        let denom = u128::from(k + 2);
        let num_factor = u128::from(2 * (2 * k + 1));

        let g = gcd(c, denom);
        c /= g;
        let reduced_denom = denom / g;

        // reduced_denom must divide num_factor exactly.
        let g2 = gcd(num_factor, reduced_denom);
        let reduced_num = num_factor / g2;
        debug_assert_eq!(reduced_denom / g2, 1);

        c = c.checked_mul(reduced_num)?;
    }

    Some(c)
}

/// Returns the n-th Catalan number computed via the closed-form binomial formula
/// `C(n) = C(2n, n) / (n + 1)`, using checked integer arithmetic.
///
/// Returns `None` if any intermediate or final value overflows `u128`.
///
/// # Examples
///
/// ```rust
/// use use_catalan::catalan_by_binomial;
///
/// assert_eq!(catalan_by_binomial(0), Some(1));
/// assert_eq!(catalan_by_binomial(5), Some(42));
/// ```
#[must_use]
pub fn catalan_by_binomial(n: u32) -> Option<u128> {
    let binom = checked_binomial_2n_n(n)?;
    let denom = u128::from(n + 1);
    // Division is exact because C(2n,n)/(n+1) is always an integer.
    Some(binom / denom)
}

/// Returns the n-th Catalan number using the direct recurrence
/// `C(n+1) = C(n) * 2 * (2n + 1) / (n + 2)` starting from `C(0) = 1`.
///
/// This is an iterative implementation of the recurrence, not a naive
/// exponential recursion.  Returns `None` on overflow.
///
/// # Examples
///
/// ```rust
/// use use_catalan::catalan_recursive;
///
/// assert_eq!(catalan_recursive(0), Some(1));
/// assert_eq!(catalan_recursive(5), Some(42));
/// ```
#[must_use]
pub fn catalan_recursive(n: u32) -> Option<u128> {
    // Delegate to the same recurrence used by `catalan`; expose a separate
    // public function so callers and tests can compare the two entry points.
    catalan(n)
}

/// Returns the first `count` Catalan numbers as a `Vec<u128>`, or `None` if
/// any value in the sequence overflows `u128`.
///
/// `catalan_sequence(0)` returns `Some(vec![])`.
///
/// # Examples
///
/// ```rust
/// use use_catalan::catalan_sequence;
///
/// assert_eq!(catalan_sequence(0), Some(vec![]));
/// assert_eq!(catalan_sequence(6), Some(vec![1, 1, 2, 5, 14, 42]));
/// ```
#[must_use]
pub fn catalan_sequence(count: usize) -> Option<Vec<u128>> {
    let mut seq = Vec::with_capacity(count);
    let mut c: u128 = 1;

    for k in 0..count {
        seq.push(c);

        if k + 1 < count {
            // advance to C(k+1)
            let k32 = u32::try_from(k).ok()?;
            let denom = u128::from(k32 + 2);
            let num_factor = u128::from(2 * (2 * k32 + 1));

            let g = gcd(c, denom);
            c /= g;
            let reduced_denom = denom / g;

            let g2 = gcd(num_factor, reduced_denom);
            let reduced_num = num_factor / g2;
            debug_assert_eq!(reduced_denom / g2, 1);

            c = c.checked_mul(reduced_num)?;
        }
    }

    Some(seq)
}

/// Returns `true` if `value` is a Catalan number.
///
/// Uses the recurrence to generate Catalan numbers up to `value` and checks
/// for an exact match.
///
/// # Examples
///
/// ```rust
/// use use_catalan::is_catalan_number;
///
/// assert!(is_catalan_number(1));
/// assert!(is_catalan_number(42));
/// assert!(!is_catalan_number(43));
/// ```
#[must_use]
pub fn is_catalan_number(value: u128) -> bool {
    let mut c: u128 = 1;
    let mut k: u32 = 0;

    loop {
        if c == value {
            return true;
        }
        if c > value {
            return false;
        }

        // Advance: C(k+1) = C(k) * 2*(2k+1) / (k+2)
        let denom = u128::from(k + 2);
        let num_factor = u128::from(2 * (2 * k + 1));

        let g = gcd(c, denom);
        c /= g;
        let reduced_denom = denom / g;

        let g2 = gcd(num_factor, reduced_denom);
        let reduced_num = num_factor / g2;
        debug_assert_eq!(reduced_denom / g2, 1);

        match c.checked_mul(reduced_num) {
            Some(next) => c = next,
            None => return false,
        }

        k += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::{
        catalan, catalan_by_binomial, catalan_recursive, catalan_sequence, is_catalan_number,
    };

    const KNOWN: &[(u32, u128)] = &[
        (0, 1),
        (1, 1),
        (2, 2),
        (3, 5),
        (4, 14),
        (5, 42),
        (6, 132),
        (7, 429),
        (8, 1430),
        (9, 4862),
        (10, 16796),
    ];

    #[test]
    fn catalan_matches_known_values() {
        for &(n, expected) in KNOWN {
            assert_eq!(catalan(n), Some(expected), "catalan({n})");
        }
    }

    #[test]
    fn catalan_by_binomial_matches_recurrence() {
        for &(n, expected) in KNOWN {
            assert_eq!(
                catalan_by_binomial(n),
                Some(expected),
                "catalan_by_binomial({n})"
            );
        }
    }

    #[test]
    fn catalan_recursive_matches_recurrence() {
        for &(n, expected) in KNOWN {
            assert_eq!(
                catalan_recursive(n),
                Some(expected),
                "catalan_recursive({n})"
            );
        }
    }

    #[test]
    fn catalan_sequence_zero() {
        assert_eq!(catalan_sequence(0), Some(vec![]));
    }

    #[test]
    fn catalan_sequence_first_six() {
        assert_eq!(catalan_sequence(6), Some(vec![1, 1, 2, 5, 14, 42]));
    }

    #[test]
    fn catalan_sequence_first_eleven() {
        let seq = catalan_sequence(11).expect("sequence should not overflow");
        let expected: Vec<u128> = KNOWN.iter().map(|&(_, v)| v).collect();
        assert_eq!(seq, expected);
    }

    #[test]
    fn is_catalan_number_true_for_known() {
        for &(_, v) in KNOWN {
            assert!(is_catalan_number(v), "{v} should be Catalan");
        }
    }

    #[test]
    fn is_catalan_number_false_for_non_catalan() {
        for non in [0, 3, 4, 6, 7, 8, 9, 10, 11, 43, 100, 1431] {
            assert!(!is_catalan_number(non), "{non} should not be Catalan");
        }
    }

    #[test]
    fn large_n_returns_none_on_overflow() {
        // C(70) overflows u128 (C(69) is the last that fits).
        assert!(catalan(70).is_none());
    }

    #[test]
    fn catalan_by_binomial_overflow_returns_none() {
        assert!(catalan_by_binomial(70).is_none());
    }
}
