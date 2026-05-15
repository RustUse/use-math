# use-matrix

<p align="center">
    <strong>Small 2D, 3D, and 4D matrix primitives for <code>RustUse</code>.</strong><br>
    Explicit matrix construction, transpose, determinants, inverses, and products without geometry-specific transforms or higher-level linear algebra algorithms.
</p>

<p align="center">
    <img alt="Rust 1.95.0+" src="https://img.shields.io/badge/Rust-1.95.0%2B-f46623?logo=rust&logoColor=white">
    <img alt="Edition 2024" src="https://img.shields.io/badge/edition-2024-0f766e">
    <img alt="Matrix primitives" src="https://img.shields.io/badge/matrix-primitives-1d4ed8">
    <img alt="License MIT or Apache-2.0" src="https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-2a9d8f">
</p>

## Install

```toml
[dependencies]
use-matrix = "0.0.3"
```

## What belongs here

`use-matrix` owns plain `f64` matrix primitives and direct matrix operations.
The current surface includes `Matrix2`, `Matrix3`, `Matrix4`, matrix-matrix
multiplication, matrix-vector multiplication, transpose, trace, determinants,
and inverses for 2x2 and 3x3 matrices.

Scalar division follows normal `f64` semantics. Dividing by zero yields
infinities or `NaN` instead of panicking.

## Neighboring crates

| Crate          | Responsibility                                                    |
| -------------- | ----------------------------------------------------------------- |
| `use-matrix`   | Matrix primitives and direct matrix operations                    |
| `use-vector`   | Vector primitives and vector operations                           |
| `use-linear`   | Higher-level linear algebra algorithms and solver-style workflows |
| `use-geometry` | Geometric transforms, shapes, points, and spatial algorithms      |
| `use-physics`  | Physical formulas that use matrices                               |

`use-matrix` intentionally does not add geometry-specific transforms,
quaternions, unit-aware matrices, or broader decomposition and solver APIs.

## Examples

### 2x2 determinant

```rust
use use_matrix::Matrix2;

let matrix = Matrix2::new(
    1.0, 2.0,
    3.0, 4.0,
);

assert_eq!(matrix.determinant(), -2.0);
```

### 2x2 inverse

```rust
use use_matrix::Matrix2;

let matrix = Matrix2::new(
    4.0, 7.0,
    2.0, 6.0,
);

let inverse = matrix.inverse().unwrap();
let identity = matrix * inverse;

assert!((identity.m00 - 1.0).abs() < 1e-10);
assert!((identity.m11 - 1.0).abs() < 1e-10);
```

### 3x3 identity and multiplication

```rust
use use_matrix::Matrix3;

let matrix = Matrix3::new(
    1.0, 2.0, 3.0,
    0.0, 1.0, 4.0,
    5.0, 6.0, 0.0,
);

assert_eq!(Matrix3::IDENTITY * matrix, matrix);
```

### 4x4 transpose

```rust
use use_matrix::Matrix4;

let matrix = Matrix4::from_rows([
    [1.0, 2.0, 3.0, 4.0],
    [5.0, 6.0, 7.0, 8.0],
    [9.0, 10.0, 11.0, 12.0],
    [13.0, 14.0, 15.0, 16.0],
]);

let transposed = matrix.transpose();

assert_eq!(transposed.m01, 5.0);
assert_eq!(transposed.m10, 2.0);
```

## Status

`use-matrix` is a concrete pre-1.0 crate in the `RustUse` math workspace. The
API stays small, explicit, and dependency-light so adjacent crates can build on
it without pulling in geometry-specific or solver-specific abstractions.
