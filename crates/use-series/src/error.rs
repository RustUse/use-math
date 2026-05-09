use core::fmt;
use std::error::Error;

/// Errors returned by checked progression helpers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeriesError {
    /// The requested progression calculation overflowed `i128`.
    Overflow {
        /// The operation that overflowed.
        operation: &'static str,
    },
}

impl fmt::Display for SeriesError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Overflow { operation } => {
                write!(
                    formatter,
                    "series operation overflowed i128 while computing {operation}"
                )
            },
        }
    }
}

impl Error for SeriesError {}
