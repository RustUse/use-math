use use_linear::solve_2x2;
use use_matrix::Matrix2;
use use_vector::Vector2;

fn main() -> Result<(), use_linear::LinearError> {
    let basis = Matrix2::new(2.0, 1.0, 5.0, 3.0);
    let rhs = Vector2::new(1.0, 2.0);

    assert_eq!(solve_2x2(basis, rhs)?, Vector2::new(1.0, -1.0));

    Ok(())
}
