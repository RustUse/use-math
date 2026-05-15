# use-equation

<p align="center">
    <strong>Small equation-solving primitives for <code>RustUse</code>.</strong><br>
    Focused helpers for explicit linear equations, quadratic equations, small 2x2 systems, and reusable root-solving traits.
</p>

<p align="center">
    <img alt="Rust 1.95.0+" src="https://img.shields.io/badge/Rust-1.95.0%2B-f46623?logo=rust&logoColor=white">
    <img alt="Edition 2024" src="https://img.shields.io/badge/edition-2024-0f766e">
    <img alt="Equation solving" src="https://img.shields.io/badge/equation-solving-1d4ed8">
    <img alt="License MIT or Apache-2.0" src="https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-2a9d8f">
</p>

## Install

```toml
[dependencies]
use-equation = "0.0.6"
```

Optional low-degree polynomial bridging:

```toml
[dependencies]
use-equation = { version = "0.0.6", features = ["polynomial"] }
```

## What belongs here

`use-equation` owns small, explicit equation-solving primitives over `f64`.
It provides direct helpers for:

- linear equations of the form `ax + b = 0`
- quadratic equations of the form `ax^2 + bx + c = 0`
- small `2x2` linear systems
- reusable solving traits and equation wrapper structs

The crate stays intentionally small. It does not add symbolic algebra,
arbitrary-precision arithmetic, iterative numerical methods, complex roots, or
large linear system solvers in v1.

## Neighboring crates

| Crate              | Responsibility                                                       |
| ------------------ | -------------------------------------------------------------------- |
| `use-equation`     | Direct equation-solving helpers, result enums, and solving traits    |
| `use-polynomial`   | Polynomial representation and direct polynomial operations           |
| `use-linear`       | Higher-level linear algebra workflows and typed matrix-driven solves |
| `use-matrix`       | Matrix primitives                                                    |
| `use-vector`       | Vector primitives                                                    |
| `use-calculus`     | Calculus workflows and derivative or integral helpers                |
| `use-optimization` | Optimization routines that may consume equation models               |
| `use-complex`      | Complex numbers and complex roots                                    |
| `use-numerical`    | Iterative numerical root-finding methods                             |

## Examples

### Solve a linear equation

```rust
use use_equation::{Roots, solve_linear};

assert_eq!(solve_linear(2.0, -4.0), Roots::One(2.0));
```

### Solve a quadratic equation

```rust
use use_equation::{Roots, solve_quadratic};

assert_eq!(solve_quadratic(1.0, -3.0, 2.0), Roots::Two(1.0, 2.0));
```

### Solve a 2x2 system

```rust
use use_equation::solve_2x2;

let solution = solve_2x2(
    2.0, 1.0, 5.0,
    1.0, -1.0, 1.0,
);

assert_eq!(solution, Some((2.0, 1.0)));
```

### Use the `RootSolver` trait

```rust
use use_equation::{LinearEquation, RootSolver, Roots};

let equation = LinearEquation::new(2.0, -4.0);

assert_eq!(equation.solve(), Roots::One(2.0));
```

## Status

`use-equation` is a concrete pre-1.0 crate in the `RustUse` math workspace. It
keeps equation solving explicit and reusable so neighboring polynomial,
numerical, linear algebra, simulation, optimization, and physics crates can
build on a small shared surface.
