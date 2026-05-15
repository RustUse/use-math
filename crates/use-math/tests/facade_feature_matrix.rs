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
    feature = "arithmetic",
    feature = "number",
    feature = "integer",
    feature = "rational",
    feature = "real",
    feature = "complex",
    feature = "geometry",
    feature = "geode",
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
        algebra as _, arithmetic as _, calculus as _, catalan as _, combinatorics as _,
        complex as _, geometry as _, integer as _, linear as _, logic as _, number as _,
        probability as _, rational as _, real as _, series as _, set as _, statistics as _,
        trigonometry as _,
    };

    let _ = use_math::geode::TypeVector::new(vec![0]);
}

#[cfg(all(
    feature = "arithmetic",
    not(feature = "geometry"),
    not(feature = "combinatorics"),
    not(feature = "catalan"),
    not(feature = "series"),
    not(feature = "integer"),
    not(feature = "logic"),
    not(feature = "set"),
    not(feature = "statistics"),
    not(feature = "trigonometry"),
    not(feature = "linear"),
    not(feature = "number"),
    not(feature = "complex"),
    not(feature = "calculus"),
    not(feature = "probability"),
    not(feature = "rational"),
    not(feature = "real"),
    not(feature = "algebra"),
    not(feature = "geode")
))]
#[test]
fn facade_supports_arithmetic_without_other_concrete_features() {
    use use_math::{checked_add, div_floor, mod_floor, saturating_add, wrapping_mul};

    assert_eq!(checked_add(u8::MAX, 1), None);
    assert_eq!(div_floor(-7, 3), -3);
    assert_eq!(mod_floor(-7, 3), 2);
    assert_eq!(saturating_add(u8::MAX, 1), u8::MAX);
    assert_eq!(wrapping_mul(200_u8, 2), 144);
    assert_eq!(use_math::arithmetic::gcd(54, 24), 6);
    assert_eq!(use_math::arithmetic::lcm(6, 15), 30);
    assert!(use_math::arithmetic::is_divisible_by(84, 7));
}

#[cfg(all(
    feature = "algebra",
    not(feature = "geometry"),
    not(feature = "combinatorics"),
    not(feature = "catalan"),
    not(feature = "series"),
    not(feature = "integer"),
    not(feature = "logic"),
    not(feature = "set"),
    not(feature = "statistics"),
    not(feature = "trigonometry"),
    not(feature = "linear"),
    not(feature = "number"),
    not(feature = "complex"),
    not(feature = "calculus"),
    not(feature = "probability"),
    not(feature = "rational"),
    not(feature = "real")
))]
#[test]
fn facade_supports_algebra_without_other_concrete_features() {
    use use_math::{identity_element, is_abelian_group, is_distributive_over, is_ring};

    let residues = [0_u8, 1, 2];
    let add_mod_3 = |left, right| (left + right) % 3;
    let mul_mod_3 = |left, right| (left * right) % 3;

    assert_eq!(identity_element(&residues, add_mod_3), Some(0));
    assert!(is_abelian_group(&residues, add_mod_3));
    assert!(is_distributive_over(&residues, mul_mod_3, add_mod_3));
    assert!(is_ring(&residues, add_mod_3, mul_mod_3));
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
fn facade_supports_calculus_without_other_concrete_features() -> Result<(), use_math::CalculusError>
{
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
            if x == 0.0 { 1.0 } else { x.sin() / x }
        },
        0.0,
    )?;

    assert!((slope - 6.0).abs() < 1.0e-6);
    assert!((area - (1.0 / 3.0)).abs() < 1.0e-6);
    assert!((sinc_limit - 1.0).abs() < 1.0e-5);
    assert!(matches!(
        symmetric_limit(|x| if x < 0.0 { -1.0 } else { 1.0 }, 0.0, 1.0e-6, 1.0e-3,),
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
fn facade_supports_probability_without_other_concrete_features()
-> Result<(), use_math::ProbabilityError> {
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
    feature = "series",
    not(feature = "geometry"),
    not(feature = "combinatorics"),
    not(feature = "catalan"),
    not(feature = "integer"),
    not(feature = "complex"),
    not(feature = "calculus"),
    not(feature = "probability"),
    not(feature = "rational"),
    not(feature = "real")
))]
#[test]
fn facade_supports_series_without_other_concrete_features() -> Result<(), use_math::SeriesError> {
    use use_math::{arithmetic_nth_term, arithmetic_sum, geometric_nth_term, geometric_sum};

    assert_eq!(arithmetic_nth_term(3, 2, 4)?, 11);
    assert_eq!(arithmetic_sum(3, 2, 5)?, 35);
    assert_eq!(geometric_nth_term(2, 3, 4)?, 162);
    assert_eq!(geometric_sum(2, 3, 4)?, 80);

    Ok(())
}

