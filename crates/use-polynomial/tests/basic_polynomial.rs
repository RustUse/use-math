use use_polynomial::Polynomial;

fn approx_eq(left: f64, right: f64) -> bool {
    (left - right).abs() < 1.0e-10
}

// --- Construction and normalization ---

#[test]
fn new_stores_coefficients_in_ascending_degree_order() {
    let p = Polynomial::new(vec![1.0, 2.0, 3.0]);
    assert_eq!(p.coefficients(), &[1.0, 2.0, 3.0]);
}

#[test]
fn new_strips_trailing_zeros() {
    let p = Polynomial::new(vec![1.0, 2.0, 0.0, 0.0]);
    assert_eq!(p.coefficients(), &[1.0, 2.0]);
}

#[test]
fn new_all_zeros_yields_zero_polynomial() {
    let p = Polynomial::new(vec![0.0, 0.0, 0.0]);
    assert!(p.is_zero());
    assert_eq!(p.coefficients(), &[] as &[f64]);
}

#[test]
fn new_empty_yields_zero_polynomial() {
    let p = Polynomial::new(vec![]);
    assert!(p.is_zero());
}

// --- Zero polynomial ---

#[test]
fn zero_is_zero() {
    assert!(Polynomial::zero().is_zero());
}

#[test]
fn zero_has_no_degree() {
    assert_eq!(Polynomial::zero().degree(), None);
}

#[test]
fn zero_evaluates_to_zero_everywhere() {
    assert!(approx_eq(Polynomial::zero().evaluate(0.0), 0.0));
    assert!(approx_eq(Polynomial::zero().evaluate(99.0), 0.0));
    assert!(approx_eq(Polynomial::zero().evaluate(-5.0), 0.0));
}

#[test]
fn zero_coefficients_are_empty() {
    assert_eq!(Polynomial::zero().coefficients(), &[] as &[f64]);
}

// --- Degree ---

#[test]
fn degree_of_constant_nonzero_is_zero() {
    assert_eq!(Polynomial::constant(3.0).degree(), Some(0));
}

#[test]
fn degree_of_linear_is_one() {
    assert_eq!(Polynomial::linear(1.0, 2.0).degree(), Some(1));
}

#[test]
fn degree_of_quadratic_is_two() {
    assert_eq!(Polynomial::quadratic(1.0, 0.0, 1.0).degree(), Some(2));
}

#[test]
fn degree_after_trailing_zero_strip_is_correct() {
    // Degree should be 1 even if extra zeros were supplied.
    let p = Polynomial::new(vec![1.0, 2.0, 0.0, 0.0]);
    assert_eq!(p.degree(), Some(1));
}

// --- Coefficient lookup ---

#[test]
fn coefficient_returns_correct_values() {
    let p = Polynomial::new(vec![5.0, 3.0, 1.0]);
    assert!(approx_eq(p.coefficient(0), 5.0));
    assert!(approx_eq(p.coefficient(1), 3.0));
    assert!(approx_eq(p.coefficient(2), 1.0));
}

#[test]
fn coefficient_returns_zero_for_out_of_range() {
    let p = Polynomial::new(vec![1.0, 2.0]);
    assert!(approx_eq(p.coefficient(5), 0.0));
}

#[test]
fn coefficient_on_zero_polynomial_returns_zero() {
    assert!(approx_eq(Polynomial::zero().coefficient(0), 0.0));
    assert!(approx_eq(Polynomial::zero().coefficient(100), 0.0));
}

// --- Evaluation ---

#[test]
fn evaluate_constant_polynomial() {
    let p = Polynomial::constant(7.0);
    assert!(approx_eq(p.evaluate(0.0), 7.0));
    assert!(approx_eq(p.evaluate(10.0), 7.0));
}

#[test]
fn evaluate_linear_polynomial() {
    // 3 + 2x at x=4 -> 11
    let p = Polynomial::linear(3.0, 2.0);
    assert!(approx_eq(p.evaluate(4.0), 11.0));
}

#[test]
fn evaluate_quadratic_polynomial() {
    // 1 - 3x + 2x² at x=5 -> 1 - 15 + 50 = 36
    let p = Polynomial::quadratic(1.0, -3.0, 2.0);
    assert!(approx_eq(p.evaluate(5.0), 36.0));
}

#[test]
fn evaluate_at_zero_returns_constant_term() {
    let p = Polynomial::new(vec![42.0, 1.0, 1.0]);
    assert!(approx_eq(p.evaluate(0.0), 42.0));
}

#[test]
fn evaluate_zero_polynomial_returns_zero() {
    assert!(approx_eq(Polynomial::zero().evaluate(7.0), 0.0));
}

// --- Derivative ---

#[test]
fn derivative_of_zero_is_zero() {
    assert!(Polynomial::zero().derivative().is_zero());
}

#[test]
fn derivative_of_constant_is_zero() {
    assert!(Polynomial::constant(5.0).derivative().is_zero());
}

#[test]
fn derivative_of_linear() {
    // d/dx (3 + 2x) = 2
    let p = Polynomial::linear(3.0, 2.0);
    let d = p.derivative();
    assert_eq!(d.coefficients(), &[2.0]);
}

