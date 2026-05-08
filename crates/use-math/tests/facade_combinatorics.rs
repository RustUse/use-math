use use_math::prelude::*;

#[test]
fn facade_prelude_exposes_combinatorics_workflow()
-> Result<(), use_math::combinatorics::CombinatoricsError> {
    assert_eq!(factorial(5)?, 120);
    assert_eq!(permutations(5, 3)?, 60);
    assert_eq!(combinations(5, 2)?, 10);

    Ok(())
}

#[test]
fn facade_root_reexports_combinatorics_workflow() -> Result<(), use_math::CombinatoricsError> {
    assert_eq!(use_math::factorial(5)?, 120);
    assert_eq!(use_math::permutations(5, 3)?, 60);
    assert_eq!(use_math::combinations(5, 2)?, 10);

    Ok(())
}

#[test]
fn facade_reexports_combinatorics_errors() {
    assert_eq!(
        use_math::combinatorics::combinations(3, 4),
        Err(use_math::combinatorics::CombinatoricsError::KExceedsN { n: 3, k: 4 })
    );
}
