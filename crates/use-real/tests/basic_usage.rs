use use_real::{Real, RealError, RealInterval, approx_eq};

fn assert_close(left: f64, right: f64, tolerance: f64) {
    assert!(
        (left - right).abs() <= tolerance,
        "expected {left} to be within {tolerance} of {right}"
    );
}

#[test]
fn direct_real_usage_covers_finite_values_and_intervals() -> Result<(), RealError> {
    let value = Real::try_new(-3.5)?;
    let interval = RealInterval::try_new(-2.0, 6.0)?;
    let midpoint = interval.midpoint();
    let clamped = interval.clamp(value.abs());

    assert_close(midpoint.value(), 2.0, 1.0e-12);
    assert!(interval.contains(Real::try_new(1.5)?));
    assert!(!interval.contains(Real::try_new(7.0)?));
    assert_eq!(clamped, Real::try_new(3.5)?);
    assert!(approx_eq(midpoint, Real::try_new(2.0)?, 1.0e-12)?);

    Ok(())
}

#[test]
fn real_validation_rejects_invalid_inputs() {
    assert!(matches!(
        Real::try_new(f64::NAN),
        Err(RealError::NonFiniteValue { .. })
    ));
    assert!(matches!(
        RealInterval::try_new(2.0, -2.0),
        Err(RealError::InvalidInterval { .. })
    ));
}
