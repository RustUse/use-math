use use_math::Series;

fn approx_eq(a: f64, b: f64) -> bool {
    (a - b).abs() < 1.0e-12
}

fn main() {
    let s = Series::new(vec![1.0, 2.0, 3.0]); // 1 + 2x + 3x²
    assert!(approx_eq(s.evaluate(2.0), 17.0));

    let d = s.derivative(); // 2 + 6x
    assert_eq!(d.coefficients(), &[2.0, 6.0]);

    let i = s.integral(0.0); // x + x² + x³
    assert_eq!(i.coefficients(), &[0.0, 1.0, 1.0, 1.0]);
}
