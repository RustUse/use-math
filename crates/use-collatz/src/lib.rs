#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Collatz trajectory utilities for `RustUse`.

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
/// The parity of a positive integer in a Collatz trajectory.
pub enum CollatzParity {
    /// The value is divisible by `2`.
    Even,
    /// The value is not divisible by `2`.
    Odd,
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Summary information for an inclusive range checked with [`verify_range`].
///
/// `max_total_stopping_time` stores `(input, total_stopping_time)`.
///
/// `max_trajectory_value` stores `(input, max_value_in_trajectory)`.
pub struct CollatzRangeSummary {
    /// The inclusive lower bound that was requested.
    pub start: u64,
    /// The inclusive upper bound that was requested.
    pub end: u64,
    /// The number of positive inputs checked in the inclusive range.
    pub checked: u64,
    /// The number of checked inputs whose trajectories reached `1` without overflow.
    pub reached_one: u64,
    /// The number of checked inputs whose odd-step arithmetic overflowed.
    pub overflowed: u64,
    /// The input with the largest total stopping time, if any values were checked.
    pub max_total_stopping_time: Option<(u64, u64)>,
    /// The input with the largest peak trajectory value, if any values were checked.
    pub max_trajectory_value: Option<(u64, u64)>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct TrajectoryMetrics {
    stopping_time: u64,
    total_stopping_time: u64,
    max_value: u64,
}

const fn is_even(n: u64) -> bool {
    (n & 1) == 0
}

const fn checked_odd_step(n: u64) -> Option<u64> {
    match n.checked_mul(3) {
        Some(value) => value.checked_add(1),
        None => None,
    }
}

fn trajectory_metrics(n: u64) -> Option<TrajectoryMetrics> {
    if n == 0 {
        return None;
    }

    let start = n;
    let mut current = n;
    let mut stopping_time = if n == 1 { Some(0) } else { None };
    let mut total_stopping_time = 0_u64;
    let mut max_value = n;

    while current != 1 {
        current = collatz_next(current)?;
        total_stopping_time = total_stopping_time.checked_add(1)?;
        max_value = max_value.max(current);

        if stopping_time.is_none() && current < start {
            stopping_time = Some(total_stopping_time);
        }
    }

    Some(TrajectoryMetrics {
        stopping_time: stopping_time.unwrap_or(total_stopping_time),
        total_stopping_time,
        max_value,
    })
}

fn update_max_pair(slot: &mut Option<(u64, u64)>, input: u64, value: u64) {
    if slot.is_none_or(|(_, current_max)| value > current_max) {
        *slot = Some((input, value));
    }
}

/// Returns the next value in the Collatz trajectory for a positive integer.
///
/// Returns `None` when `n == 0` or when the checked odd step `3 * n + 1`
/// overflows `u64`.
///
/// # Examples
///
/// ```rust
/// use use_collatz::collatz_next;
///
/// assert_eq!(collatz_next(1), Some(4));
/// assert_eq!(collatz_next(2), Some(1));
/// assert_eq!(collatz_next(3), Some(10));
/// assert_eq!(collatz_next(0), None);
/// ```
#[must_use]
pub fn collatz_next(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }

