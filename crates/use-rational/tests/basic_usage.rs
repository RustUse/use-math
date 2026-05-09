use use_rational::{Rational, RationalError};

fn assert_close(left: f64, right: f64, tolerance: f64) {
    assert!(
        (left - right).abs() <= tolerance,
        "expected {left} to be within {tolerance} of {right}"
    );
}

#[test]
fn direct_rational_usage_covers_normalization_and_arithmetic() -> Result<(), RationalError> {
    let half = Rational::try_new(2, 4)?;
    let third = Rational::try_new(1, 3)?;

    assert_eq!(half, Rational::try_new(1, 2)?);
    assert_eq!(half.checked_add(third)?, Rational::try_new(5, 6)?);
    assert_eq!(half.checked_sub(third)?, Rational::try_new(1, 6)?);
    assert_eq!(half.checked_mul(third)?, Rational::try_new(1, 6)?);
    assert_eq!(half.checked_div(third)?, Rational::try_new(3, 2)?);
    assert_eq!(half.reciprocal()?, Rational::try_new(2, 1)?);
    assert_close(half.as_f64(), 0.5, 1.0e-12);

    Ok(())
}

#[test]
fn rational_validation_rejects_invalid_inputs() {
    assert!(matches!(
        Rational::try_new(1, 0),
        Err(RationalError::ZeroDenominator)
    ));
    assert!(matches!(
        Rational::zero().reciprocal(),
        Err(RationalError::DivisionByZero)
    ));
}
