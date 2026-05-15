use use_math::{TypeVector, geode_memoized, hyper_catalan};

fn main() -> Result<(), use_math::GeodeError> {
    let vector = TypeVector::new(vec![0, 1])?;

    assert_eq!(hyper_catalan(&vector)?, 1);
    assert_eq!(geode_memoized(&vector)?, 3);

    Ok(())
}
