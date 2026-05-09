use crate::catalan::catalan;

/// An iterator that yields Catalan numbers in order, starting from C(0) = 1.
///
/// The iterator stops cleanly when the next value would overflow `u128`.
///
/// # Examples
///
/// ```rust
/// use use_catalan::CatalanSequence;
///
/// let first_six: Vec<u128> = CatalanSequence::new().take(6).collect();
/// assert_eq!(first_six, vec![1, 1, 2, 5, 14, 42]);
/// ```
pub struct CatalanSequence {
    /// Current Catalan number C(index).
    current: Option<u128>,
    /// Index of the current value.
    index: u32,
}

impl CatalanSequence {
    /// Creates a new `CatalanSequence` starting at C(0) = 1.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            current: Some(1),
            index: 0,
        }
    }
}

impl Default for CatalanSequence {
    fn default() -> Self {
        Self::new()
    }
}

impl Iterator for CatalanSequence {
    type Item = u128;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.current?;
        // Advance to the next Catalan number before returning.
        self.current = catalan(self.index.checked_add(1)?);
        self.index = self.index.saturating_add(1);
        Some(value)
    }
}

#[cfg(test)]
mod tests {
    use super::CatalanSequence;

    #[test]
    fn iterator_yields_first_six() {
        let values: Vec<u128> = CatalanSequence::new().take(6).collect();
        assert_eq!(values, vec![1, 1, 2, 5, 14, 42]);
    }

    #[test]
    fn iterator_terminates_before_overflow() {
        // There are exactly 70 valid u128 Catalan numbers (C(0)..=C(69)).
        let count = CatalanSequence::new().count();
        assert_eq!(count, 70, "expected 70 Catalan numbers to fit in u128");
    }

    #[test]
    fn default_is_same_as_new() {
        let a: Vec<u128> = CatalanSequence::new().take(5).collect();
        let b: Vec<u128> = CatalanSequence::default().take(5).collect();
        assert_eq!(a, b);
    }
}