    if is_even(n) {
        Some(n / 2)
    } else {
        checked_odd_step(n)
    }
}

/// Returns the full Collatz trajectory from `n` down to `1`.
///
/// Returns `None` when `n == 0` or when a checked odd step overflows.
///
/// # Examples
///
/// ```rust
/// use use_collatz::collatz_sequence;
///
/// assert_eq!(collatz_sequence(6), Some(vec![6, 3, 10, 5, 16, 8, 4, 2, 1]));
/// assert_eq!(collatz_sequence(0), None);
/// ```
#[must_use]
pub fn collatz_sequence(n: u64) -> Option<Vec<u64>> {
    if n == 0 {
        return None;
    }

    let mut current = n;
    let mut sequence = vec![current];

    while current != 1 {
        current = collatz_next(current)?;
        sequence.push(current);
    }

    Some(sequence)
}

/// Returns the number of steps needed to first reach a value smaller than `n`.
///
/// Returns `Some(0)` for `n == 1`.
///
/// Returns `None` when `n == 0` or when a checked odd step overflows.
///
/// # Examples
///
/// ```rust
/// use use_collatz::stopping_time;
///
/// assert_eq!(stopping_time(1), Some(0));
/// assert_eq!(stopping_time(6), Some(1));
/// assert_eq!(stopping_time(3), Some(6));
/// ```
#[must_use]
pub fn stopping_time(n: u64) -> Option<u64> {
    trajectory_metrics(n).map(|metrics| metrics.stopping_time)
}

/// Returns the number of steps needed to reach `1`.
///
/// Returns `None` when `n == 0` or when a checked odd step overflows.
///
/// # Examples
///
/// ```rust
/// use use_collatz::total_stopping_time;
///
/// assert_eq!(total_stopping_time(1), Some(0));
/// assert_eq!(total_stopping_time(6), Some(8));
/// ```
#[must_use]
pub fn total_stopping_time(n: u64) -> Option<u64> {
    trajectory_metrics(n).map(|metrics| metrics.total_stopping_time)
}

/// Returns the full trajectory length including the starting value and terminal `1`.
///
/// Returns `None` when `n == 0` or when a checked odd step overflows.
///
/// # Examples
///
/// ```rust
/// use use_collatz::trajectory_len;
///
/// assert_eq!(trajectory_len(1), Some(1));
/// assert_eq!(trajectory_len(6), Some(9));
/// ```
#[must_use]
pub fn trajectory_len(n: u64) -> Option<u64> {
    total_stopping_time(n)?.checked_add(1)
}

/// Returns the largest value reached in the trajectory from `n` to `1`.
///
/// Returns `None` when `n == 0` or when a checked odd step overflows.
///
/// # Examples
///
/// ```rust
/// use use_collatz::max_value_in_trajectory;
///
/// assert_eq!(max_value_in_trajectory(1), Some(1));
/// assert_eq!(max_value_in_trajectory(6), Some(16));
/// ```
#[must_use]
pub fn max_value_in_trajectory(n: u64) -> Option<u64> {
    trajectory_metrics(n).map(|metrics| metrics.max_value)
}

/// Returns `true` when the trajectory reaches `1` without overflow.
///
/// Returns `false` for `n == 0` or when a checked odd step overflows.
///
/// # Examples
///
/// ```rust
/// use use_collatz::reaches_one;
///
/// assert!(reaches_one(6));
/// assert!(!reaches_one(0));
/// ```
#[must_use]
pub fn reaches_one(n: u64) -> bool {
    trajectory_metrics(n).is_some()
}

/// Returns the parity of a positive input.
///
/// Returns `None` when `n == 0`.
///
/// # Examples
///
/// ```rust
/// use use_collatz::{CollatzParity, parity};
///
/// assert_eq!(parity(8), Some(CollatzParity::Even));
/// assert_eq!(parity(9), Some(CollatzParity::Odd));
/// assert_eq!(parity(0), None);
/// ```
#[must_use]
pub fn parity(n: u64) -> Option<CollatzParity> {
    if n == 0 {
        return None;
    }

    if is_even(n) {
        Some(CollatzParity::Even)
    } else {
        Some(CollatzParity::Odd)
    }
}

/// Returns the parity pattern for the trajectory from `n` to `1`.
///
/// The returned vector includes the starting value and every nonterminal value
/// in the trajectory, but it excludes the final `1`. For `n == 1`, this means
/// the parity vector is empty.
///
/// Returns `None` when `n == 0` or when a checked odd step overflows.
///
/// # Examples
///
/// ```rust
/// use use_collatz::{CollatzParity, parity_vector};
///
/// assert_eq!(
///     parity_vector(6),
///     Some(vec![
///         CollatzParity::Even,
///         CollatzParity::Odd,
///         CollatzParity::Even,
///         CollatzParity::Odd,
///         CollatzParity::Even,
///         CollatzParity::Even,
///         CollatzParity::Even,
///         CollatzParity::Even,
///     ])
/// );
/// assert_eq!(parity_vector(1), Some(vec![]));
/// ```
#[must_use]
pub fn parity_vector(n: u64) -> Option<Vec<CollatzParity>> {
    if n == 0 {
        return None;
    }

    let mut current = n;
    let mut parities = Vec::new();

    while current != 1 {
        parities.push(parity(current)?);
        current = collatz_next(current)?;
    }

    Some(parities)
}

/// Verifies the inclusive range `[start, end]` with bounded Collatz exploration.
///
/// The function skips `0`, counts the number of positive inputs checked, counts
/// how many trajectories reach `1`, counts how many overflow during checked odd
/// steps, tracks the input with the largest total stopping time, and tracks the
/// input with the largest peak trajectory value.
///
/// When `start > end`, the returned summary is empty and no values are checked.
///
/// # Examples
///
/// ```rust
/// use use_collatz::verify_range;
///
/// let summary = verify_range(1, 10);
///
/// assert_eq!(summary.checked, 10);
/// assert_eq!(summary.reached_one, 10);
/// assert_eq!(summary.overflowed, 0);
/// assert_eq!(summary.max_total_stopping_time, Some((9, 19)));
/// assert_eq!(summary.max_trajectory_value, Some((7, 52)));
/// ```
#[must_use]
pub fn verify_range(start: u64, end: u64) -> CollatzRangeSummary {
    let mut summary = CollatzRangeSummary {
        start,
        end,
        checked: 0,
        reached_one: 0,
        overflowed: 0,
        max_total_stopping_time: None,
        max_trajectory_value: None,
    };

    if start > end {
        return summary;
    }

    for input in start..=end {
        if input == 0 {
            continue;
        }

        summary.checked += 1;

        if let Some(metrics) = trajectory_metrics(input) {
            summary.reached_one += 1;
            update_max_pair(
                &mut summary.max_total_stopping_time,
                input,
                metrics.total_stopping_time,
            );
            update_max_pair(&mut summary.max_trajectory_value, input, metrics.max_value);
        } else {
            summary.overflowed += 1;
        }
    }

    summary
}

#[cfg(test)]
mod tests {
    use super::{
        CollatzParity, CollatzRangeSummary, collatz_next, collatz_sequence,
        max_value_in_trajectory, parity, parity_vector, reaches_one, stopping_time,
        total_stopping_time, trajectory_len, verify_range,
    };

