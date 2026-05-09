use use_complex::{Complex, Imaginary};

fn assert_close(lhs: f64, rhs: f64) {
    let difference = (lhs - rhs).abs();

    assert!(
        difference <= 1.0e-10,
        "left={lhs}, right={rhs}, diff={difference}"
    );
}

#[test]
fn direct_complex_crate_usage_covers_rectangular_and_polar_helpers() {
    let value = Complex::new(3.0_f64, 4.0_f64);
    let lifted = Complex::from(Imaginary::new(4.0_f64));
    let (magnitude, argument) = value.to_polar();

    assert_eq!(value.to_string(), "3 + 4i");
    assert_eq!(lifted, Complex::new(0.0, 4.0));
    assert_close(magnitude, 5.0);
    assert_close(argument, value.argument());
}
