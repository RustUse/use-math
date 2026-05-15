# use-vector

<p align="center">
    <strong>Small 2D, 3D, and 4D vector primitives for <code>RustUse</code>.</strong><br>
    Explicit dot products, cross products, normalization, distances, and interpolation without geometry-specific or matrix-specific abstractions.
</p>

<p align="center">
    <img alt="Rust 1.95.0+" src="https://img.shields.io/badge/Rust-1.95.0%2B-f46623?logo=rust&logoColor=white">
    <img alt="Edition 2024" src="https://img.shields.io/badge/edition-2024-0f766e">
    <img alt="Vector primitives" src="https://img.shields.io/badge/vector-primitives-1d4ed8">
    <img alt="License MIT or Apache-2.0" src="https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-2a9d8f">
</p>

## Install

```toml
[dependencies]
use-vector = "0.0.4"
```

## What belongs here

`use-vector` owns plain `f64` vector primitives and reusable vector operations.
The current surface includes `Vector2`, `Vector3`, `Vector4`, dot products,
`Vector3::cross`, norms, normalization, scaling, distances, and linear interpolation.

Scalar division follows normal `f64` semantics. Dividing by zero yields infinities or
`NaN` instead of panicking.

## Neighboring crates

| Crate          | Responsibility                                                          |
| -------------- | ----------------------------------------------------------------------- |
| `use-vector`   | Vector primitives and vector operations                                 |
| `use-matrix`   | Matrix primitives                                                       |
| `use-linear`   | Higher-level linear algebra algorithms and matrix-oriented workflows    |
| `use-geometry` | Points, shapes, angles, geometric relationships, and spatial algorithms |
| `use-physics`  | Physical formulas that use vectors                                      |

`use-vector` intentionally does not add geometry-specific types, matrices, unit-aware
vectors, or domain-specific physics helpers.

## Examples

### Vector magnitude

```rust
use use_vector::Vector2;

let a = Vector2::new(3.0, 4.0);

assert_eq!(a.magnitude(), 5.0);
```

### Dot and cross products

```rust
use use_vector::{Vector2, Vector3};

let a = Vector2::new(1.0, 2.0);
let b = Vector2::new(3.0, 4.0);

assert_eq!(a.dot(b), 11.0);

let x = Vector3::new(1.0, 0.0, 0.0);
let y = Vector3::new(0.0, 1.0, 0.0);

assert_eq!(x.cross(y), Vector3::new(0.0, 0.0, 1.0));
```

### Normalization

```rust
use use_vector::Vector2;

let unit = Vector2::new(3.0, 4.0)
    .normalize()
    .expect("non-zero finite vector should normalize");

assert!((unit.x - 0.6).abs() < 1.0e-12);
assert!((unit.y - 0.8).abs() < 1.0e-12);
```

### Distance

```rust
use use_vector::Vector3;

let start = Vector3::ZERO;
let end = Vector3::new(2.0, 3.0, 6.0);

assert_eq!(start.distance(end), 7.0);
```

## Status

`use-vector` is a concrete pre-1.0 crate in the `RustUse` math workspace. The API stays
small, explicit, and dependency-free so adjacent crates can build on a stable vector core.
