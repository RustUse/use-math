#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Logic utilities for `RustUse`.

pub mod prelude;

/// Returns the material implication `left -> right`.
#[must_use]
pub const fn implication(left: bool, right: bool) -> bool {
    !left || right
}

/// Returns whether two boolean values are logically equivalent.
#[must_use]
pub const fn equivalence(left: bool, right: bool) -> bool {
    left == right
}

/// Returns the exclusive-or of two boolean values.
#[must_use]
pub const fn exclusive_or(left: bool, right: bool) -> bool {
    left ^ right
}

/// Returns the NAND of two boolean values.
#[must_use]
pub const fn nand(left: bool, right: bool) -> bool {
    !(left && right)
}

/// Returns the NOR of two boolean values.
#[must_use]
pub const fn nor(left: bool, right: bool) -> bool {
    !(left || right)
}

/// Returns whether at least two of three inputs are `true`.
#[must_use]
pub const fn majority(first: bool, second: bool, third: bool) -> bool {
    (first && (second || third)) || (second && third)
}

#[cfg(test)]
mod tests {
    use super::{equivalence, exclusive_or, implication, majority, nand, nor};

    #[test]
    fn computes_binary_boolean_helpers() {
        assert!(implication(false, false));
        assert!(implication(false, true));
        assert!(implication(true, true));
        assert!(!implication(true, false));

        assert!(equivalence(true, true));
        assert!(equivalence(false, false));
        assert!(!equivalence(true, false));

        assert!(exclusive_or(true, false));
        assert!(exclusive_or(false, true));
        assert!(!exclusive_or(true, true));
        assert!(!exclusive_or(false, false));

        assert!(!nand(true, true));
        assert!(nand(true, false));

        assert!(nor(false, false));
        assert!(!nor(true, false));
    }

    #[test]
    fn computes_majority_truths() {
        assert!(majority(true, true, false));
        assert!(majority(true, false, true));
        assert!(majority(false, true, true));
        assert!(majority(true, true, true));
        assert!(!majority(true, false, false));
        assert!(!majority(false, false, false));
    }
}
