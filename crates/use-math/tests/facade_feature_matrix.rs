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

#[cfg(all(feature = "complex", not(feature = "geometry"), not(feature = "combinatorics")))]
#[test]
fn facade_supports_complex_without_other_concrete_features() {
    use use_math::{Complex, Imaginary};

    let value = Complex::new(3.0_f64, 4.0_f64);

    assert_eq!(Complex::from(Imaginary::new(4.0_f64)), Complex::new(0.0, 4.0));
    assert!((value.magnitude() - 5.0).abs() < 1.0e-10);
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
