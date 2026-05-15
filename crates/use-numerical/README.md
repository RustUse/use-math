# use-numerical

<p align="center">
    <strong>Small approximate numerical helpers for <code>RustUse</code>.</strong><br>
    Focused helpers for floating-point comparison, finite differences, numerical integration, and iterative root finding.
</p>

<p align="center">
    <img alt="Rust 1.95.0+" src="https://img.shields.io/badge/Rust-1.95.0%2B-f46623?logo=rust&logoColor=white">
    <img alt="Edition 2024" src="https://img.shields.io/badge/edition-2024-0f766e">
    <img alt="Approximate numerical methods" src="https://img.shields.io/badge/approximate-numerical%20methods-1d4ed8">
    <img alt="License MIT or Apache-2.0" src="https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-2a9d8f">
</p>

## Install

```toml
[dependencies]
use-numerical = "0.0.6"
```

Optional bounded-interval bridge support:

```toml
[dependencies]
use-numerical = { version = "0.0.6", features = ["interval"] }
```

## What belongs here

`use-numerical` owns small, explicit helpers for approximate numerical work
over `f64`. It provides:

- tolerance-based floating-point comparison
- first-derivative finite difference helpers
- deterministic numerical integration rules
- iterative root-finding helpers and error types

This crate is intentionally about approximation-oriented methods only. It does
not add exact algebraic solvers, symbolic differentiation, symbolic
integration, polynomial-specific APIs, matrix solving, or optimization
workflows.

## Neighboring crates

| Crate              | Responsibility                                                      |
| ------------------ | ------------------------------------------------------------------- |
| `use-numerical`    | Approximate numerical methods, tolerances, and iterative solvers    |
| `use-equation`     | Exact small equation helpers such as linear and quadratic formulas  |
| `use-polynomial`   | Polynomial structs, evaluation, derivatives, and direct operations  |
| `use-calculus`     | Calculus concepts and higher-level derivative or integral workflows |
| `use-optimization` | Optimization algorithms and optimization workflows                  |
| `use-real`         | Real-number abstractions and validation policy                      |
| `use-linear`       | Matrix and vector driven linear-system solving                      |

## Examples

### Compare floating-point values with an epsilon

```rust
use use_numerical::approx_eq;

assert!(approx_eq(0.1 + 0.2, 0.3, 1e-12));
```

### Approximate a derivative with a central difference

```rust
use use_numerical::central_difference;

let derivative = central_difference(|x| x * x, 3.0, 1e-6);

assert!((derivative - 6.0).abs() < 1e-5);
```

### Integrate numerically with the trapezoidal rule

```rust
use use_numerical::trapezoidal_rule;

let area = trapezoidal_rule(|x| x * x, 0.0, 1.0, 1_000).unwrap();

assert!((area - 1.0 / 3.0).abs() < 1e-3);
```

### Find a root with bisection

```rust
use use_numerical::{RootOptions, bisection};

let root = bisection(
    |x| x * x - 2.0,
    1.0,
    2.0,
    RootOptions::default(),
)
.unwrap();

assert!((root - 2.0_f64.sqrt()).abs() < 1e-8);
```

### Find a root with Newton-Raphson

```rust
use use_numerical::{RootOptions, newton_raphson};

let root = newton_raphson(
    |x| x * x - 2.0,
    |x| 2.0 * x,
    1.0,
    RootOptions::default(),
)
.unwrap();

assert!((root - 2.0_f64.sqrt()).abs() < 1e-8);
```

## Status

`use-numerical` is a concrete pre-1.0 crate in the `RustUse` math workspace.
It keeps approximation helpers explicit and reusable so neighboring calculus,
equation, polynomial, simulation, control, signal, optimization, and physics
crates can build on one small numerical surface.
