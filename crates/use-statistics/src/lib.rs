#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Statistical utilities for `RustUse`.

/// Errors returned by statistics helpers when the input cannot support the requested summary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StatisticsError {
    /// The input slice was empty.
    EmptyInput,
    /// The input slice does not contain enough values for the requested operation.
    InsufficientData { needed: usize, actual: usize },
    /// The input slice is too large to convert its length into an exact `f64` count.
    TooManyValues { actual: usize },
}

impl core::fmt::Display for StatisticsError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::EmptyInput => formatter.write_str("statistics input must not be empty"),
            Self::InsufficientData { needed, actual } => write!(
                formatter,
                "statistics operation requires at least {needed} values, received {actual}"
            ),
            Self::TooManyValues { actual } => write!(
                formatter,
                "statistics input length {actual} exceeds the exact counting range supported by this helper"
            ),
        }
    }
}

impl std::error::Error for StatisticsError {}

/// Returns the arithmetic mean of `values`.
///
/// # Errors
///
/// Returns [`StatisticsError::EmptyInput`] when `values` is empty, or
/// [`StatisticsError::TooManyValues`] if the slice length cannot be represented
/// exactly in the internal floating-point count conversion.
pub fn mean(values: &[f64]) -> Result<f64, StatisticsError> {
    ensure_non_empty(values)?;
    let count = exact_len_as_f64(values.len())?;

    Ok(values.iter().sum::<f64>() / count)
}

/// Returns the median of `values`.
///
/// # Errors
///
/// Returns [`StatisticsError::EmptyInput`] when `values` is empty.
pub fn median(values: &[f64]) -> Result<f64, StatisticsError> {
    ensure_non_empty(values)?;

    let mut sorted = values.to_vec();
    sorted.sort_by(f64::total_cmp);

    let middle = sorted.len() / 2;

    if sorted.len().is_multiple_of(2) {
        Ok(f64::midpoint(sorted[middle - 1], sorted[middle]))
    } else {
        Ok(sorted[middle])
    }
}

/// Returns the population variance of `values`.
///
/// # Errors
///
/// Returns [`StatisticsError::EmptyInput`] when `values` is empty, or
/// [`StatisticsError::TooManyValues`] if the slice length cannot be represented
/// exactly in the internal floating-point count conversion.
pub fn population_variance(values: &[f64]) -> Result<f64, StatisticsError> {
    ensure_non_empty(values)?;
    let count = exact_len_as_f64(values.len())?;

    variance_with_denominator(values, count)
}

/// Returns the sample variance of `values` using Bessel's correction.
///
/// # Errors
///
/// Returns [`StatisticsError::InsufficientData`] when fewer than two values are
/// provided, or [`StatisticsError::TooManyValues`] if the slice length cannot
/// be represented exactly in the internal floating-point count conversion.
pub fn sample_variance(values: &[f64]) -> Result<f64, StatisticsError> {
    ensure_len_at_least(values, 2)?;
    let count = exact_len_as_f64(values.len() - 1)?;

    variance_with_denominator(values, count)
}

/// Returns the population standard deviation of `values`.
///
/// # Errors
///
/// Propagates any error returned by [`population_variance`].
pub fn population_std_dev(values: &[f64]) -> Result<f64, StatisticsError> {
    Ok(population_variance(values)?.sqrt())
}

/// Returns the sample standard deviation of `values` using Bessel's correction.
///
/// # Errors
///
/// Propagates any error returned by [`sample_variance`].
pub fn sample_std_dev(values: &[f64]) -> Result<f64, StatisticsError> {
    Ok(sample_variance(values)?.sqrt())
}

fn variance_with_denominator(values: &[f64], denominator: f64) -> Result<f64, StatisticsError> {
    let average = mean(values)?;
    let squared_deviations = values
        .iter()
        .map(|value| {
            let delta = *value - average;
            delta * delta
        })
        .sum::<f64>();

    Ok(squared_deviations / denominator)
}

const fn ensure_non_empty(values: &[f64]) -> Result<(), StatisticsError> {
    ensure_len_at_least(values, 1)
}

const fn ensure_len_at_least(values: &[f64], needed: usize) -> Result<(), StatisticsError> {
    if values.len() < needed {
        if values.is_empty() {
            Err(StatisticsError::EmptyInput)
        } else {
            Err(StatisticsError::InsufficientData {
                needed,
                actual: values.len(),
            })
        }
    } else {
        Ok(())
    }
}

fn exact_len_as_f64(length: usize) -> Result<f64, StatisticsError> {
    let count =
        u32::try_from(length).map_err(|_| StatisticsError::TooManyValues { actual: length })?;

    Ok(f64::from(count))
}

pub mod prelude;

#[cfg(test)]
mod tests {
    use super::{
        StatisticsError, mean, median, population_std_dev, population_variance, sample_std_dev,
        sample_variance,
    };

    fn assert_close(left: f64, right: f64) {
        assert!((left - right).abs() < 1.0e-12, "left={left}, right={right}");
    }

    #[test]
    fn computes_common_summary_statistics() -> Result<(), StatisticsError> {
        let values = [2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];

        assert_close(mean(&values)?, 5.0);
        assert_close(median(&values)?, 4.5);
        assert_close(population_variance(&values)?, 4.0);
        assert_close(population_std_dev(&values)?, 2.0);

        Ok(())
    }

    #[test]
    fn computes_sample_summaries_and_reports_invalid_inputs() {
        let values = [1.0, 2.0, 3.0, 4.0];

        assert_close(
            sample_variance(&values).expect("sample variance should succeed"),
            1.666_666_666_666_666_7,
        );
        assert_close(
            sample_std_dev(&values).expect("sample std dev should succeed"),
            1.290_994_448_735_805_6,
        );
        assert_eq!(mean(&[]), Err(StatisticsError::EmptyInput));
        assert_eq!(
            sample_variance(&[3.0]),
            Err(StatisticsError::InsufficientData {
                needed: 2,
                actual: 1,
            })
        );
    }
}
