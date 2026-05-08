use use_geometry::{
    Aabb2, Circle, Line2, Orientation2, Point2, Segment2, Triangle, Vector2, try_orientation_2d,
};

fn approx_eq(left: f64, right: f64) -> bool {
    (left - right).abs() < 1.0e-10
}

fn main() -> Result<(), use_geometry::GeometryError> {
    let a = Point2::try_new(0.0, 0.0)?;
    let b = Point2::try_new(4.0, 0.0)?;
    let c = Point2::try_new(0.0, 3.0)?;

    let triangle = Triangle::try_new(a, b, c)?;
    let circle = Circle::try_new(a, 3.0)?;
    let segment = Segment2::try_new(a, b)?;
    let line = Line2::try_from_point_direction(a, Vector2::new(4.0, 0.0))?;
    let bounds = Aabb2::from_points(b, c);

    assert!(approx_eq(triangle.area(), 6.0));
    assert!(approx_eq(circle.radius(), 3.0));
    assert_eq!(segment.point_at(0.25), Point2::new(1.0, 0.0));
    assert!(line.contains_point(Point2::new(2.0, 0.0)));
    assert!(bounds.contains_point(Point2::new(2.0, 1.5)));
    assert_eq!(try_orientation_2d(a, b, c)?, Orientation2::CounterClockwise);

    Ok(())
}
