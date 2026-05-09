use use_math::prelude::*;

fn approx_eq(a: f64, b: f64) -> bool {
    (a - b).abs() < 1.0e-12
}

#[test]
fn facade_prelude_exposes_series() {
    let s = Series::new(vec![1.0, 2.0, 3.0]); // 1 + 2x + 3x²
    assert!(approx_eq(s.evaluate(2.0), 17.0));
    assert_eq!(s.order(), Some(2));
}

#[test]
fn facade_root_reexports_series() {
    let s = use_math::Series::new(vec![1.0, 2.0, 3.0]);
    assert!(approx_eq(s.evaluate(2.0), 17.0));
}

#[test]
fn facade_series_module_is_accessible() {
    let s = use_math::series::Series::zero();
    assert!(s.is_zero());
}
