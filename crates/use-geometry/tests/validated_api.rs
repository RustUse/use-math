use use_geometry::{
    Aabb2, Circle, GeometryError, Line2, Orientation2, Point2, Segment2, Triangle, Vector2,
    distance_2d, orientation_2d_with_tolerance, try_orientation_2d,
};

fn approx_eq(left: f64, right: f64) -> bool {
    (left - right).abs() < 1.0e-10
}

#[test]
fn validated_geometry_types_compose_across_modules() -> Result<(), GeometryError> {
    let a = Point2::try_new(0.0, 0.0)?;
    let b = Point2::try_new(4.0, 0.0)?;
    let c = Point2::try_new(0.0, 3.0)?;

    let segment = Segment2::try_new(a, b)?;
    let line = Line2::try_new(a, c)?;
    let triangle = Triangle::try_new(a, b, c)?;
    let circle = Circle::try_new(a, 3.0)?;
    let offset = Vector2::try_from_points(a, b)?;
    let bounds = Aabb2::from_points(a, c);

    assert!(approx_eq(distance_2d(a, b), 4.0));
    assert!(approx_eq(a.distance_squared_to(c), 9.0));
    assert!(approx_eq(segment.length(), 4.0));
    assert_eq!(line.try_slope()?, None);
    assert_eq!(try_orientation_2d(a, b, c)?, Orientation2::CounterClockwise);
    assert_eq!(
        orientation_2d_with_tolerance(a, b, c, 0.0)?,
        Orientation2::CounterClockwise
    );
    assert!(approx_eq(triangle.area(), 6.0));
    assert!(approx_eq(
        circle.circumference(),
        6.0 * core::f64::consts::PI
    ));
    assert_eq!(a.translate(offset), Point2::new(4.0, 0.0));
    assert!(circle.contains_point(Point2::new(0.0, 3.0)));
    assert!(bounds.contains_point(Point2::new(0.0, 1.5)));

    Ok(())
}

#[test]
fn validated_geometry_constructors_reject_non_finite_inputs() {
    assert!(matches!(
        Point2::try_new(f64::NAN, 0.0),
        Err(GeometryError::NonFiniteComponent {
            type_name: "Point2",
            component: "x",
            value,
        }) if value.is_nan()
    ));

    assert_eq!(
        Segment2::try_new(Point2::new(0.0, 0.0), Point2::new(1.0, f64::INFINITY)),
        Err(GeometryError::NonFiniteComponent {
            type_name: "Point2",
            component: "y",
            value: f64::INFINITY,
        })
    );
}

#[test]
fn validated_geometry_tolerance_workflow_composes_across_types() -> Result<(), GeometryError> {
    let center = Point2::try_new(2.0, 2.0)?;
    let circle = Circle::try_new(center, 1.5)?;
    let bounds = circle.aabb();
    let diameter = Segment2::try_new(Point2::new(0.5, 2.0), Point2::new(3.5, 2.0))?;
    let line = Line2::try_from_point_direction(center, Vector2::new(1.0, 0.0))?;

    assert_eq!(bounds.min(), Point2::new(0.5, 0.5));
    assert_eq!(bounds.max(), Point2::new(3.5, 3.5));
    assert!(bounds.contains_point(center));
    assert!(bounds.contains_point_with_tolerance(Point2::new(3.6, 2.0), 0.1)?);
    assert!(circle.contains_point_with_tolerance(diameter.end(), 0.0)?);
    assert_eq!(diameter.aabb().min(), Point2::new(0.5, 2.0));
    assert_eq!(diameter.aabb().max(), Point2::new(3.5, 2.0));
    assert!(line.contains_point_with_tolerance(Point2::new(4.0, 2.05), 0.05)?);

    Ok(())
}

#[test]
fn validated_geometry_tolerance_helpers_reject_negative_tolerance_consistently()
-> Result<(), GeometryError> {
    let bounds = Aabb2::from_points(Point2::new(0.0, 0.0), Point2::new(2.0, 2.0));
    let circle = Circle::try_new(Point2::new(1.0, 1.0), 1.0)?;
    let line = Line2::try_new(Point2::new(0.0, 0.0), Point2::new(2.0, 2.0))?;

    assert_eq!(
        bounds.contains_point_with_tolerance(Point2::new(2.0, 2.0), -0.1),
        Err(GeometryError::NegativeTolerance(-0.1))
    );
    assert_eq!(
        circle.contains_point_with_tolerance(Point2::new(2.0, 1.0), -0.1),
        Err(GeometryError::NegativeTolerance(-0.1))
    );
    assert_eq!(
        line.contains_point_with_tolerance(Point2::new(1.0, 1.0), -0.1),
        Err(GeometryError::NegativeTolerance(-0.1))
    );

    Ok(())
}
