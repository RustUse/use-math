use use_series::{
    SeriesError, arithmetic_nth_term, arithmetic_sum, geometric_nth_term, geometric_sum,
};

#[test]
fn direct_series_usage_covers_progressions() -> Result<(), SeriesError> {
    assert_eq!(arithmetic_nth_term(3, 2, 4)?, 11);
    assert_eq!(arithmetic_sum(3, 2, 5)?, 35);
    assert_eq!(arithmetic_sum(10, -3, 4)?, 22);
    assert_eq!(geometric_nth_term(2, 3, 4)?, 162);
    assert_eq!(geometric_sum(2, 3, 4)?, 80);
    assert_eq!(geometric_sum(5, 1, 4)?, 20);

    Ok(())
}

#[test]
fn series_validation_reports_overflow() {
    assert!(matches!(
        arithmetic_nth_term(i128::MAX, 1, 1),
        Err(SeriesError::Overflow {
            operation: "arithmetic_nth_term"
        })
    ));
    assert!(matches!(
        geometric_sum(i128::MAX, 2, 2),
        Err(SeriesError::Overflow {
            operation: "geometric_sum"
        })
    ));
}
