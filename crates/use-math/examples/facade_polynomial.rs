use use_math::Polynomial;

fn main() {
    let p = Polynomial::quadratic(1.0, -3.0, 2.0);
    assert!((p.evaluate(5.0) - 36.0).abs() < 1.0e-10);

    let d = p.derivative();
    assert_eq!(d.coefficients(), &[-3.0, 4.0]);
}
