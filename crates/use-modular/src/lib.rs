#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Small modular arithmetic primitives for `RustUse`.

fn checked_modulus(modulus: i64) -> Option<i128> {
    (modulus > 0).then_some(i128::from(modulus))
}

fn normalized_i128(value: i64, modulus: i64) -> Option<i128> {
    let modulus = checked_modulus(modulus)?;
    Some(i128::from(value).rem_euclid(modulus))
}

/// Basic modular arithmetic helpers.
pub mod arithmetic {
    use crate::{checked_modulus, normalized_i128};

    /// Normalizes `value` into the residue class `0..modulus`.
    ///
    /// Returns `None` when `modulus <= 0`.
    #[must_use]
    pub fn mod_normalize(value: i64, modulus: i64) -> Option<i64> {
        i64::try_from(normalized_i128(value, modulus)?).ok()
    }

    /// Computes `(a + b) mod modulus` and returns the normalized residue.
    ///
    /// Returns `None` when `modulus <= 0`.
    #[must_use]
    pub fn mod_add(a: i64, b: i64, modulus: i64) -> Option<i64> {
        let modulus = checked_modulus(modulus)?;
        let sum = normalized_i128(a, modulus as i64)? + normalized_i128(b, modulus as i64)?;
        i64::try_from(sum.rem_euclid(modulus)).ok()
    }

    /// Computes `(a - b) mod modulus` and returns the normalized residue.
    ///
    /// Returns `None` when `modulus <= 0`.
    #[must_use]
    pub fn mod_sub(a: i64, b: i64, modulus: i64) -> Option<i64> {
        let modulus = checked_modulus(modulus)?;
        let difference = normalized_i128(a, modulus as i64)? - normalized_i128(b, modulus as i64)?;
        i64::try_from(difference.rem_euclid(modulus)).ok()
    }

    /// Computes `(a * b) mod modulus` and returns the normalized residue.
    ///
    /// Uses `i128` internally to reduce overflow risk for large `i64` inputs.
    /// Returns `None` when `modulus <= 0`.
    #[must_use]
    pub fn mod_mul(a: i64, b: i64, modulus: i64) -> Option<i64> {
        let modulus = checked_modulus(modulus)?;
        let product = normalized_i128(a, modulus as i64)? * normalized_i128(b, modulus as i64)?;
        i64::try_from(product.rem_euclid(modulus)).ok()
    }
}

/// Modular exponentiation helpers.
pub mod power {
    use crate::{checked_modulus, normalized_i128};

    /// Computes `base.pow(exponent) mod modulus` using exponentiation by squaring.
    ///
    /// Returns the normalized residue in `0..modulus`, or `None` when
    /// `modulus <= 0`.
    #[must_use]
    pub fn mod_pow(base: i64, exponent: u64, modulus: i64) -> Option<i64> {
        let modulus_i128 = checked_modulus(modulus)?;
        let mut result = i128::from(1 % modulus);
        let mut factor = normalized_i128(base, modulus)?;
        let mut power = exponent;

        while power > 0 {
            if power & 1 == 1 {
                result = (result * factor).rem_euclid(modulus_i128);
            }

            factor = (factor * factor).rem_euclid(modulus_i128);
            power >>= 1;
        }

        i64::try_from(result).ok()
    }
}

/// Modular inverse helpers.
pub mod inverse {
    use crate::{checked_modulus, normalized_i128};

    /// Computes the multiplicative inverse of `value` modulo `modulus`.
    ///
    /// Returns `Some(inverse)` only when the inverse exists. The returned
    /// residue is normalized to `0..modulus`. Returns `None` when
    /// `modulus <= 0` or when `value` and `modulus` are not coprime.
    #[must_use]
    pub fn mod_inverse(value: i64, modulus: i64) -> Option<i64> {
        let modulus_i128 = checked_modulus(modulus)?;
        let value_i128 = normalized_i128(value, modulus)?;
        let (gcd, coefficient, _) = extended_gcd(value_i128, modulus_i128);

        (gcd == 1)
            .then(|| coefficient.rem_euclid(modulus_i128))
            .and_then(|inverse| i64::try_from(inverse).ok())
    }

