#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Small prime number utilities for `RustUse`.

/// Primality checks and neighboring prime search.
pub mod primality {
    /// Returns `true` when `n` is prime.
    ///
    /// `0` and `1` are not prime. `2` and `3` are prime. Even values greater
    /// than `2` are not prime.
    #[must_use]
    pub fn is_prime(n: u64) -> bool {
        match n {
            0 | 1 => false,
            2 | 3 => true,
            _ if n % 2 == 0 => false,
            _ => {
                let mut divisor = 3_u64;

                while divisor <= n / divisor {
                    if n % divisor == 0 {
                        return false;
                    }

                    divisor += 2;
                }

                true
            },
        }
    }

    /// Returns `true` when `n` is composite.
    ///
    /// Composite values are integers greater than `1` that are not prime.
    #[must_use]
    pub fn is_composite(n: u64) -> bool {
        n > 1 && !is_prime(n)
    }

    /// Returns the smallest prime strictly greater than `n`.
    ///
    /// Returns `None` when no larger `u64` value exists.
    #[must_use]
    pub fn next_prime(n: u64) -> Option<u64> {
        if n < 2 {
            return Some(2);
        }

        let mut candidate = n.checked_add(1)?;

        if candidate == 2 {
            return Some(2);
        }

        if candidate % 2 == 0 {
            candidate = candidate.checked_add(1)?;
        }

        loop {
            if is_prime(candidate) {
                return Some(candidate);
            }

            candidate = candidate.checked_add(2)?;
        }
    }

    /// Returns the largest prime strictly less than `n`.
    ///
    /// Returns `None` for `0`, `1`, and `2`.
    #[must_use]
    pub fn previous_prime(n: u64) -> Option<u64> {
        if n <= 2 {
            return None;
        }

        if n == 3 {
            return Some(2);
        }

        let mut candidate = n - 1;

        if candidate % 2 == 0 {
            candidate -= 1;
        }

        loop {
            if is_prime(candidate) {
                return Some(candidate);
            }

            if candidate <= 3 {
                return Some(2);
            }

            candidate -= 2;
        }
    }
}

/// Prime factorization helpers.
pub mod factorization {
    /// Returns the prime factorization of `n` as `(prime, exponent)` pairs.
    ///
    /// `0` and `1` return an empty vector in this crate. Results are sorted in
    /// ascending prime order.
    #[must_use]
    pub fn factorization(n: u64) -> Vec<(u64, u32)> {
        if n < 2 {
            return Vec::new();
        }

        let mut remaining = n;
        let mut factors = Vec::new();
        let mut exponent = 0_u32;

        while remaining % 2 == 0 {
            remaining /= 2;
            exponent += 1;
        }

        if exponent > 0 {
            factors.push((2, exponent));
        }

        let mut divisor = 3_u64;
        while divisor <= remaining / divisor {
            let mut local_exponent = 0_u32;

            while remaining % divisor == 0 {
                remaining /= divisor;
                local_exponent += 1;
            }

            if local_exponent > 0 {
                factors.push((divisor, local_exponent));
            }

            divisor += 2;
        }

        if remaining > 1 {
            factors.push((remaining, 1));
        }

        factors
    }

    /// Returns the prime factors of `n`, including repeated multiplicities.
    ///
    /// `0` and `1` return an empty vector in this crate.
    #[must_use]
    pub fn prime_factors(n: u64) -> Vec<u64> {
        factorization(n)
            .into_iter()
            .flat_map(|(prime, exponent)| core::iter::repeat_n(prime, exponent as usize))
            .collect()
    }

    /// Returns the unique prime factors of `n` in ascending order.
    ///
    /// `0` and `1` return an empty vector in this crate.
    #[must_use]
    pub fn unique_prime_factors(n: u64) -> Vec<u64> {
        factorization(n)
            .into_iter()
            .map(|(prime, _)| prime)
            .collect()
    }
}

/// Prime sieves and sieve-backed prime generation.
pub mod sieve {
    /// Returns a boolean sieve up to and including `limit`.
    ///
    /// The returned vector has length `limit + 1`, and index `i` is `true`
    /// exactly when `i` is prime.
    #[must_use]
    pub fn sieve(limit: usize) -> Vec<bool> {
        let mut flags = vec![true; limit.saturating_add(1)];

        if let Some(first) = flags.get_mut(0) {
            *first = false;
        }

        if let Some(second) = flags.get_mut(1) {
            *second = false;
        }

        let mut candidate = 2_usize;
        while candidate <= limit / candidate {
            if flags[candidate] {
                let mut multiple = candidate * candidate;
                while multiple <= limit {
                    flags[multiple] = false;
                    multiple += candidate;
                }
            }

            candidate += 1;
        }

        flags
    }

