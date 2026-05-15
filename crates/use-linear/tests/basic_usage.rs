use use_linear::{LinearError, solve_2x2};
use use_matrix::Matrix2;
use use_vector::Vector2;

#[test]
fn direct_linear_usage_solves_small_systems() -> Result<(), LinearError> {
    let matrix = Matrix2::new(2.0, 1.0, 5.0, 3.0);
    let solution = solve_2x2(matrix, Vector2::new(1.0, 2.0))?;

    assert_eq!(solution, Vector2::new(1.0, -1.0));
    assert_eq!(matrix * solution, Vector2::new(1.0, 2.0));

    Ok(())
}
