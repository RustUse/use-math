use use_math::prelude::*;

fn approx_eq(left: f64, right: f64) -> bool {
    (left - right).abs() < 1.0e-10
}

#[test]
fn facade_prelude_exposes_polynomial_type() {
    let p = Polynomial::quadratic(1.0, -3.0, 2.0);
    assert_eq!(p.degree(), Some(2));
    assert!(approx_eq(p.evaluate(5.0), 36.0));
}

#[test]
fn facade_root_reexports_polynomial() {
    let p = use_math::Polynomial::linear(3.0, 2.0);
    assert!(approx_eq(p.evaluate(1.0), 5.0));
}

#[test]
fn facade_polynomial_module_is_accessible() {
    let p = use_math::polynomial::Polynomial::zero();
    assert!(p.is_zero());
}
