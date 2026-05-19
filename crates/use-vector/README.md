# use-vector

<p align="center">
    <strong>Const-generic <code>f64</code> vector primitives for <code>RustUse</code>.</strong><br>
    Explicit dot products, normalization, distances, interpolation, component-wise helpers, and 3D cross products without geometry-specific or matrix-specific abstractions.
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
use-vector = "0.0.7"
```

## What belongs here

`use-vector` owns plain `f64` vector primitives and reusable vector operations.
The core type is `Vector<const N: usize>`, with `Vector2`, `Vector3`, and
`Vector4` aliases for common dimensions. The current surface includes array
construction, indexing, named accessors for 2D/3D/4D aliases, dot products,
`Vector3::cross`, norms, normalization, scaling, distances, linear interpolation,
component-wise mapping/zipping, component min/max/clamp/abs helpers, and finite
or `NaN` checks.

Components are stored privately. Use `vector[index]`, `as_array`, `into_array`,
or named accessors such as `x()`, `y()`, `z()`, and `w()` instead of public
fields.

Scalar division follows normal `f64` semantics. Dividing by zero yields infinities or
`NaN` instead of panicking.

## Neighboring crates

| Crate                         | Responsibility                                                          |
| ----------------------------- | ----------------------------------------------------------------------- |
| `use-vector`                  | Vector primitives and vector operations                                 |
| `use-matrix`                  | Matrix primitives                                                       |
| `use-linear`                  | Higher-level linear algebra algorithms and matrix-oriented workflows    |
| `use-geometry` (sibling repo) | Points, shapes, angles, geometric relationships, and spatial algorithms |
| `use-physics`                 | Physical formulas that use vectors                                      |

`use-vector` intentionally does not add geometry-specific types, matrices, unit-aware
vectors, or domain-specific physics helpers.

## Examples

### Generic vector magnitude

```rust
use use_vector::Vector;

let vector = Vector::<3>::from_array([2.0, 3.0, 6.0]);

assert_eq!(vector.dimension(), 3);
assert_eq!(vector.magnitude(), 7.0);
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

assert!((unit.x() - 0.6).abs() < 1.0e-12);
assert!((unit.y() - 0.8).abs() < 1.0e-12);
```

### Component helpers

```rust
use use_vector::Vector;

let vector = Vector::<3>::from_array([-1.0, 2.0, 5.0]);
let lower = Vector::<3>::ZERO;
let upper = Vector::<3>::from_array([2.0, 3.0, 4.0]);

assert_eq!(vector.abs().into_array(), [1.0, 2.0, 5.0]);
assert_eq!(vector.clamp_components(lower, upper).into_array(), [0.0, 2.0, 4.0]);
assert!(vector.is_finite());
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
