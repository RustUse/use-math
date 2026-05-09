#[cfg(all(feature = "geometry", feature = "combinatorics"))]
#[test]
fn facade_supports_geometry_and_combinatorics_together() -> Result<(), Box<dyn std::error::Error>> {
    use use_math::prelude::*;

    let origin = Point2::try_new(0.0, 0.0)?;
    let point = Point2::try_new(3.0, 4.0)?;
    let distance = distance_2d(origin, point);

    assert_eq!(factorial(4)?, 24);
    assert_eq!(combinations(5, 2)?, 10);
    assert!((distance - 5.0).abs() < 1.0e-10);
    assert_eq!(midpoint_2d(origin, point), Point2::try_new(1.5, 2.0)?);

    Ok(())
}

#[cfg(all(
    feature = "number",
    feature = "integer",
    feature = "rational",
    feature = "real",
    feature = "complex",
    feature = "geometry",
    feature = "combinatorics",
    feature = "series",
    feature = "catalan",
    feature = "algebra",
    feature = "linear",
    feature = "calculus",
    feature = "probability",
    feature = "statistics",
    feature = "trigonometry",
    feature = "logic",
    feature = "set",
))]
#[test]
fn facade_exposes_all_namespace_features() {
    use use_math::{
        algebra as _, calculus as _, catalan as _, combinatorics as _, complex as _, geometry as _,
        integer as _, linear as _, logic as _, number as _, probability as _, rational as _,
        real as _, series as _, set as _, statistics as _, trigonometry as _,
    };
}

#[cfg(all(feature = "geometry", not(feature = "combinatorics")))]
#[test]
fn facade_supports_geometry_without_combinatorics() -> Result<(), use_math::GeometryError> {
    use use_math::{Point2, distance_2d, midpoint_2d};

    let origin = Point2::try_new(0.0, 0.0)?;
    let point = Point2::try_new(3.0, 4.0)?;

    assert!((distance_2d(origin, point) - 5.0).abs() < 1.0e-10);
    assert_eq!(midpoint_2d(origin, point), Point2::try_new(1.5, 2.0)?);

    Ok(())
}

#[cfg(all(
    feature = "complex",
    not(feature = "geometry"),
    not(feature = "combinatorics")
))]
#[test]
fn facade_supports_complex_without_other_concrete_features() {
    use use_math::{Complex, Imaginary};

    let value = Complex::new(3.0_f64, 4.0_f64);

    assert_eq!(
        Complex::from(Imaginary::new(4.0_f64)),
        Complex::new(0.0, 4.0)
    );
    assert!((value.magnitude() - 5.0).abs() < 1.0e-10);
}

#[cfg(all(
    feature = "calculus",
    not(feature = "geometry"),
    not(feature = "combinatorics"),
    not(feature = "complex")
))]
#[test]
fn facade_supports_calculus_without_other_concrete_features() -> Result<(), use_math::CalculusError> {
    use use_math::{
        Differentiator, IntegrationInterval, Integrator, LimitApproximator, symmetric_limit,
    };

    let differentiator = Differentiator::try_new(1.0e-5)?;
    let integrator = Integrator::try_new(128)?;
    let interval = IntegrationInterval::try_new(0.0, 1.0)?;
    let limit = LimitApproximator::try_new(1.0e-6, 1.0e-5)?;

    let slope = differentiator.derivative_at(|x| x.powi(2), 3.0)?;
    let area = integrator.simpson(|x| x * x, interval)?;
    let sinc_limit = limit.at(
        |x| {
            if x == 0.0 {
                1.0
            } else {
                x.sin() / x
            }
        },
        0.0,
    )?;

    assert!((slope - 6.0).abs() < 1.0e-6);
    assert!((area - (1.0 / 3.0)).abs() < 1.0e-6);
    assert!((sinc_limit - 1.0).abs() < 1.0e-5);
    assert!(matches!(
        symmetric_limit(
            |x| if x < 0.0 { -1.0 } else { 1.0 },
            0.0,
            1.0e-6,
            1.0e-3,
        ),
        Err(use_math::CalculusError::LimitMismatch { .. })
    ));

    Ok(())
}

