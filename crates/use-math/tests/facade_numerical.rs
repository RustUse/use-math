#[cfg(feature = "numerical")]
#[test]
fn facade_supports_numerical_namespace() {
    use use_math::numerical::{
        RootOptions, approx_eq, bisection, central_difference, newton_raphson, trapezoidal_rule,
    };

    assert!(approx_eq(0.1 + 0.2, 0.3, 1.0e-12));

    let slope = central_difference(|x| x * x, 3.0, 1.0e-6);
    let area = trapezoidal_rule(|x| x * x, 0.0, 1.0, 1_000).unwrap();
    let bisection_root =
        bisection(|x| x.mul_add(x, -2.0), 1.0, 2.0, RootOptions::default()).unwrap();
    let newton_root = newton_raphson(
        |x| x.mul_add(x, -2.0),
        |x| 2.0 * x,
        1.0,
        RootOptions::default(),
    )
    .unwrap();

    assert!((slope - 6.0).abs() < 1.0e-5);
    assert!((area - 1.0 / 3.0).abs() < 1.0e-6);
    assert!((bisection_root - 2.0_f64.sqrt()).abs() < 1.0e-8);
    assert!((newton_root - 2.0_f64.sqrt()).abs() < 1.0e-8);
}

#[cfg(all(feature = "numerical", feature = "interval"))]
#[test]
fn facade_supports_numerical_interval_bridge() {
    use use_math::{
        interval::Interval,
        numerical::{self, RootOptions},
    };

    let root = numerical::bisection_interval(
        |x| x.mul_add(x, -2.0),
        Interval::closed(1.0, 2.0),
        RootOptions::default(),
    )
    .unwrap();

    assert!((root - 2.0_f64.sqrt()).abs() < 1.0e-8);
}
