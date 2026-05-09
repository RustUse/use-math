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

#[cfg(all(feature = "combinatorics", not(feature = "geometry")))]
#[test]
fn facade_supports_combinatorics_without_geometry() -> Result<(), use_math::CombinatoricsError> {
    use use_math::{combinations, factorial, permutations};

    assert_eq!(factorial(4)?, 24);
    assert_eq!(permutations(5, 2)?, 20);
    assert_eq!(combinations(5, 2)?, 10);

    Ok(())
}

#[cfg(feature = "polynomial")]
#[test]
fn facade_supports_polynomial() {
    use use_math::Polynomial;

    let p = Polynomial::quadratic(1.0, -3.0, 2.0);
    assert!((p.evaluate(5.0) - 36.0).abs() < 1.0e-10);
    assert_eq!(p.derivative().coefficients(), &[-3.0, 4.0]);
}

#[cfg(not(any(
    feature = "geometry",
    feature = "combinatorics",
    feature = "polynomial"
)))]
#[test]
fn facade_compiles_without_optional_features() {
    let crate_loaded = true;

    assert!(crate_loaded);
}
