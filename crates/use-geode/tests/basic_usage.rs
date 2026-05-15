use use_catalan::catalan;
use use_geode::{
    GeodeError, TypeVector, catalan_from_geode_dimension, geode, geode_memoized, hyper_catalan,
    polygon_edge_count, polygon_vertex_count,
};

#[test]
fn direct_geode_usage_covers_core_workflow() -> Result<(), GeodeError> {
    let vector = TypeVector::new(vec![0, 1])?;

    assert_eq!(polygon_edge_count(&vector)?, 4);
    assert_eq!(polygon_vertex_count(&vector)?, 4);
    assert_eq!(hyper_catalan(&vector)?, 1);
    assert_eq!(geode(&vector)?, 3);
    assert_eq!(geode_memoized(&vector)?, 3);

    Ok(())
}

#[test]
fn first_axis_helper_matches_use_catalan() -> Result<(), GeodeError> {
    for n in 0_u64..=5 {
        let catalan_value = catalan(n).map_err(|_| GeodeError::ArithmeticOverflow)?;
        assert_eq!(catalan_from_geode_dimension(n)?, catalan_value);
    }

    Ok(())
}
