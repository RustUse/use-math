use crate::{EigenError, Eigenpair};

/// A non-empty collection of eigenpairs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Eigensystem<T> {
    pairs: Vec<Eigenpair<T>>,
}

impl<T> Eigensystem<T> {
    /// Creates a new eigensystem.
    ///
    /// # Errors
    ///
    /// Returns [`EigenError::EmptyEigensystem`] when `pairs` is empty.
    pub fn new(pairs: Vec<Eigenpair<T>>) -> Result<Self, EigenError> {
        Self::try_new(pairs)
    }

    /// Creates a new eigensystem.
    ///
    /// # Errors
    ///
    /// Returns [`EigenError::EmptyEigensystem`] when `pairs` is empty.
    pub fn try_new(pairs: Vec<Eigenpair<T>>) -> Result<Self, EigenError> {
        if pairs.is_empty() {
            Err(EigenError::EmptyEigensystem)
        } else {
            Ok(Self { pairs })
        }
    }

    /// Creates a new eigensystem from eigenpairs.
    ///
    /// # Errors
    ///
    /// Returns [`EigenError::EmptyEigensystem`] when `pairs` is empty.
    pub fn from_pairs(pairs: Vec<Eigenpair<T>>) -> Result<Self, EigenError> {
        Self::try_new(pairs)
    }

    /// Returns the stored eigenpairs.
    #[must_use]
    pub fn pairs(&self) -> &[Eigenpair<T>] {
        &self.pairs
    }

    /// Returns the stored eigenpairs.
    #[must_use]
    pub fn into_inner(self) -> Vec<Eigenpair<T>> {
        self.pairs
    }

    /// Returns the number of stored eigenpairs.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.pairs.len()
    }

    /// Returns whether the eigensystem is empty.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.pairs.is_empty()
    }
}

impl<T> AsRef<[Eigenpair<T>]> for Eigensystem<T> {
    fn as_ref(&self) -> &[Eigenpair<T>] {
        self.pairs()
    }
}

#[cfg(test)]
mod tests {
    use super::Eigensystem;
    use crate::{EigenError, Eigenpair, Eigenvalue, Eigenvector};

    #[test]
    fn stores_multiple_eigenpairs() {
        let pairs = vec![
            Eigenpair::new(
                Eigenvalue::new(2_i32),
                Eigenvector::new(vec![1_i32, 0]).expect("valid eigenvector"),
            ),
            Eigenpair::new(
                Eigenvalue::new(5_i32),
                Eigenvector::new(vec![0_i32, 1]).expect("valid eigenvector"),
            ),
        ];

        let system = Eigensystem::new(pairs).expect("valid eigensystem");

        assert_eq!(system.len(), 2);
        assert_eq!(*system.pairs()[0].value().as_ref(), 2);
        assert_eq!(system.pairs()[1].vector().coordinates(), &[0, 1]);
    }

    #[test]
    fn rejects_an_empty_eigensystem() {
        assert_eq!(
            Eigensystem::<i32>::new(vec![]),
            Err(EigenError::EmptyEigensystem)
        );
    }
}
