use use_math::{combinations, factorial, permutations};

fn main() -> Result<(), use_math::CombinatoricsError> {
    assert_eq!(factorial(5)?, 120);
    assert_eq!(permutations(5, 3)?, 60);
    assert_eq!(combinations(5, 2)?, 10);

    Ok(())
}
