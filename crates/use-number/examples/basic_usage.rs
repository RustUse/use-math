use use_number::{
    GOLDEN_RATIO, NumberCategory, NumberSign, SQRT_3, classify_number, classify_number_sign,
};

fn main() {
    assert_eq!(classify_number(f64::INFINITY), NumberCategory::Infinite);
    assert_eq!(
        classify_number(f64::from_bits(1)),
        NumberCategory::Subnormal
    );
    assert_eq!(classify_number_sign(-12.0), Some(NumberSign::Negative));
    assert!(
        GOLDEN_RATIO
            .mul_add(GOLDEN_RATIO, -(GOLDEN_RATIO + 1.0))
            .abs()
            < 1.0e-12
    );
    assert!(SQRT_3.mul_add(SQRT_3, -3.0).abs() < 1.0e-12);
}
