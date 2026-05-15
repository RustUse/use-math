# use-prime

<p align="center">
    <strong>Small prime number utilities for <code>RustUse</code>.</strong><br>
    Practical primality checks, prime generation, sieves, and deterministic factorization without cryptographic or arbitrary-precision concerns.
</p>

<p align="center">
    <img alt="Rust 1.95.0+" src="https://img.shields.io/badge/Rust-1.95.0%2B-f46623?logo=rust&logoColor=white">
    <img alt="Edition 2024" src="https://img.shields.io/badge/edition-2024-0f766e">
    <img alt="Prime utilities" src="https://img.shields.io/badge/prime-utilities-1d4ed8">
    <img alt="License MIT or Apache-2.0" src="https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-2a9d8f">
</p>

## Install

```toml
[dependencies]
use-prime = "0.0.5"
```

## What belongs here

`use-prime` owns practical prime-number utilities: primality checks, next and
previous prime search, sieve generation, and deterministic prime factorization
over primitive unsigned integers.

This crate is intended for practical number-theory helpers, not cryptographic
prime generation. It keeps the first release explicit, dependency-free, and
predictable.

## Neighboring crates

| Crate               | Responsibility                                                  |
| ------------------- | --------------------------------------------------------------- |
| `use-prime`         | Primality checks, prime search, sieves, and factorization       |
| `use-modular`       | Modular arithmetic, congruence, inverses, and exponentiation    |
| `use-arithmetic`    | GCD, LCM, divisibility, parity, and arithmetic helpers          |
| `use-integer`       | Integer classification and integer-specific descriptive helpers |
| `use-number`        | Broader number abstractions and shared numeric vocabulary       |
| `use-combinatorics` | Counting helpers that may consume prime utilities               |
| `use-algebra`       | Algebraic structures and law checking over caller-supplied ops  |
| `use-cryptography`  | Future cryptographic protocols and stronger prime requirements  |

`use-prime` intentionally does not define modular arithmetic,
probabilistic primality tests, cryptographic-strength prime generation, or
arbitrary-precision integer support.

## Examples

### Primality and neighboring primes

```rust
use use_prime::{is_prime, next_prime, previous_prime};

assert!(is_prime(13));
assert_eq!(next_prime(13), Some(17));
assert_eq!(previous_prime(13), Some(11));
```

### Prime factors and multiplicities

```rust
use use_prime::{factorization, prime_factors, unique_prime_factors};

assert_eq!(prime_factors(60), vec![2, 2, 3, 5]);
assert_eq!(unique_prime_factors(60), vec![2, 3, 5]);
assert_eq!(factorization(60), vec![(2, 2), (3, 1), (5, 1)]);
```

### Sieve-backed prime generation

```rust
use use_prime::primes_up_to;

assert_eq!(primes_up_to(20), vec![2, 3, 5, 7, 11, 13, 17, 19]);
```

## Status

`use-prime` is a concrete pre-1.0 crate in the `RustUse` math workspace. The
API stays small and deterministic so adjacent number, integer, arithmetic,
modular, combinatorics, algebra, and future cryptography crates can share one
focused prime-utility surface.
