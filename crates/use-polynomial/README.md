# use-polynomial

<p align="center">
    <strong>Utility-first polynomial primitives for `RustUse`.</strong><br>
    Construct, evaluate, differentiate, and combine polynomials with a small, focused API.
</p>

<p align="center">
    <img alt="Rust 1.95.0+" src="https://img.shields.io/badge/Rust-1.95.0%2B-f46623?logo=rust&logoColor=white">
    <img alt="Edition 2024" src="https://img.shields.io/badge/edition-2024-0f766e">
    <img alt="Arithmetic f64" src="https://img.shields.io/badge/arithmetic-f64-1d4ed8">
    <img alt="License MIT or Apache-2.0" src="https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-2a9d8f">
</p>

<p align="center">
    <a href="#what-this-crate-provides">Surface</a> ·
    <a href="#when-to-use-it-directly">When to use it</a> ·
    <a href="#installation">Installation</a> ·
    <a href="#quick-examples">Examples</a> ·
    <a href="#design-notes">Design</a> ·
    <a href="#scope">Scope</a>
</p>

`use-polynomial` provides simple, composable polynomial primitives for `f64` coefficients. The crate stays intentionally small: explicit construction, Horner's method for evaluation, and standard algebraic operations without heavyweight abstractions.

## What this crate provides

| Item                            | Purpose                                         |
| ------------------------------- | ----------------------------------------------- |
| `Polynomial::new(coeffs)`       | Construct from a coefficient vector             |
| `Polynomial::zero()`            | The zero polynomial                             |
| `Polynomial::constant(c)`       | Constant polynomial `c`                         |
| `Polynomial::linear(a, b)`      | Linear polynomial `a + bx`                      |
| `Polynomial::quadratic(a, b, c)`| Quadratic polynomial `a + bx + cx²`             |
| `.degree()`                     | Degree, or `None` for the zero polynomial        |
| `.coefficients()`               | Coefficients slice in ascending degree order    |
| `.coefficient(n)`               | Single coefficient, `0.0` if out of range       |
| `.evaluate(x)`                  | Evaluate at `x` via Horner's method             |
| `.derivative()`                 | Formal derivative                               |
| `.add(other)`                   | Polynomial addition                             |
| `.sub(other)`                   | Polynomial subtraction                          |
| `.mul(other)`                   | Polynomial multiplication                      |
| `.scale(s)`                     | Scalar multiplication                           |
| `.is_zero()`                    | Test for the zero polynomial                    |

## When to use it directly

Choose `use-polynomial` directly when polynomial arithmetic is the only math surface you need, or when you want the narrowest possible dependency and API surface.

| Scenario                                            | Use `use-polynomial` directly? | Why                                            |
| --------------------------------------------------- | ------------------------------ | ---------------------------------------------- |
| You need to evaluate or differentiate polynomials   | Yes                            | The crate is tiny and purpose-built            |
| You also need geometry or combinatorics support     | Usually no                     | `use-math` is the cleaner integration surface  |
| You need symbolic algebra, roots, or parsing        | No                             | Intentionally out of scope for this crate      |

## Installation

Within the `use-math` workspace, add the path dependency:

```toml
[dependencies]
use-polynomial = { path = "../use-polynomial" }
```

After the first crates.io release:

```toml
[dependencies]
use-polynomial = "0.0.1"
```

Or via the `use-math` facade with the `polynomial` feature:

```toml
[dependencies]
use-math = { version = "0.0.1", features = ["polynomial"] }
```

## Quick examples

### Construct and evaluate a quadratic

```rust
use use_polynomial::Polynomial;

let p = Polynomial::quadratic(1.0, -3.0, 2.0);
// 1 - 3x + 2x²

assert_eq!(p.degree(), Some(2));
assert_eq!(p.evaluate(0.0), 1.0);
assert_eq!(p.evaluate(1.0), 0.0);
assert_eq!(p.evaluate(5.0), 36.0);
```

### Compute a derivative

```rust
use use_polynomial::Polynomial;

// d/dx (1 - 3x + 2x²) = -3 + 4x
let p = Polynomial::quadratic(1.0, -3.0, 2.0);
let d = p.derivative();
assert_eq!(d.coefficients(), &[-3.0, 4.0]);
```

### Add polynomials

```rust
use use_polynomial::Polynomial;

let p = Polynomial::linear(1.0, 2.0);   // 1 + 2x
let q = Polynomial::linear(3.0, 4.0);   // 3 + 4x
let sum = p.add(&q);
assert_eq!(sum.coefficients(), &[4.0, 6.0]);
```

### Multiply polynomials

```rust
use use_polynomial::Polynomial;

// (1 + x)(1 - x) = 1 - x²
let a = Polynomial::linear(1.0, 1.0);
let b = Polynomial::linear(1.0, -1.0);
let product = a.mul(&b);
assert_eq!(product.coefficients(), &[1.0, 0.0, -1.0]);
```

## Design notes

- **Coefficients in ascending degree order.** `coefficients[i]` is the
  coefficient of `x^i`. For example `[1.0, 2.0, 3.0]` represents `1 + 2x + 3x²`.
- **Canonical representation.** Trailing zero coefficients are always stripped
  on construction so that `[1.0, 0.0, 0.0]` and `[1.0]` represent the same
  polynomial.
- **Zero polynomial.** Stored as an empty coefficient vector. `degree()` returns
  `None` and `is_zero()` returns `true`. Evaluates to `0.0` everywhere.
- **Horner's method.** `evaluate` uses Horner's method for numerically stable
  and efficient evaluation.
- **Direct crate, not nested.** `use-polynomial` sits directly under `use-math`
  following the `RustUse` one-layer design rule. It is not nested under a
  `use-numbers` umbrella.
- **v0.1 scope.** The API uses `f64` throughout. Generic numeric trait
  abstractions, symbolic algebra, root solving, parsing, and formal power series
  are intentionally deferred.

## Scope

- `f64` coefficients only. Generic numeric traits are deferred.
- Algebraic operations: `add`, `sub`, `mul`, `scale`, `derivative`.
- Evaluation via Horner's method.
- Symbolic algebra, root solving, GCD, polynomial division, and parsing are
  intentionally out of scope.
- Formal power series and lazy evaluation are intentionally out of scope.

## Future directions

Additional direct crates planned for `use-math`:

- `use-series` — formal power series and generating functions
- `use-primes` — prime sieve, primality testing, factorization
- `use-catalan` — Catalan numbers and tree counting
- `use-hyper-catalan` — hyper-Catalan / Fuss-Catalan sequences
- `use-geode` — geodesic and spherical geometry helpers

## Status

`use-polynomial` is a pre-1.0 direct crate in the `RustUse/use-math` workspace.
The API may evolve incrementally before the first crates.io release.