    fn extended_gcd(a: i128, b: i128) -> (i128, i128, i128) {
        let (mut old_r, mut r) = (a, b);
        let (mut old_s, mut s) = (1_i128, 0_i128);
        let (mut old_t, mut t) = (0_i128, 1_i128);

        while r != 0 {
            let quotient = old_r / r;

            (old_r, r) = (r, old_r - quotient * r);
            (old_s, s) = (s, old_s - quotient * s);
            (old_t, t) = (t, old_t - quotient * t);
        }

        (old_r.abs(), old_s, old_t)
    }
}

/// Modular congruence helpers.
pub mod congruence {
    use crate::arithmetic::mod_normalize;

    /// Returns `true` when `a` and `b` are congruent modulo `modulus`.
    ///
    /// Returns `false` when `modulus <= 0`.
    #[must_use]
    pub fn is_congruent(a: i64, b: i64, modulus: i64) -> bool {
        match (mod_normalize(a, modulus), mod_normalize(b, modulus)) {
            (Some(left), Some(right)) => left == right,
            _ => false,
        }
    }
}

pub use arithmetic::{mod_add, mod_mul, mod_normalize, mod_sub};
pub use congruence::is_congruent;
pub use inverse::mod_inverse;
pub use power::mod_pow;

/// A normalized modular residue paired with its positive modulus.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Modular {
    value: i64,
    modulus: i64,
}

impl Modular {
    /// Creates a normalized modular value.
    ///
    /// Returns `None` when `modulus <= 0`.
    #[must_use]
    pub fn new(value: i64, modulus: i64) -> Option<Self> {
        Some(Self {
            value: mod_normalize(value, modulus)?,
            modulus,
        })
    }

    /// Returns the normalized residue in `0..modulus`.
    #[must_use]
    pub const fn value(self) -> i64 {
        self.value
    }

    /// Returns the positive modulus carried by this value.
    #[must_use]
    pub const fn modulus(self) -> i64 {
        self.modulus
    }

    /// Adds two modular values with the same modulus.
    ///
    /// Returns `None` when the moduli differ.
    #[must_use]
    pub fn add(self, other: Self) -> Option<Self> {
        let modulus = self.same_modulus(other)?;
        Self::new(mod_add(self.value, other.value, modulus)?, modulus)
    }

    /// Subtracts two modular values with the same modulus.
    ///
    /// Returns `None` when the moduli differ.
    #[must_use]
    pub fn sub(self, other: Self) -> Option<Self> {
        let modulus = self.same_modulus(other)?;
        Self::new(mod_sub(self.value, other.value, modulus)?, modulus)
    }

    /// Multiplies two modular values with the same modulus.
    ///
    /// Returns `None` when the moduli differ.
    #[must_use]
    pub fn mul(self, other: Self) -> Option<Self> {
        let modulus = self.same_modulus(other)?;
        Self::new(mod_mul(self.value, other.value, modulus)?, modulus)
    }

    /// Raises the modular value to `exponent` using modular exponentiation.
    #[must_use]
    pub fn pow(self, exponent: u64) -> Option<Self> {
        Self::new(mod_pow(self.value, exponent, self.modulus)?, self.modulus)
    }

    /// Computes the multiplicative inverse when one exists.
    #[must_use]
    pub fn inverse(self) -> Option<Self> {
        Self::new(mod_inverse(self.value, self.modulus)?, self.modulus)
    }

    fn same_modulus(self, other: Self) -> Option<i64> {
        (self.modulus == other.modulus).then_some(self.modulus)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        Modular, is_congruent, mod_add, mod_inverse, mod_mul, mod_normalize, mod_pow, mod_sub,
    };