#[cfg(all(
    feature = "logic",
    not(feature = "geometry"),
    not(feature = "combinatorics"),
    not(feature = "catalan"),
    not(feature = "series"),
    not(feature = "integer"),
    not(feature = "complex"),
    not(feature = "calculus"),
    not(feature = "probability"),
    not(feature = "rational"),
    not(feature = "real")
))]
#[test]
fn facade_supports_logic_without_other_concrete_features() {
    use use_math::{equivalence, exclusive_or, implication, majority, nand, nor};

    assert!(implication(false, true));
    assert!(!implication(true, false));
    assert!(equivalence(true, true));
    assert!(exclusive_or(true, false));
    assert!(!nand(true, true));
    assert!(nor(false, false));
    assert!(majority(true, true, false));
}

#[cfg(all(
    feature = "set",
    not(feature = "geometry"),
    not(feature = "combinatorics"),
    not(feature = "catalan"),
    not(feature = "series"),
    not(feature = "integer"),
    not(feature = "logic"),
    not(feature = "complex"),
    not(feature = "calculus"),
    not(feature = "probability"),
    not(feature = "rational"),
    not(feature = "real")
))]
#[test]
fn facade_supports_set_without_other_concrete_features() {
    use use_math::{
        are_disjoint, contains_member, is_subset, set_difference, set_intersection,
        set_symmetric_difference, set_union,
    };

    let left = [1, 2, 2, 3];
    let right = [3, 4, 2, 5];

    assert!(contains_member(&left, &1));
    assert!(is_subset(&[2, 3], &right));
    assert!(!are_disjoint(&left, &right));
    assert_eq!(set_union(&left, &right), vec![1, 2, 3, 4, 5]);
    assert_eq!(set_intersection(&left, &right), vec![2, 3]);
    assert_eq!(set_difference(&left, &right), vec![1]);
    assert_eq!(set_symmetric_difference(&left, &right), vec![1, 4, 5]);
}

#[cfg(all(
    feature = "trigonometry",
    not(feature = "geometry"),
    not(feature = "combinatorics"),
    not(feature = "catalan"),
    not(feature = "series"),
    not(feature = "integer"),
    not(feature = "logic"),
    not(feature = "set"),
    not(feature = "complex"),
    not(feature = "calculus"),
    not(feature = "probability"),
    not(feature = "rational"),
    not(feature = "real")
))]
#[test]
fn facade_supports_trigonometry_without_other_concrete_features() {
    use core::f64::consts::PI;
    use use_math::{
        Angle, cos_deg, degrees_to_radians, normalize_degrees, radians_to_degrees, sin_deg, tan_deg,
    };

    let acute = Angle::from_degrees(30.0);
    let wrapped = Angle::from_degrees(765.0).normalized();

    assert!((acute.radians() - (PI / 6.0)).abs() < 1.0e-12);
    assert!((degrees_to_radians(90.0) - (PI / 2.0)).abs() < 1.0e-12);
    assert!((radians_to_degrees(acute.radians()) - 30.0).abs() < 1.0e-12);
    assert!((wrapped.degrees() - 45.0).abs() < 1.0e-12);
    assert!((normalize_degrees(-90.0) - 270.0).abs() < 1.0e-12);
    assert!((acute.sin() - 0.5).abs() < 1.0e-12);
    assert!((cos_deg(60.0) - 0.5).abs() < 1.0e-12);
    assert!((sin_deg(30.0) - 0.5).abs() < 1.0e-12);
    assert!((tan_deg(45.0) - 1.0).abs() < 1.0e-12);
}

