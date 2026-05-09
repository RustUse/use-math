use use_series::Series;

fn approx_eq(a: f64, b: f64) -> bool {
    (a - b).abs() < 1.0e-12
}

#[test]
fn series_api_covers_basic_workflow() {
    let s = Series::new(vec![1.0, 2.0, 3.0]); // 1 + 2x + 3x²

    // evaluation
    assert!(approx_eq(s.evaluate(2.0), 17.0));

    // accessors
    assert_eq!(s.order(), Some(2));
    assert_eq!(s.len(), 3);
    assert!(!s.is_empty());
    assert!(!s.is_zero());

    // derivative
    assert_eq!(s.derivative().coefficients(), &[2.0, 6.0]);

    // integral
    assert_eq!(s.integral(0.0).coefficients(), &[0.0, 1.0, 1.0, 1.0]);
}

#[test]
fn series_zero_behavior() {
    let z = Series::zero();
    assert!(z.is_zero());
    assert!(z.is_empty());
    assert_eq!(z.order(), None);
    assert!(approx_eq(z.evaluate(99.0), 0.0));
    assert!(approx_eq(z.coefficient(0), 0.0));
}

#[test]
fn series_arithmetic_workflow() {
    let a = Series::new(vec![1.0, 2.0]); // 1 + 2x
    let b = Series::new(vec![3.0, 4.0]); // 3 + 4x

    let sum = a.add(&b);
    assert_eq!(sum.coefficients(), &[4.0, 6.0]);

    let diff = b.sub(&a);
    assert_eq!(diff.coefficients(), &[2.0, 2.0]);

    let product = a.mul(&b); // (1 + 2x)(3 + 4x) = 3 + 10x + 8x²
    assert_eq!(product.coefficients(), &[3.0, 10.0, 8.0]);

    let scaled = a.scale(3.0);
    assert_eq!(scaled.coefficients(), &[3.0, 6.0]);
}

#[test]
fn series_truncate_and_shift() {
    let s = Series::new(vec![1.0, 2.0, 3.0, 4.0]);

    let t = s.truncate(2);
    assert_eq!(t.coefficients(), &[1.0, 2.0, 3.0]);

    let shifted = Series::new(vec![1.0, 2.0]).shift(2);
    assert_eq!(shifted.coefficients(), &[0.0, 0.0, 1.0, 2.0]);
}
