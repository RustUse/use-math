use crate::EigenError;

/// A validated eigenvector coordinate container.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Eigenvector<T> {
    coordinates: Vec<T>,
}

impl<T> Eigenvector<T> {
    /// Creates a new eigenvector from coordinate data.
    ///
    /// # Errors
    ///
    /// Returns [`EigenError::EmptyEigenvector`] when `coordinates` is empty.
    pub fn new(coordinates: Vec<T>) -> Result<Self, EigenError> {
        Self::try_new(coordinates)
    }

    /// Creates a new eigenvector from coordinate data.
    ///
    /// # Errors
    ///
    /// Returns [`EigenError::EmptyEigenvector`] when `coordinates` is empty.
    pub fn try_new(coordinates: Vec<T>) -> Result<Self, EigenError> {
        if coordinates.is_empty() {
            Err(EigenError::EmptyEigenvector)
        } else {
            Ok(Self { coordinates })
        }
    }

    /// Creates a new eigenvector from coordinate data.
    ///
    /// # Errors
    ///
    /// Returns [`EigenError::EmptyEigenvector`] when `coordinates` is empty.
    pub fn from_coordinates(coordinates: Vec<T>) -> Result<Self, EigenError> {
        Self::try_new(coordinates)
    }

    /// Returns the stored coordinates.
    #[must_use]
    pub fn coordinates(&self) -> &[T] {
        &self.coordinates
    }

    /// Returns the stored coordinates.
    #[must_use]
    pub fn into_inner(self) -> Vec<T> {
        self.coordinates
    }

    /// Returns the vector dimension.
    #[must_use]
    pub const fn dimension(&self) -> usize {
        self.coordinates.len()
    }
}

impl<T> AsRef<[T]> for Eigenvector<T> {
    fn as_ref(&self) -> &[T] {
        self.coordinates()
    }
}

#[cfg(test)]
mod tests {
    use super::Eigenvector;
    use crate::EigenError;

    #[test]
    fn constructs_an_eigenvector_and_exposes_coordinates() {
        let vector = Eigenvector::from_coordinates(vec![1_i32, 0, -1]).expect("valid eigenvector");

        assert_eq!(vector.coordinates(), &[1, 0, -1]);
        assert_eq!(vector.dimension(), 3);
    }

    #[test]
    fn rejects_empty_coordinates() {
        assert_eq!(
            Eigenvector::<i32>::new(vec![]),
            Err(EigenError::EmptyEigenvector)
        );
    }
}
