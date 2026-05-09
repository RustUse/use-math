use use_linear::{LinearError, Matrix2, Vector2, dot, solve_2x2};

fn assert_close(left: f64, right: f64) {
    assert!((left - right).abs() < 1.0e-12, "left={left}, right={right}");
}

#[test]
fn direct_linear_usage_covers_vectors_matrices_and_solves() -> Result<(), LinearError> {
    let vector = Vector2::new(3.0, 4.0);
    let other = Vector2::new(-2.0, 1.0);
    let matrix = Matrix2::new(2.0, 1.0, 5.0, 3.0);

    assert_eq!(vector + other, Vector2::new(1.0, 5.0));
    assert_eq!(vector - other, Vector2::new(5.0, 3.0));
    assert_close(dot(vector, other), -2.0);
    assert_close(vector.magnitude(), 5.0);
    assert_eq!(
        matrix.mul_vector(Vector2::new(1.0, -1.0)),
        Vector2::new(1.0, 2.0)
    );
    assert_eq!(matrix * Matrix2::identity(), matrix);
    assert_eq!(
        solve_2x2(matrix, Vector2::new(1.0, 2.0))?,
        Vector2::new(1.0, -1.0)
    );

    Ok(())
}
