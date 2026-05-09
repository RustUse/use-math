use use_polynomial::Polynomial;

fn main() {
    // Construct a quadratic: 1 - 3x + 2x²
    let p = Polynomial::quadratic(1.0, -3.0, 2.0);
    assert_eq!(p.degree(), Some(2));

    // Evaluate at x = 5: 1 - 15 + 50 = 36
    let value = p.evaluate(5.0);
    assert!((value - 36.0).abs() < 1.0e-10);

    // Differentiate: d/dx (1 - 3x + 2x²) = -3 + 4x
    let derivative = p.derivative();
    assert_eq!(derivative.coefficients(), &[-3.0, 4.0]);

    // Add two polynomials: (1 - 3x + 2x²) + (0 + x) = 1 - 2x + 2x²
    let linear = Polynomial::linear(0.0, 1.0);
    let sum = p.add(&linear);
    assert_eq!(sum.coefficients(), &[1.0, -2.0, 2.0]);

    // Multiply: (1 + x)(1 - x) = 1 - x²
    let a = Polynomial::linear(1.0, 1.0);
    let b = Polynomial::linear(1.0, -1.0);
    let product = a.mul(&b);
    assert_eq!(product.coefficients(), &[1.0, 0.0, -1.0]);
}
