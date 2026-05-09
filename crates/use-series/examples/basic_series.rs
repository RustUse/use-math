use use_series::Series;

fn approx_eq(a: f64, b: f64) -> bool {
    (a - b).abs() < 1.0e-12
}

fn main() {
    // Build a series representing 1 + 2x + 3x²
    let series = Series::new(vec![1.0, 2.0, 3.0]);

    // Evaluate at x = 2: 1 + 4 + 12 = 17
    assert!(approx_eq(series.evaluate(2.0), 17.0));

    // Formal derivative: 2 + 6x
    let deriv = series.derivative();
    assert_eq!(deriv.coefficients(), &[2.0, 6.0]);

    // Formal integral with constant 0: x + x² + x³
    let integ = series.integral(0.0);
    assert_eq!(integ.coefficients(), &[0.0, 1.0, 1.0, 1.0]);

    // Truncate to order 1: 1 + 2x
    let truncated = series.truncate(1);
    assert_eq!(truncated.coefficients(), &[1.0, 2.0]);

    // Shift by 2: x² + 2x³ + 3x⁴
    let shifted = series.shift(2);
    assert_eq!(shifted.coefficients(), &[0.0, 0.0, 1.0, 2.0, 3.0]);

    // Arithmetic
    let lhs = Series::new(vec![1.0, 2.0]);
    let rhs = Series::new(vec![3.0, 4.0]);
    let sum = lhs.add(&rhs);
    assert_eq!(sum.coefficients(), &[4.0, 6.0]);
    let product = lhs.mul(&rhs); // (1 + 2x)(3 + 4x) = 3 + 10x + 8x²
    assert_eq!(product.coefficients(), &[3.0, 10.0, 8.0]);
}