#[cfg(all(
    feature = "statistics",
    not(feature = "geometry"),
    not(feature = "combinatorics"),
    not(feature = "catalan"),
    not(feature = "series"),
    not(feature = "integer"),
    not(feature = "logic"),
    not(feature = "set"),
    not(feature = "trigonometry"),
    not(feature = "complex"),
    not(feature = "calculus"),
    not(feature = "probability"),
    not(feature = "rational"),
    not(feature = "real")
))]
#[test]
fn facade_supports_statistics_without_other_concrete_features()
-> Result<(), use_math::StatisticsError> {
    use use_math::{
        mean, median, population_std_dev, population_variance, sample_std_dev, sample_variance,
    };

    let values = [2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
    let sample = [1.0, 2.0, 3.0, 4.0];

    assert!((mean(&values)? - 5.0).abs() < 1.0e-12);
    assert!((median(&values)? - 4.5).abs() < 1.0e-12);
    assert!((population_variance(&values)? - 4.0).abs() < 1.0e-12);
    assert!((population_std_dev(&values)? - 2.0).abs() < 1.0e-12);
    assert!((sample_variance(&sample)? - 1.666_666_666_666_666_7).abs() < 1.0e-12);
    assert!((sample_std_dev(&sample)? - 1.290_994_448_735_805_6).abs() < 1.0e-12);

    Ok(())
}

#[cfg(all(
    feature = "linear",
    not(feature = "geometry"),
    not(feature = "combinatorics"),
    not(feature = "catalan"),
    not(feature = "series"),
    not(feature = "integer"),
    not(feature = "logic"),
    not(feature = "set"),
    not(feature = "statistics"),
    not(feature = "trigonometry"),
    not(feature = "complex"),
    not(feature = "calculus"),
    not(feature = "probability"),
    not(feature = "rational"),
    not(feature = "real")
))]
#[test]
fn facade_supports_linear_without_other_concrete_features() -> Result<(), use_math::LinearError> {
    use use_math::{LinearVector2, Matrix2, dot, solve_2x2};

    let vector = LinearVector2::new(3.0, 4.0);
    let other = LinearVector2::new(-2.0, 1.0);
    let matrix = Matrix2::new(2.0, 1.0, 5.0, 3.0);

    assert_eq!(vector + other, LinearVector2::new(1.0, 5.0));
    assert!((dot(vector, other) + 2.0).abs() < 1.0e-12);
    assert!((vector.magnitude() - 5.0).abs() < 1.0e-12);
    assert_eq!(
        matrix.mul_vector(LinearVector2::new(1.0, -1.0)),
        LinearVector2::new(1.0, 2.0)
    );
    assert_eq!(matrix * Matrix2::identity(), matrix);
    assert_eq!(
        solve_2x2(matrix, LinearVector2::new(1.0, 2.0))?,
        LinearVector2::new(1.0, -1.0)
    );

    Ok(())
}

#[cfg(all(
    feature = "number",
    not(feature = "geometry"),
    not(feature = "combinatorics"),
    not(feature = "catalan"),
    not(feature = "series"),
    not(feature = "integer"),
    not(feature = "logic"),
    not(feature = "set"),
    not(feature = "statistics"),
    not(feature = "trigonometry"),
    not(feature = "linear"),
    not(feature = "complex"),
    not(feature = "calculus"),
    not(feature = "probability"),
    not(feature = "rational"),
    not(feature = "real")
))]
#[test]
fn facade_supports_number_without_other_concrete_features() {
    use use_math::{
        GOLDEN_RATIO, NumberCategory, NumberSign, SQRT_3, classify_number, classify_number_sign,
        is_finite_number,
    };

    assert_eq!(classify_number(f64::NAN), NumberCategory::Nan);
    assert_eq!(
        classify_number(f64::from_bits(1)),
        NumberCategory::Subnormal
    );
    assert_eq!(classify_number_sign(-12.5), Some(NumberSign::Negative));
    assert_eq!(classify_number_sign(f64::NAN), None);
    assert!(is_finite_number(3.5));
    assert!(!is_finite_number(f64::INFINITY));
    assert!(
        GOLDEN_RATIO
            .mul_add(GOLDEN_RATIO, -(GOLDEN_RATIO + 1.0))
            .abs()
            < 1.0e-12
    );
    assert!(SQRT_3.mul_add(SQRT_3, -3.0).abs() < 1.0e-12);
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
fn facade_supports_rational_without_other_concrete_features() -> Result<(), use_math::RationalError>
{
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
