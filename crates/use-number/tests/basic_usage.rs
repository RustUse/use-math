use use_number::{
    GOLDEN_RATIO, NumberCategory, NumberSign, SQRT_3, classify_number, classify_number_sign,
    is_finite_number,
};

#[test]
fn direct_number_usage_covers_classification_and_constants() {
    assert_eq!(classify_number(f64::NAN), NumberCategory::Nan);
    assert_eq!(classify_number(f64::NEG_INFINITY), NumberCategory::Infinite);
    assert_eq!(classify_number(0.0), NumberCategory::Zero);
    assert_eq!(
        classify_number(f64::from_bits(1)),
        NumberCategory::Subnormal
    );
    assert_eq!(classify_number(4.0), NumberCategory::Normal);

    assert_eq!(classify_number_sign(-3.5), Some(NumberSign::Negative));
    assert_eq!(classify_number_sign(-0.0), Some(NumberSign::Zero));
    assert_eq!(classify_number_sign(9.0), Some(NumberSign::Positive));
    assert_eq!(classify_number_sign(f64::NAN), None);

    assert!(is_finite_number(7.0));
    assert!(!is_finite_number(f64::INFINITY));

    assert!(
        GOLDEN_RATIO
            .mul_add(GOLDEN_RATIO, -(GOLDEN_RATIO + 1.0))
            .abs()
            < 1.0e-12
    );
    assert!(SQRT_3.mul_add(SQRT_3, -3.0).abs() < 1.0e-12);
}