    /// Returns every prime less than or equal to `limit`.
    #[must_use]
    pub fn primes_up_to(limit: usize) -> Vec<usize> {
        sieve(limit)
            .into_iter()
            .enumerate()
            .filter_map(|(value, is_prime)| is_prime.then_some(value))
            .collect()
    }
}

pub use factorization::{factorization, prime_factors, unique_prime_factors};
pub use primality::{is_composite, is_prime, next_prime, previous_prime};
pub use sieve::{primes_up_to, sieve};

#[cfg(test)]
mod tests {
    use super::{
        factorization, is_composite, is_prime, next_prime, previous_prime, prime_factors,
        primes_up_to, sieve, unique_prime_factors,
    };

    #[test]
    fn classifies_zero_one_two_and_three() {
        assert!(!is_prime(0));
        assert!(!is_prime(1));
        assert!(is_prime(2));
        assert!(is_prime(3));
    }

    #[test]
    fn rejects_small_composites() {
        assert!(!is_prime(4));
        assert!(is_prime(5));
        assert!(!is_prime(9));
        assert!(is_prime(11));
    }

    #[test]
    fn rejects_even_composites_greater_than_two() {
        assert!(!is_prime(6));
        assert!(!is_prime(42));
        assert!(!is_prime(10_000));
    }

    #[test]
    fn rejects_square_composites() {
        assert!(!is_prime(49));
        assert!(!is_prime(121));
    }

    #[test]
    fn accepts_larger_primes() {
        assert!(is_prime(7_919));
        assert!(is_prime(10_007));
    }

    #[test]
    fn rejects_larger_composites() {
        assert!(!is_prime(7_920));
        assert!(!is_prime(10_001));
    }

    #[test]
    fn classifies_composite_values() {
        assert!(!is_composite(0));
        assert!(!is_composite(1));
        assert!(!is_composite(2));
        assert!(is_composite(4));
        assert!(is_composite(9));
    }

    #[test]
    fn finds_next_prime() {
        assert_eq!(next_prime(0), Some(2));
        assert_eq!(next_prime(1), Some(2));
        assert_eq!(next_prime(2), Some(3));
        assert_eq!(next_prime(14), Some(17));
    }

    #[test]
    fn finds_previous_prime() {
        assert_eq!(previous_prime(0), None);
        assert_eq!(previous_prime(1), None);
        assert_eq!(previous_prime(2), None);
        assert_eq!(previous_prime(3), Some(2));
        assert_eq!(previous_prime(13), Some(11));
    }

    #[test]
    fn computes_prime_factors() {
        assert_eq!(prime_factors(0), Vec::<u64>::new());
        assert_eq!(prime_factors(1), Vec::<u64>::new());
        assert_eq!(prime_factors(2), vec![2]);
        assert_eq!(prime_factors(12), vec![2, 2, 3]);
        assert_eq!(prime_factors(60), vec![2, 2, 3, 5]);
    }

    #[test]
    fn computes_unique_prime_factors() {
        assert_eq!(unique_prime_factors(0), Vec::<u64>::new());
        assert_eq!(unique_prime_factors(1), Vec::<u64>::new());
        assert_eq!(unique_prime_factors(2), vec![2]);
        assert_eq!(unique_prime_factors(12), vec![2, 3]);
    }

    #[test]
    fn computes_factorization_pairs() {
        assert_eq!(factorization(0), Vec::<(u64, u32)>::new());
        assert_eq!(factorization(1), Vec::<(u64, u32)>::new());
        assert_eq!(factorization(2), vec![(2, 1)]);
        assert_eq!(factorization(12), vec![(2, 2), (3, 1)]);
        assert_eq!(factorization(60), vec![(2, 2), (3, 1), (5, 1)]);
    }

    #[test]
    fn builds_sieve_flags() {
        assert_eq!(sieve(0), vec![false]);
        assert_eq!(sieve(1), vec![false, false]);
        assert_eq!(sieve(2), vec![false, false, true]);

        let flags = sieve(10);
        assert_eq!(
            flags,
            vec![
                false, false, true, true, false, true, false, true, false, false, false
            ]
        );
    }

    #[test]
    fn lists_primes_up_to_limit() {
        assert_eq!(primes_up_to(0), Vec::<usize>::new());
        assert_eq!(primes_up_to(1), Vec::<usize>::new());
        assert_eq!(primes_up_to(2), vec![2]);
        assert_eq!(primes_up_to(10), vec![2, 3, 5, 7]);
        assert_eq!(primes_up_to(20), vec![2, 3, 5, 7, 11, 13, 17, 19]);
    }
}
