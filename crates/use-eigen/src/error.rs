use core::fmt;

/// Errors returned by validated eigen primitives.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EigenError {
    /// The provided eigenvector coordinates were empty.
    EmptyEigenvector,
    /// The provided eigensystem contained no eigenpairs.
    EmptyEigensystem,
    /// The provided eigenspace basis contained no eigenvectors.
    EmptyEigenspace,
}

impl fmt::Display for EigenError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyEigenvector => {
                formatter.write_str("eigenvector coordinates must not be empty")
            },
            Self::EmptyEigensystem => {
                formatter.write_str("eigensystem must contain at least one eigenpair")
            },
            Self::EmptyEigenspace => {
                formatter.write_str("eigenspace basis must contain at least one eigenvector")
            },
        }
    }
}

impl std::error::Error for EigenError {}
