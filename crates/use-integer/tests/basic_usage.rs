use use_integer::{
    IntegerError, IntegerSign, are_coprime, classify_sign, gcd, is_divisible_by, is_even,
    is_odd, lcm,
};

#[test]
fn direct_integer_usage_covers_classification_and_helpers() -> Result<(), IntegerError> {
    assert_eq!(classify_sign(-5), IntegerSign::Negative);
    assert_eq!(classify_sign(0), IntegerSign::Zero);
    assert_eq!(classify_sign(9), IntegerSign::Positive);
    assert!(is_even(12));
    assert!(is_odd(7));
    assert!(is_divisible_by(84, 7)?);
    assert_eq!(gcd(-54, 24), 6);
    assert_eq!(lcm(-6, 15)?, 30);
    assert!(are_coprime(35, 64));

    Ok(())
}

#[test]
fn integer_validation_rejects_invalid_divisors_and_reports_overflow() {
    assert_eq!(is_divisible_by(10, 0), Err(IntegerError::DivisionByZero));
    assert!(matches!(
        lcm(i128::MAX, i128::MAX - 1),
        Err(IntegerError::ArithmeticOverflow { operation: "lcm" })
    ));
}