    #[test]
    fn accepts_positive_modulus() {
        assert_eq!(mod_normalize(0, 1), Some(0));
        assert_eq!(mod_normalize(7, 5), Some(2));
    }

    #[test]
    fn rejects_zero_modulus() {
        assert_eq!(mod_normalize(3, 0), None);
        assert_eq!(mod_add(1, 2, 0), None);
        assert_eq!(mod_sub(1, 2, 0), None);
        assert_eq!(mod_mul(1, 2, 0), None);
        assert_eq!(mod_pow(2, 3, 0), None);
        assert_eq!(mod_inverse(3, 0), None);
        assert!(!is_congruent(1, 1, 0));
    }

    #[test]
    fn rejects_negative_modulus() {
        assert_eq!(mod_normalize(3, -5), None);
        assert_eq!(mod_add(1, 2, -5), None);
        assert_eq!(mod_sub(1, 2, -5), None);
        assert_eq!(mod_mul(1, 2, -5), None);
        assert_eq!(mod_pow(2, 3, -5), None);
        assert_eq!(mod_inverse(3, -5), None);
        assert!(!is_congruent(1, 1, -5));
    }

    #[test]
    fn normalizes_positive_values() {
        assert_eq!(mod_normalize(17, 5), Some(2));
    }

    #[test]
    fn normalizes_negative_values() {
        assert_eq!(mod_normalize(-1, 5), Some(4));
        assert_eq!(mod_normalize(-13, 5), Some(2));
    }

    #[test]
    fn adds_residues() {
        assert_eq!(mod_add(4, 3, 5), Some(2));
    }

    #[test]
    fn subtracts_residues() {
        assert_eq!(mod_sub(2, 4, 5), Some(3));
    }

    #[test]
    fn multiplies_residues() {
        assert_eq!(mod_mul(4, 4, 5), Some(1));
    }

    #[test]
    fn computes_modular_powers() {
        assert_eq!(mod_pow(2, 10, 1_000), Some(24));
    }

    #[test]
    fn handles_zero_exponent() {
        assert_eq!(mod_pow(9, 0, 5), Some(1));
        assert_eq!(mod_pow(9, 0, 1), Some(0));
    }

    #[test]
    fn computes_existing_inverse() {
        assert_eq!(mod_inverse(3, 11), Some(4));
    }

    #[test]
    fn reports_missing_inverse() {
        assert_eq!(mod_inverse(2, 4), None);
    }

    #[test]
    fn checks_congruence() {
        assert!(is_congruent(17, 5, 12));
    }

    #[test]
    fn checks_non_congruence() {
        assert!(!is_congruent(17, 6, 12));
    }

    #[test]
    fn multiplies_large_values_with_i128_intermediate() {
        let left = 3_037_000_500_i64;
        let right = 3_037_000_500_i64;
        let modulus = 97_i64;
        let expected = (i128::from(left) * i128::from(right)).rem_euclid(i128::from(modulus));

        assert_eq!(mod_mul(left, right, modulus), i64::try_from(expected).ok());
    }

    #[test]
    fn constructs_and_operates_on_modular_values() {
        let left = Modular::new(-1, 5).expect("valid modular value");
        let right = Modular::new(3, 5).expect("valid modular value");
        let different = Modular::new(1, 7).expect("valid modular value");

        assert_eq!(left.value(), 4);
        assert_eq!(left.modulus(), 5);
        assert_eq!(left.add(right).map(Modular::value), Some(2));
        assert_eq!(left.sub(right).map(Modular::value), Some(1));
        assert_eq!(left.mul(right).map(Modular::value), Some(2));
        assert_eq!(right.pow(4).map(Modular::value), Some(1));
        assert_eq!(right.inverse().map(Modular::value), Some(2));
        assert_eq!(left.add(different), None);
    }
}
