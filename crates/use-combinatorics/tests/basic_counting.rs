use use_combinatorics::{CombinatoricsError, combinations, factorial, permutations};

#[test]
fn direct_combinatorics_api_covers_basic_counting() -> Result<(), CombinatoricsError> {
    assert_eq!(factorial(6)?, 720);
    assert_eq!(permutations(6, 2)?, 30);
    assert_eq!(combinations(6, 2)?, 15);

    Ok(())
}

#[test]
fn direct_combinatorics_api_reports_invalid_inputs() {
    assert_eq!(
        combinations(2, 3),
        Err(CombinatoricsError::KExceedsN { n: 2, k: 3 })
    );
}
