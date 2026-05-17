use use_math::{collatz_sequence, parity_vector, total_stopping_time, verify_range};

fn main() {
    assert_eq!(collatz_sequence(6), Some(vec![6, 3, 10, 5, 16, 8, 4, 2, 1]));
    assert_eq!(total_stopping_time(6), Some(8));
    assert_eq!(parity_vector(1), Some(vec![]));
    assert_eq!(use_math::collatz::max_value_in_trajectory(7), Some(52));

    let summary = verify_range(1, 10);

    assert_eq!(summary.max_total_stopping_time, Some((9, 19)));
    assert_eq!(summary.max_trajectory_value, Some((7, 52)));
}
