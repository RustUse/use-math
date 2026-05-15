# use-modular

<p align="center">
    <strong>Small modular arithmetic primitives for <code>RustUse</code>.</strong><br>
    Normalized modular residues, congruence checks, inverses, and exponentiation without number-taxonomy or cryptographic protocol concerns.
</p>

<p align="center">
    <img alt="Rust 1.95.0+" src="https://img.shields.io/badge/Rust-1.95.0%2B-f46623?logo=rust&logoColor=white">
    <img alt="Edition 2024" src="https://img.shields.io/badge/edition-2024-0f766e">
    <img alt="Modular arithmetic" src="https://img.shields.io/badge/modular-arithmetic-1d4ed8">
    <img alt="License MIT or Apache-2.0" src="https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-2a9d8f">
</p>

## Install

```toml
[dependencies]
use-modular = "0.0.5"
```

## What belongs here

`use-modular` owns small modular arithmetic operations over primitive signed
integers. It keeps normalization, addition, subtraction, multiplication,
exponentiation, inverses, and congruence checks explicit while always returning
residues normalized to `0..modulus`.

Invalid moduli are handled conservatively: functions return `None` when
`modulus <= 0`, and congruence checks return `false`.

## Neighboring crates

| Crate               | Responsibility                                                      |
| ------------------- | ------------------------------------------------------------------- |
| `use-modular`       | Modular arithmetic operations and congruence checks                 |
| `use-integer`       | Integer classification and integer-specific descriptive helpers     |
| `use-number`        | Broader number abstractions and shared numeric vocabulary           |
| `use-prime`         | Primality, sieves, prime generation, and factorization              |
| `use-algebra`       | Algebraic structures and law checking over caller-supplied ops      |
| `use-combinatorics` | Counting helpers that may consume modular arithmetic                |
| `use-cryptography`  | Future protocol and cryptographic constructions built on modularity |

`use-modular` intentionally does not define primality testing,
factorization, ring traits, cryptographic protocols, or large arbitrary-
precision integer types.

## Examples

### Normalize and combine residues

```rust
use use_modular::{mod_add, mod_mul, mod_sub};

assert_eq!(mod_add(4, 3, 5), Some(2));
assert_eq!(mod_sub(2, 4, 5), Some(3));
assert_eq!(mod_mul(4, 4, 5), Some(1));
```

### Exponentiation and inverse

```rust
use use_modular::{mod_inverse, mod_pow};

assert_eq!(mod_pow(2, 10, 1_000), Some(24));
assert_eq!(mod_inverse(3, 11), Some(4));
```

### Congruence

```rust
use use_modular::is_congruent;

assert!(is_congruent(17, 5, 12));
assert!(!is_congruent(17, 6, 12));
```

## Status

`use-modular` is a concrete pre-1.0 crate in the `RustUse` math workspace. The
API stays explicit and dependency-free so adjacent integer, number, algebra,
combinatorics, and future cryptography crates can share one small modular
arithmetic vocabulary.
