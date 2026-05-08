use use_math::{geometry, prelude::*};

fn approx_eq(left: f64, right: f64) -> bool {
    (left - right).abs() < 1.0e-10
}

#[test]
fn facade_prelude_exposes_validated_geometry_workflow() -> Result<(), geometry::GeometryError> {
    let a = Point2::try_new(0.0, 0.0)?;
    let b = Point2::try_new(4.0, 0.0)?;
    let c = Point2::try_new(0.0, 3.0)?;

    let triangle = Triangle::try_new(a, b, c)?;
    let bounds = Aabb2::from_points(a, c);

    assert!(approx_eq(distance_2d(a, b), 4.0));
    assert!(approx_eq(midpoint_2d(a, c).y(), 1.5));
    assert_eq!(try_orientation_2d(a, b, c)?, Orientation2::CounterClockwise);
    assert_eq!(
        orientation_2d_with_tolerance(a, b, c, 0.0)?,
        Orientation2::CounterClockwise
    );
    assert!(approx_eq(triangle.perimeter(), 12.0));
    assert!(bounds.contains_point(Point2::new(0.0, 1.5)));

    Ok(())
}

#[test]
fn facade_root_reexports_geometry_workflow() -> Result<(), use_math::GeometryError> {
    let a = use_math::Point2::try_new(0.0, 0.0)?;
    let b = use_math::Point2::try_new(4.0, 0.0)?;
    let c = use_math::Point2::try_new(0.0, 3.0)?;

    let triangle = use_math::Triangle::try_new(a, b, c)?;
    let bounds = use_math::Aabb2::from_points(a, c);

    assert!(approx_eq(use_math::distance_2d(a, b), 4.0));
    assert!(approx_eq(use_math::midpoint_2d(a, c).y(), 1.5));
    assert_eq!(
        use_math::try_orientation_2d(a, b, c)?,
        use_math::Orientation2::CounterClockwise
    );
    assert_eq!(
        use_math::orientation_2d_with_tolerance(a, b, c, 0.0)?,
        use_math::Orientation2::CounterClockwise
    );
    assert!(approx_eq(triangle.perimeter(), 12.0));
    assert!(bounds.contains_point(use_math::Point2::new(0.0, 1.5)));

    Ok(())
}

#[test]
fn facade_reexports_geometry_errors() {
    assert!(matches!(
        geometry::Point2::try_new(0.0, f64::NEG_INFINITY),
        Err(geometry::GeometryError::NonFiniteComponent {
            type_name: "Point2",
            component: "y",
            value: f64::NEG_INFINITY,
        })
    ));
}
