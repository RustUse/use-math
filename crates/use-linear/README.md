# use-linear

<p align="center">
    <strong>Small 2D vector and 2×2 matrix helpers for `RustUse`.</strong><br>
    Explicit dot products, matrix products, and small linear-system solves without a larger linear-algebra framework.
</p>

<p align="center">
    <img alt="Rust 1.95.0+" src="https://img.shields.io/badge/Rust-1.95.0%2B-f46623?logo=rust&logoColor=white">
    <img alt="Edition 2024" src="https://img.shields.io/badge/edition-2024-0f766e">
    <img alt="Linear basics" src="https://img.shields.io/badge/linear-basics-1d4ed8">
    <img alt="License MIT or Apache-2.0" src="https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-2a9d8f">
</p>

## Install

```toml
[dependencies]
use-linear = "0.0.1"
```

## Foundation

`use-linear` provides a deliberately small numeric linear-algebra surface for 2D vectors and 2×2 matrices. The first slice includes a `Vector2` type, a `Matrix2` type, dot products, matrix-vector and matrix-matrix products, and explicit solving of small linear systems through `LinearError`. The crate stays close to ordinary `f64` math instead of introducing a large generic tensor framework.

<table>
    <tr>
        <td width="33%" valign="top">
            <strong>Vector primitives</strong><br>
            <code>Vector2</code> covers basic 2D numeric vectors with arithmetic, dot products, and norms.
        </td>
        <td width="33%" valign="top">
            <strong>Matrix primitives</strong><br>
            <code>Matrix2</code> covers determinants, trace, transpose, matrix products, and matrix-vector products.
        </td>
        <td width="33%" valign="top">
            <strong>Small system solves</strong><br>
            <code>solve_2x2</code> and <code>Matrix2::solve</code> keep singular handling explicit with <code>LinearError</code>.
        </td>
    </tr>
</table>

| Helper group              | Primary items              | Best fit                                                                  |
| ------------------------- | -------------------------- | ------------------------------------------------------------------------- |
| Vector operations         | `Vector2`, `dot`           | Small numeric code that needs explicit 2D linear operations               |
| Matrix operations         | `Matrix2`                  | Call sites that need compact 2×2 products, determinants, and trace values |
| Solves and error handling | `solve_2x2`, `LinearError` | Small systems where singular behavior should stay explicit                |

## When to use directly

Choose `use-linear` directly when vector and matrix utilities are the only surface you need and you want to keep that concern narrower than the umbrella facade.

| Scenario                                                | Use `use-linear` directly? | Why                                                                               |
| ------------------------------------------------------- | -------------------------- | --------------------------------------------------------------------------------- |
| You need 2D numeric vectors and 2×2 matrices            | Yes                        | The current surface already covers the common small linear-algebra cases directly |
| You need explicit handling of singular 2×2 solves       | Yes                        | The API keeps failure visible through `LinearError`                               |
| You also need geometry, calculus, or other math domains | Usually no                 | `use-math` can compose the concrete surfaces behind features                      |

## Scope

- The current surface is intentionally small and concrete.
- Helpers stay focused on `f64`-based 2D vectors and 2×2 matrices instead of a broader generic tensor framework.
- Geometry-specific spatial types and broader algebraic traits belong in adjacent focused crates.

## Examples

### Vector and matrix products

```rust
use use_linear::{Matrix2, Vector2, dot};

let vector = Vector2::new(3.0, 4.0);
let other = Vector2::new(-2.0, 1.0);
let matrix = Matrix2::new(2.0, 1.0, 5.0, 3.0);

assert!((dot(vector, other) + 2.0).abs() < 1.0e-12);
assert!((vector.magnitude() - 5.0).abs() < 1.0e-12);
assert_eq!(matrix.mul_vector(Vector2::new(1.0, -1.0)), Vector2::new(1.0, 2.0));
assert_eq!(matrix * Matrix2::identity(), matrix);
```

### Solving a 2×2 system

```rust
use use_linear::{Matrix2, Vector2, solve_2x2};

let matrix = Matrix2::new(2.0, 1.0, 5.0, 3.0);
let rhs = Vector2::new(1.0, 2.0);

assert_eq!(solve_2x2(matrix, rhs)?, Vector2::new(1.0, -1.0));
# Ok::<(), use_linear::LinearError>(())
```

## Status

`use-linear` is a concrete pre-1.0 crate in the `RustUse` docs surface. The API remains intentionally small while adjacent geometry and algebra crates continue to grow around it.
