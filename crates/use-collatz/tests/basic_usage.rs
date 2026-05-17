use use_collatz::{
    CollatzParity, collatz_sequence, max_value_in_trajectory, parity_vector, verify_range,
};

#[test]
fn supports_basic_collatz_workflows() {
    assert_eq!(collatz_sequence(7).map(|sequence| sequence.len()), Some(17));
    assert_eq!(max_value_in_trajectory(7), Some(52));
    assert_eq!(
        parity_vector(7).map(|parities| parities[0]),
        Some(CollatzParity::Odd)
    );

    let summary = verify_range(1, 10);

    assert_eq!(summary.checked, 10);
    assert_eq!(summary.max_total_stopping_time, Some((9, 19)));
    assert_eq!(summary.max_trajectory_value, Some((7, 52)));
}
