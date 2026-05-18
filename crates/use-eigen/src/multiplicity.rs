/// Algebraic and geometric multiplicity for one eigenvalue.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EigenMultiplicity {
    algebraic: usize,
    geometric: usize,
}

impl EigenMultiplicity {
    /// Creates a new multiplicity pair.
    #[must_use]
    pub const fn new(algebraic: usize, geometric: usize) -> Self {
        Self {
            algebraic,
            geometric,
        }
    }

    /// Returns the algebraic multiplicity.
    #[must_use]
    pub const fn algebraic(self) -> usize {
        self.algebraic
    }

    /// Returns the geometric multiplicity.
    #[must_use]
    pub const fn geometric(self) -> usize {
        self.geometric
    }

    /// Returns whether both multiplicities are equal to one.
    #[must_use]
    pub const fn is_simple(self) -> bool {
        self.algebraic == 1 && self.geometric == 1
    }
}

#[cfg(test)]
mod tests {
    use super::EigenMultiplicity;

    #[test]
    fn exposes_algebraic_and_geometric_multiplicity() {
        let multiplicity = EigenMultiplicity::new(3, 2);

        assert_eq!(multiplicity.algebraic(), 3);
        assert_eq!(multiplicity.geometric(), 2);
        assert!(!multiplicity.is_simple());
        assert!(EigenMultiplicity::new(1, 1).is_simple());
    }
}
