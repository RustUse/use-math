use core::fmt;
use std::error::Error;

/// Errors returned by checked Catalan-family helpers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CatalanError {
    /// The Fuss-Catalan order must be at least one.
    ZeroOrder,
    /// The Catalan number no longer fits in `u128`.
    CatalanOverflow(u64),
    /// The Fuss-Catalan number no longer fits in `u128`.
    FussCatalanOverflow {
        /// The requested Fuss-Catalan order.
        order: u64,
        /// The requested sequence index.
        n: u64,
    },
}

impl fmt::Display for CatalanError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroOrder => formatter.write_str("Fuss-Catalan order must be at least 1"),
            Self::CatalanOverflow(n) => {
                write!(formatter, "Catalan number overflowed u128 for n={n}")
            },
            Self::FussCatalanOverflow { order, n } => {
                write!(
                    formatter,
                    "Fuss-Catalan number overflowed u128 for order={order}, n={n}"
                )
            },
        }
    }
}

impl Error for CatalanError {}
