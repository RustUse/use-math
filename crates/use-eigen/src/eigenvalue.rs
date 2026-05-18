/// A scalar eigenvalue wrapper.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Eigenvalue<T> {
    value: T,
}

impl<T> Eigenvalue<T> {
    /// Creates a new eigenvalue.
    #[must_use]
    pub const fn new(value: T) -> Self {
        Self { value }
    }

    /// Creates a new eigenvalue from a scalar value.
    #[must_use]
    pub const fn from_value(value: T) -> Self {
        Self::new(value)
    }

    /// Returns the stored scalar value by reference.
    #[must_use]
    pub const fn as_ref(&self) -> &T {
        &self.value
    }

    /// Returns the stored scalar value.
    #[must_use]
    pub fn into_inner(self) -> T {
        self.value
    }
}

impl<T> AsRef<T> for Eigenvalue<T> {
    fn as_ref(&self) -> &T {
        self.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::Eigenvalue;

    #[test]
    fn constructs_and_exposes_an_eigenvalue() {
        let value = Eigenvalue::from_value(7_i32);

        assert_eq!(*value.as_ref(), 7);
        assert_eq!(value.into_inner(), 7);
    }
}
