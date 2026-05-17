use use_math::{collatz, prelude::*};

#[test]
fn facade_prelude_exposes_collatz_workflow() {
    assert_eq!(collatz_sequence(6), Some(vec![6, 3, 10, 5, 16, 8, 4, 2, 1]));
    assert_eq!(total_stopping_time(6), Some(8));
    assert_eq!(trajectory_len(6), Some(9));
    assert_eq!(max_value_in_trajectory(7), Some(52));
    assert_eq!(parity_vector(1), Some(vec![]));

    let summary = verify_range(1, 10);

    assert_eq!(summary.checked, 10);
    assert_eq!(summary.max_total_stopping_time, Some((9, 19)));
    assert_eq!(summary.max_trajectory_value, Some((7, 52)));
}

#[test]
fn facade_root_and_namespace_reexports_collatz_workflow() {
    assert_eq!(use_math::stopping_time(3), Some(6));
    assert!(use_math::reaches_one(6));
    assert_eq!(use_math::parity(8), Some(use_math::CollatzParity::Even));
    assert_eq!(collatz::parity(9), Some(collatz::CollatzParity::Odd));
    assert_eq!(collatz::collatz_next(1), Some(4));
    assert_eq!(
        collatz::verify_range(1, 10).max_trajectory_value,
        Some((7, 52))
    );
}