#[test]
fn derivative_of_quadratic() {
    // d/dx (1 - 3x + 2x²) = -3 + 4x
    let p = Polynomial::quadratic(1.0, -3.0, 2.0);
    let d = p.derivative();
    assert_eq!(d.coefficients(), &[-3.0, 4.0]);
}

#[test]
fn derivative_of_cubic() {
    // d/dx (1 + 2x + 3x² + 4x³) = 2 + 6x + 12x²
    let p = Polynomial::new(vec![1.0, 2.0, 3.0, 4.0]);
    let d = p.derivative();
    assert_eq!(d.coefficients(), &[2.0, 6.0, 12.0]);
}

// --- Addition ---

#[test]
fn add_two_polynomials_same_degree() {
    let p = Polynomial::linear(1.0, 2.0);
    let q = Polynomial::linear(3.0, 4.0);
    assert_eq!(p.add(&q).coefficients(), &[4.0, 6.0]);
}

#[test]
fn add_polynomials_different_degrees() {
    let p = Polynomial::quadratic(1.0, 2.0, 3.0);
    let q = Polynomial::linear(4.0, 5.0);
    assert_eq!(p.add(&q).coefficients(), &[5.0, 7.0, 3.0]);
}

#[test]
fn add_zero_polynomial_is_identity() {
    let p = Polynomial::quadratic(1.0, -3.0, 2.0);
    assert_eq!(p.add(&Polynomial::zero()), p);
}

#[test]
fn add_inverse_gives_zero() {
    let p = Polynomial::linear(1.0, 2.0);
    let neg = p.scale(-1.0);
    assert!(p.add(&neg).is_zero());
}

// --- Subtraction ---

#[test]
fn sub_two_polynomials_same_degree() {
    let p = Polynomial::linear(5.0, 6.0);
    let q = Polynomial::linear(1.0, 2.0);
    assert_eq!(p.sub(&q).coefficients(), &[4.0, 4.0]);
}

#[test]
fn sub_polynomial_from_itself_gives_zero() {
    let p = Polynomial::quadratic(1.0, -3.0, 2.0);
    assert!(p.sub(&p).is_zero());
}

#[test]
fn sub_different_degrees() {
    let p = Polynomial::quadratic(5.0, 3.0, 1.0);
    let q = Polynomial::constant(2.0);
    assert_eq!(p.sub(&q).coefficients(), &[3.0, 3.0, 1.0]);
}

// --- Multiplication ---

#[test]
fn mul_yields_difference_of_squares() {
    // (1 + x)(1 - x) = 1 - x²
    let a = Polynomial::linear(1.0, 1.0);
    let b = Polynomial::linear(1.0, -1.0);
    assert_eq!(a.mul(&b).coefficients(), &[1.0, 0.0, -1.0]);
}

#[test]
fn mul_by_zero_is_zero() {
    let p = Polynomial::quadratic(1.0, 2.0, 3.0);
    assert!(p.mul(&Polynomial::zero()).is_zero());
    assert!(Polynomial::zero().mul(&p).is_zero());
}

#[test]
fn mul_by_constant_one_is_identity() {
    let p = Polynomial::quadratic(1.0, 2.0, 3.0);
    let one = Polynomial::constant(1.0);
    assert_eq!(p.mul(&one), p);
}

#[test]
fn mul_linear_by_linear() {
    // (1 + 2x)(3 + 4x) = 3 + 10x + 8x²
    let a = Polynomial::linear(1.0, 2.0);
    let b = Polynomial::linear(3.0, 4.0);
    assert_eq!(a.mul(&b).coefficients(), &[3.0, 10.0, 8.0]);
}

// --- Scaling ---

#[test]
fn scale_by_scalar() {
    let p = Polynomial::linear(1.0, 2.0);
    assert_eq!(p.scale(3.0).coefficients(), &[3.0, 6.0]);
}

#[test]
fn scale_by_zero_gives_zero() {
    let p = Polynomial::quadratic(1.0, 2.0, 3.0);
    assert!(p.scale(0.0).is_zero());
}

#[test]
fn scale_by_one_is_identity() {
    let p = Polynomial::quadratic(1.0, -2.0, 5.0);
    assert_eq!(p.scale(1.0), p);
}

// --- Constructors: constant, linear, quadratic ---

#[test]
fn constant_constructor() {
    let p = Polynomial::constant(5.0);
    assert_eq!(p.degree(), Some(0));
    assert!(approx_eq(p.evaluate(99.0), 5.0));
}

#[test]
fn constant_zero_yields_zero_polynomial() {
    assert!(Polynomial::constant(0.0).is_zero());
}

#[test]
fn linear_constructor() {
    // 3 + 2x
    let p = Polynomial::linear(3.0, 2.0);
    assert_eq!(p.degree(), Some(1));
    assert!(approx_eq(p.evaluate(1.0), 5.0));
}

#[test]
fn quadratic_constructor() {
    // 1 - 3x + 2x²
    let p = Polynomial::quadratic(1.0, -3.0, 2.0);
    assert_eq!(p.degree(), Some(2));
    assert!(approx_eq(p.evaluate(0.0), 1.0));
    assert!(approx_eq(p.evaluate(1.0), 0.0));
    assert!(approx_eq(p.evaluate(2.0), 3.0));
}
