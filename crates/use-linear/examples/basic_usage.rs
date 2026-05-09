use use_linear::{Matrix2, Vector2, dot, solve_2x2};

fn main() -> Result<(), use_linear::LinearError> {
    let vector = Vector2::new(3.0, 4.0);
    let basis = Matrix2::new(2.0, 1.0, 5.0, 3.0);
    let rhs = Vector2::new(1.0, 2.0);

    assert!((dot(vector, Vector2::new(-2.0, 1.0)) + 2.0).abs() < 1.0e-12);
    assert!((vector.magnitude() - 5.0).abs() < 1.0e-12);
    assert_eq!(
        basis.mul_vector(Vector2::new(1.0, -1.0)),
        Vector2::new(1.0, 2.0)
    );
    assert_eq!(basis * Matrix2::identity(), basis);
    assert_eq!(solve_2x2(basis, rhs)?, Vector2::new(1.0, -1.0));

    Ok(())
}
