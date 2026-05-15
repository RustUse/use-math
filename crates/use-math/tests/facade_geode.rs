use use_math::prelude::*;

#[test]
fn facade_prelude_exposes_geode_workflow() -> Result<(), use_math::geode::GeodeError> {
    let vector = TypeVector::new(vec![0, 1])?;

    assert_eq!(face_count(&vector), 1);
    assert_eq!(hyper_catalan(&vector)?, 1);
    assert_eq!(geode_memoized(&vector)?, 3);

    Ok(())
}

#[test]
fn facade_root_reexports_geode_workflow() -> Result<(), use_math::GeodeError> {
    let vector = use_math::TypeVector::new(vec![0, 1])?;

    assert_eq!(use_math::face_count(&vector), 1);
    assert_eq!(use_math::hyper_catalan(&vector)?, 1);
    assert_eq!(use_math::geode::geode(&vector)?, 3);
    assert_eq!(use_math::geode::geode_memoized(&vector)?, 3);

    Ok(())
}
