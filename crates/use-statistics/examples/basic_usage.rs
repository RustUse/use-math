use use_statistics::{
    mean, median, population_std_dev, population_variance, sample_std_dev, sample_variance,
};

fn main() -> Result<(), use_statistics::StatisticsError> {
    let values = [2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];

    assert!((mean(&values)? - 5.0).abs() < 1.0e-12);
    assert!((median(&values)? - 4.5).abs() < 1.0e-12);
    assert!((population_variance(&values)? - 4.0).abs() < 1.0e-12);
    assert!((population_std_dev(&values)? - 2.0).abs() < 1.0e-12);

    let sample = [1.0, 2.0, 3.0, 4.0];

    assert!((sample_variance(&sample)? - 1.666_666_666_666_666_7).abs() < 1.0e-12);
    assert!((sample_std_dev(&sample)? - 1.290_994_448_735_805_6).abs() < 1.0e-12);

    Ok(())
}
