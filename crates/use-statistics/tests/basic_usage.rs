use use_statistics::{
    StatisticsError, mean, median, population_std_dev, population_variance, sample_std_dev,
    sample_variance,
};

fn assert_close(left: f64, right: f64) {
    assert!((left - right).abs() < 1.0e-12, "left={left}, right={right}");
}

#[test]
fn direct_statistics_usage_covers_summary_helpers() -> Result<(), StatisticsError> {
    let values = [2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
    let sample = [1.0, 2.0, 3.0, 4.0];

    assert_close(mean(&values)?, 5.0);
    assert_close(median(&values)?, 4.5);
    assert_close(population_variance(&values)?, 4.0);
    assert_close(population_std_dev(&values)?, 2.0);
    assert_close(sample_variance(&sample)?, 1.666_666_666_666_666_7);
    assert_close(sample_std_dev(&sample)?, 1.290_994_448_735_805_6);

    Ok(())
}