#[cfg(all(
    feature = "catalan",
    not(feature = "geometry"),
    not(feature = "combinatorics"),
    not(feature = "integer"),
    not(feature = "complex"),
    not(feature = "calculus"),
    not(feature = "probability"),
    not(feature = "rational"),
    not(feature = "real")
))]
#[test]
fn facade_supports_catalan_without_other_concrete_features() -> Result<(), use_math::CatalanError> {
    use use_math::{catalan, fuss_catalan};

    assert_eq!(catalan(4)?, 14);
    assert_eq!(fuss_catalan(3, 3)?, 12);

    Ok(())
}

#[cfg(all(
    feature = "probability",
    not(feature = "geometry"),
    not(feature = "combinatorics"),
    not(feature = "complex"),
    not(feature = "calculus")
))]
#[test]
fn facade_supports_probability_without_other_concrete_features() -> Result<(), use_math::ProbabilityError> {
    use use_math::{Bernoulli, Probability, independent_intersection, independent_union};

    let rain = Probability::from_fraction(1, 4)?;
    let traffic = Probability::try_new(0.5)?;
    let commute = Bernoulli::new(rain);

    assert!((independent_intersection(rain, traffic).value() - 0.125).abs() < 1.0e-12);
    assert!((independent_union(rain, traffic).value() - 0.625).abs() < 1.0e-12);
    assert_eq!(commute.failure_probability(), Probability::try_new(0.75)?);
    assert!((commute.variance() - 0.1875).abs() < 1.0e-12);

    Ok(())
}

#[cfg(all(
    feature = "real",
    not(feature = "geometry"),
    not(feature = "combinatorics"),
    not(feature = "complex"),
    not(feature = "calculus"),
    not(feature = "probability")
))]
#[test]
fn facade_supports_real_without_other_concrete_features() -> Result<(), use_math::RealError> {
    use use_math::{Real, RealInterval, approx_eq};

    let interval = RealInterval::try_new(-2.0, 6.0)?;
    let midpoint = interval.midpoint();
    let clamped = interval.clamp(Real::try_new(8.0)?);

    assert!(interval.contains(Real::try_new(1.5)?));
    assert_eq!(clamped, Real::try_new(6.0)?);
    assert!(approx_eq(midpoint, Real::try_new(2.0)?, 1.0e-12)?);

    Ok(())
}

#[cfg(all(
    feature = "integer",
    not(feature = "geometry"),
    not(feature = "combinatorics"),
    not(feature = "complex"),
    not(feature = "calculus"),
    not(feature = "probability"),
    not(feature = "rational"),
    not(feature = "real")
))]
#[test]
fn facade_supports_integer_without_other_concrete_features() -> Result<(), use_math::IntegerError> {
    use use_math::{IntegerSign, classify_sign, gcd, is_divisible_by, lcm};

    assert_eq!(classify_sign(-12), IntegerSign::Negative);
    assert!(is_divisible_by(84, 7)?);
    assert_eq!(gcd(-54, 24), 6);
    assert_eq!(lcm(-6, 15)?, 30);

    Ok(())
}

#[cfg(all(
    feature = "rational",
    not(feature = "geometry"),
    not(feature = "combinatorics"),
    not(feature = "complex"),
    not(feature = "calculus"),
    not(feature = "probability"),
    not(feature = "real")
))]
#[test]
fn facade_supports_rational_without_other_concrete_features() -> Result<(), use_math::RationalError> {
    use use_math::Rational;

    let half = Rational::try_new(1, 2)?;
    let third = Rational::try_new(1, 3)?;

    assert_eq!(half.checked_add(third)?, Rational::try_new(5, 6)?);
    assert_eq!(half.checked_div(third)?, Rational::try_new(3, 2)?);
    assert_eq!(half.reciprocal()?, Rational::try_new(2, 1)?);

    Ok(())
}

#[cfg(all(feature = "combinatorics", not(feature = "geometry")))]
#[test]
fn facade_supports_combinatorics_without_geometry() -> Result<(), use_math::CombinatoricsError> {
    use use_math::{combinations, factorial, permutations};

    assert_eq!(factorial(4)?, 24);
    assert_eq!(permutations(5, 2)?, 20);
    assert_eq!(combinations(5, 2)?, 10);

    Ok(())
}

#[cfg(not(any(feature = "geometry", feature = "combinatorics")))]
#[test]
fn facade_compiles_without_optional_features() {
    let crate_loaded = true;

    assert!(crate_loaded);
}
