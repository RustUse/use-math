use crate::error::SeriesError;

/// Returns the zero-based `index`th term of an arithmetic progression.
///
/// # Errors
///
/// Returns [`SeriesError::Overflow`] when the resulting term does not fit in
/// `i128`.
///
/// # Examples
///
/// ```rust
/// use use_series::arithmetic_nth_term;
///
/// assert_eq!(arithmetic_nth_term(3, 2, 4)?, 11);
/// # Ok::<(), use_series::SeriesError>(())
/// ```
pub const fn arithmetic_nth_term(first: i128, step: i128, index: u64) -> Result<i128, SeriesError> {
    let Some(offset) = step.checked_mul(index as i128) else {
        return Err(SeriesError::Overflow {
            operation: "arithmetic_nth_term",
        });
    };

    match first.checked_add(offset) {
        Some(value) => Ok(value),
        None => Err(SeriesError::Overflow {
            operation: "arithmetic_nth_term",
        }),
    }
}

/// Returns the sum of the first `terms` values of an arithmetic progression.
///
/// # Errors
///
/// Returns [`SeriesError::Overflow`] when the partial sum does not fit in
/// `i128`.
///
/// # Examples
///
/// ```rust
/// use use_series::arithmetic_sum;
///
/// assert_eq!(arithmetic_sum(3, 2, 5)?, 35);
/// # Ok::<(), use_series::SeriesError>(())
/// ```
pub const fn arithmetic_sum(first: i128, step: i128, terms: u64) -> Result<i128, SeriesError> {
    let mut sum = 0_i128;
    let mut index = 0_u64;

    while index < terms {
        let Ok(term) = arithmetic_nth_term(first, step, index) else {
            return Err(SeriesError::Overflow {
                operation: "arithmetic_sum",
            });
        };

        sum = match sum.checked_add(term) {
            Some(value) => value,
            None => {
                return Err(SeriesError::Overflow {
                    operation: "arithmetic_sum",
                });
            },
        };

        index += 1;
    }

    Ok(sum)
}

/// Returns the zero-based `index`th term of a geometric progression.
///
/// # Errors
///
/// Returns [`SeriesError::Overflow`] when the resulting term does not fit in
/// `i128`.
///
/// # Examples
///
/// ```rust
/// use use_series::geometric_nth_term;
///
/// assert_eq!(geometric_nth_term(2, 3, 4)?, 162);
/// # Ok::<(), use_series::SeriesError>(())
/// ```
pub const fn geometric_nth_term(first: i128, ratio: i128, index: u64) -> Result<i128, SeriesError> {
    let mut value = first;
    let mut exponent = 0_u64;

    while exponent < index {
        value = match value.checked_mul(ratio) {
            Some(next) => next,
            None => {
                return Err(SeriesError::Overflow {
                    operation: "geometric_nth_term",
                });
            },
        };

        exponent += 1;
    }

    Ok(value)
}

/// Returns the sum of the first `terms` values of a geometric progression.
///
/// # Errors
///
/// Returns [`SeriesError::Overflow`] when the partial sum does not fit in
/// `i128`.
///
/// # Examples
///
/// ```rust
/// use use_series::geometric_sum;
///
/// assert_eq!(geometric_sum(2, 3, 4)?, 80);
/// # Ok::<(), use_series::SeriesError>(())
/// ```
pub const fn geometric_sum(first: i128, ratio: i128, terms: u64) -> Result<i128, SeriesError> {
    let mut sum = 0_i128;
    let mut term = first;
    let mut index = 0_u64;

    while index < terms {
        sum = match sum.checked_add(term) {
            Some(value) => value,
            None => {
                return Err(SeriesError::Overflow {
                    operation: "geometric_sum",
                });
            },
        };

        term = match term.checked_mul(ratio) {
            Some(next) => next,
            None if index + 1 == terms => term,
            None => {
                return Err(SeriesError::Overflow {
                    operation: "geometric_sum",
                });
            },
        };

        index += 1;
    }

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::{arithmetic_nth_term, arithmetic_sum, geometric_nth_term, geometric_sum};
    use crate::SeriesError;

    #[test]
    fn computes_arithmetic_progressions() {
        assert_eq!(arithmetic_nth_term(3, 2, 4), Ok(11));
        assert_eq!(arithmetic_sum(3, 2, 5), Ok(35));
        assert_eq!(arithmetic_sum(10, -3, 4), Ok(22));
        assert_eq!(arithmetic_sum(8, 5, 0), Ok(0));
    }

    #[test]
    fn computes_geometric_progressions() {
        assert_eq!(geometric_nth_term(2, 3, 4), Ok(162));
        assert_eq!(geometric_sum(2, 3, 4), Ok(80));
        assert_eq!(geometric_sum(5, 1, 4), Ok(20));
        assert_eq!(geometric_sum(4, -2, 4), Ok(-20));
        assert_eq!(geometric_sum(7, 3, 0), Ok(0));
    }

    #[test]
    fn reports_overflow() {
        assert!(matches!(
            arithmetic_nth_term(i128::MAX, 1, 1),
            Err(SeriesError::Overflow {
                operation: "arithmetic_nth_term"
            })
        ));
        assert!(matches!(
            arithmetic_sum(i128::MAX, 1, 2),
            Err(SeriesError::Overflow {
                operation: "arithmetic_sum"
            })
        ));
        assert!(matches!(
            geometric_nth_term(i128::MAX, 2, 1),
            Err(SeriesError::Overflow {
                operation: "geometric_nth_term"
            })
        ));
        assert!(matches!(
            geometric_sum(i128::MAX, 2, 2),
            Err(SeriesError::Overflow {
                operation: "geometric_sum"
            })
        ));
    }
}
