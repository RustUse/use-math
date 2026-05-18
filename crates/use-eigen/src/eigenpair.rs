use crate::{Eigenvalue, Eigenvector};

/// An eigenvalue paired with one eigenvector.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Eigenpair<T> {
    value: Eigenvalue<T>,
    vector: Eigenvector<T>,
}

impl<T> Eigenpair<T> {
    /// Creates a new eigenpair.
    #[must_use]
    pub const fn new(value: Eigenvalue<T>, vector: Eigenvector<T>) -> Self {
        Self { value, vector }
    }

    /// Returns the stored eigenvalue.
    #[must_use]
    pub const fn value(&self) -> &Eigenvalue<T> {
        &self.value
    }

    /// Returns the stored eigenvector.
    #[must_use]
    pub const fn vector(&self) -> &Eigenvector<T> {
        &self.vector
    }

    /// Returns the stored eigenvalue and eigenvector.
    #[must_use]
    pub fn into_inner(self) -> (Eigenvalue<T>, Eigenvector<T>) {
        (self.value, self.vector)
    }
}

#[cfg(test)]
mod tests {
    use super::Eigenpair;
    use crate::{Eigenvalue, Eigenvector};

    #[test]
    fn pairs_an_eigenvalue_with_an_eigenvector() {
        let pair = Eigenpair::new(
            Eigenvalue::new(3_i32),
            Eigenvector::new(vec![1_i32, 0]).expect("valid eigenvector"),
        );

        assert_eq!(*pair.value().as_ref(), 3);
        assert_eq!(pair.vector().coordinates(), &[1, 0]);
    }
}
