# use-linear

<p align="center">
    <strong>Small linear-system helpers for <code>RustUse</code>.</strong><br>
    Solver-style helpers that build on <code>use-matrix</code> and <code>use-vector</code> instead of redefining primitives.
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
use-linear = "0.0.3"
use-matrix = "0.0.3"
use-vector = "0.0.3"
```

## Foundation

`use-linear` provides a deliberately small linear-algebra surface focused on
algorithms that compose the primitive crates. The current slice includes
`solve_2x2` plus `LinearError` for explicit handling of singular or non-finite
systems. Matrix primitives live in `use-matrix`, and vector primitives live in
`use-vector`.

<table>
    <tr>
        <td width="33%" valign="top">
            <strong>Algorithm layer</strong><br>
            <code>solve_2x2</code> solves small systems without redefining the underlying matrix or vector types.
        </td>
        <td width="33%" valign="top">
            <strong>Explicit errors</strong><br>
            <code>LinearError</code> keeps singular and non-finite solve failures visible at the call site.
        </td>
        <td width="33%" valign="top">
            <strong>Focused composition</strong><br>
            Pair <code>use-linear</code> with <code>use-matrix</code> and <code>use-vector</code> when the primitive ownership boundaries matter.
        </td>
    </tr>
</table>

| Helper group              | Primary items              | Best fit                                                            |
| ------------------------- | -------------------------- | ------------------------------------------------------------------- |
| Solves and error handling | `solve_2x2`, `LinearError` | Small systems where singular behavior should stay explicit          |
| Neighbor primitives       | `use-matrix`, `use-vector` | Call sites that want focused matrix and vector ownership boundaries |

## When to use directly

Choose `use-linear` directly when solve-oriented helpers are the only linear
surface you need and you want primitive ownership to stay in neighboring crates.

| Scenario                                                   | Use `use-linear` directly? | Why                                                                            |
| ---------------------------------------------------------- | -------------------------- | ------------------------------------------------------------------------------ |
| You need explicit handling of singular 2×2 solves          | Yes                        | The API keeps solve failures visible through `LinearError`                     |
| You want matrix and vector types to stay in focused crates | Yes                        | `use-linear` composes `use-matrix` and `use-vector` instead of redefining them |
| You also need geometry, calculus, or other math domains    | Usually no                 | `use-math` can compose the concrete surfaces behind features                   |

## Scope

- `use-matrix` owns matrix primitives and direct matrix operations.
- `use-vector` owns vector primitives and vector operations.
- `use-linear` owns higher-level linear algorithms over those primitives.
- The crate intentionally does not define its own matrix/vector types, geometry transforms, or decomposition frameworks.

## Examples

### Solving a 2x2 system

```rust
use use_linear::solve_2x2;
use use_matrix::Matrix2;
use use_vector::Vector2;

let matrix = Matrix2::new(2.0, 1.0, 5.0, 3.0);
let rhs = Vector2::new(1.0, 2.0);

assert_eq!(solve_2x2(matrix, rhs)?, Vector2::new(1.0, -1.0));
# Ok::<(), use_linear::LinearError>(())
```

### Handling singular systems

```rust
use use_linear::{LinearError, solve_2x2};
use use_matrix::Matrix2;
use use_vector::Vector2;

let matrix = Matrix2::new(1.0, 2.0, 2.0, 4.0);
let rhs = Vector2::new(1.0, 2.0);

assert_eq!(
    solve_2x2(matrix, rhs),
    Err(LinearError::SingularMatrix { determinant: 0.0 })
);
```

## Status

`use-linear` is a concrete pre-1.0 crate in the `RustUse` math workspace. The
API remains intentionally small while `use-matrix` and `use-vector` own the
primitive surface and adjacent crates continue to grow around them.
