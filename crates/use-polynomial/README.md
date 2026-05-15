# use-polynomial

<p align="center">
    <strong>Small polynomial primitives and operations for <code>RustUse</code>.</strong><br>
    Concrete coefficient-based polynomials with evaluation, derivatives, arithmetic, and simple real root helpers for low-degree cases.
</p>

<p align="center">
    <img alt="Rust 1.95.0+" src="https://img.shields.io/badge/Rust-1.95.0%2B-f46623?logo=rust&logoColor=white">
    <img alt="Edition 2024" src="https://img.shields.io/badge/edition-2024-0f766e">
    <img alt="Polynomial primitives" src="https://img.shields.io/badge/polynomial-primitives-1d4ed8">
    <img alt="License MIT or Apache-2.0" src="https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-2a9d8f">
</p>

## Install

```toml
[dependencies]
use-polynomial = "0.0.5"
```

## What belongs here

`use-polynomial` owns concrete polynomial representation and direct polynomial
operations over `f64` coefficients. It provides construction, degree handling,
evaluation, derivatives, integrals, arithmetic, and small real-root helpers for
degree 1 and 2 workflows.

Coefficient ordering is explicit: `coefficients[i]` is the coefficient for
`x^i`, so `3 + 2x + x^2` is stored as `vec![3.0, 2.0, 1.0]`.

The zero polynomial is represented as an empty coefficient vector. That keeps
`degree()` and `leading_coefficient()` explicit by returning `None` for the zero
polynomial.

## Neighboring crates

| Crate              | Responsibility                                             |
| ------------------ | ---------------------------------------------------------- |
| `use-polynomial`   | Polynomial representation and direct polynomial operations |
| `use-equation`     | Equation-solving workflows built on polynomial expressions |
| `use-numerical`    | Iterative numerical methods and higher-degree root finding |
| `use-calculus`     | Calculus concepts and function-based derivative workflows  |
| `use-complex`      | Complex numbers and complex roots                          |
| `use-algebra`      | Algebraic structures and law checking                      |
| `use-real`         | Real-number wrappers and real-specific policy              |
| `use-optimization` | Optimization routines that may consume polynomial models   |

`use-polynomial` intentionally does not define symbolic algebra,
interpolation frameworks, arbitrary-precision arithmetic, or complex roots in
v1.

## Examples

### Construct, inspect, and evaluate

```rust
use use_polynomial::Polynomial;

let p = Polynomial::new(vec![3.0, 2.0, 1.0]);

assert_eq!(p.degree(), Some(2));
assert_eq!(p.evaluate(2.0), 11.0);
```

### Differentiate a polynomial

```rust
use use_polynomial::Polynomial;

let p = Polynomial::new(vec![3.0, 2.0, 1.0]);
let derivative = p.derivative();

assert_eq!(derivative.coefficients(), &[2.0, 2.0]);
```

### Solve a quadratic with real roots

```rust
use use_polynomial::quadratic_roots;

let roots = quadratic_roots(1.0, -3.0, 2.0);

assert_eq!(roots, vec![1.0, 2.0]);
```

## Status

`use-polynomial` is a concrete pre-1.0 crate in the `RustUse` math workspace.
The API stays small and explicit so adjacent equation, numerical, calculus,
algebra, complex, geometry, real-number, and optimization crates can build on a
single focused polynomial surface.