    #[test]
    fn computes_next_values() {
        assert_eq!(collatz_next(1), Some(4));
        assert_eq!(collatz_next(2), Some(1));
        assert_eq!(collatz_next(3), Some(10));
        assert_eq!(collatz_next(0), None);
    }

    #[test]
    fn detects_checked_overflow_for_odd_steps() {
        assert_eq!(collatz_next(u64::MAX), None);
    }

    #[test]
    fn builds_full_sequences() {
        assert_eq!(collatz_sequence(6), Some(vec![6, 3, 10, 5, 16, 8, 4, 2, 1]));
        assert_eq!(collatz_sequence(0), None);
    }

    #[test]
    fn computes_total_stopping_times() {
        assert_eq!(total_stopping_time(1), Some(0));
        assert_eq!(total_stopping_time(6), Some(8));
    }

    #[test]
    fn computes_trajectory_lengths() {
        assert_eq!(trajectory_len(1), Some(1));
        assert_eq!(trajectory_len(6), Some(9));
    }

    #[test]
    fn computes_trajectory_maxima() {
        assert_eq!(max_value_in_trajectory(6), Some(16));
        assert_eq!(max_value_in_trajectory(1), Some(1));
    }

    #[test]
    fn computes_stopping_times() {
        assert_eq!(stopping_time(1), Some(0));
        assert_eq!(stopping_time(6), Some(1));
        assert_eq!(stopping_time(3), Some(6));
    }

    #[test]
    fn reports_reachability_without_panicking() {
        assert!(reaches_one(6));
        assert!(!reaches_one(0));
    }

    #[test]
    fn classifies_parity_helpers() {
        assert_eq!(parity(8), Some(CollatzParity::Even));
        assert_eq!(parity(9), Some(CollatzParity::Odd));
        assert_eq!(parity(0), None);
    }

    #[test]
    fn builds_parity_vectors_without_terminal_one() {
        assert_eq!(
            parity_vector(6),
            Some(vec![
                CollatzParity::Even,
                CollatzParity::Odd,
                CollatzParity::Even,
                CollatzParity::Odd,
                CollatzParity::Even,
                CollatzParity::Even,
                CollatzParity::Even,
                CollatzParity::Even,
            ])
        );
        assert_eq!(parity_vector(1), Some(vec![]));
        assert_eq!(parity_vector(0), None);
    }

    #[test]
    fn verifies_bounded_ranges() {
        assert_eq!(
            verify_range(1, 10),
            CollatzRangeSummary {
                start: 1,
                end: 10,
                checked: 10,
                reached_one: 10,
                overflowed: 0,
                max_total_stopping_time: Some((9, 19)),
                max_trajectory_value: Some((7, 52)),
            }
        );
    }

    #[test]
    fn skips_zero_and_allows_empty_ranges() {
        assert_eq!(
            verify_range(0, 0),
            CollatzRangeSummary {
                start: 0,
                end: 0,
                checked: 0,
                reached_one: 0,
                overflowed: 0,
                max_total_stopping_time: None,
                max_trajectory_value: None,
            }
        );

        assert_eq!(
            verify_range(10, 1),
            CollatzRangeSummary {
                start: 10,
                end: 1,
                checked: 0,
                reached_one: 0,
                overflowed: 0,
                max_total_stopping_time: None,
                max_trajectory_value: None,
            }
        );
    }
}
