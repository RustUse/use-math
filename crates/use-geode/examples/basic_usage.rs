use use_geode::{TypeVector, geode_memoized, hyper_catalan, polygon_edge_count};

fn main() -> Result<(), use_geode::GeodeError> {
    let vector = TypeVector::new(vec![0, 1])?;

    assert_eq!(polygon_edge_count(&vector)?, 4);
    assert_eq!(hyper_catalan(&vector)?, 1);
    assert_eq!(geode_memoized(&vector)?, 3);

    Ok(())
}
