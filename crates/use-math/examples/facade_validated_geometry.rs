use use_math::{Orientation2, Point2, Triangle, distance_2d, try_orientation_2d};

fn approx_eq(left: f64, right: f64) -> bool {
    (left - right).abs() < 1.0e-10
}

fn main() -> Result<(), use_math::GeometryError> {
    let a = Point2::try_new(0.0, 0.0)?;
    let b = Point2::try_new(4.0, 0.0)?;
    let c = Point2::try_new(0.0, 3.0)?;

    let triangle = Triangle::try_new(a, b, c)?;

    assert!(approx_eq(distance_2d(a, b), 4.0));
    assert!(approx_eq(triangle.perimeter(), 12.0));
    assert_eq!(try_orientation_2d(a, b, c)?, Orientation2::CounterClockwise);

    Ok(())
}
