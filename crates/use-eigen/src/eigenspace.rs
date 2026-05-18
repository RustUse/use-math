use crate::{EigenError, Eigenvalue, Eigenvector};

/// The eigenspace associated with one eigenvalue.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EigenSpace<T> {
    value: Eigenvalue<T>,
    basis: Vec<Eigenvector<T>>,
}

impl<T> EigenSpace<T> {
    /// Creates a new eigenspace from an eigenvalue and basis vectors.
    ///
    /// # Errors
    ///
    /// Returns [`EigenError::EmptyEigenspace`] when `basis` is empty.
    pub fn new(value: Eigenvalue<T>, basis: Vec<Eigenvector<T>>) -> Result<Self, EigenError> {
        Self::try_new(value, basis)
    }

    /// Creates a new eigenspace from an eigenvalue and basis vectors.
    ///
    /// # Errors
    ///
    /// Returns [`EigenError::EmptyEigenspace`] when `basis` is empty.
    pub fn try_new(value: Eigenvalue<T>, basis: Vec<Eigenvector<T>>) -> Result<Self, EigenError> {
        if basis.is_empty() {
            Err(EigenError::EmptyEigenspace)
        } else {
            Ok(Self { value, basis })
        }
    }

    /// Returns the associated eigenvalue.
    #[must_use]
    pub const fn value(&self) -> &Eigenvalue<T> {
        &self.value
    }

    /// Returns the basis vectors.
    #[must_use]
    pub fn basis(&self) -> &[Eigenvector<T>] {
        &self.basis
    }

    /// Returns the stored eigenvalue and basis vectors.
    #[must_use]
    pub fn into_inner(self) -> (Eigenvalue<T>, Vec<Eigenvector<T>>) {
        (self.value, self.basis)
    }

    /// Returns the number of basis vectors.
    #[must_use]
    pub const fn dimension(&self) -> usize {
        self.basis.len()
    }
}

#[cfg(test)]
mod tests {
    use super::EigenSpace;
    use crate::{EigenError, Eigenvalue, Eigenvector};

    #[test]
    fn stores_an_eigenspace_basis() {
        let space = EigenSpace::new(
            Eigenvalue::new(4_i32),
            vec![Eigenvector::new(vec![1_i32, 1]).expect("valid eigenvector")],
        )
        .expect("valid eigenspace");

        assert_eq!(*space.value().as_ref(), 4);
        assert_eq!(space.dimension(), 1);
        assert_eq!(space.basis()[0].coordinates(), &[1, 1]);
    }

    #[test]
    fn rejects_an_empty_basis() {
        assert_eq!(
            EigenSpace::<i32>::new(Eigenvalue::new(1_i32), vec![]),
            Err(EigenError::EmptyEigenspace)
        );
    }
}
