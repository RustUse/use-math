# use-eigen

<p align="center">
    <strong>Small eigenvalue, eigenvector, and eigensystem primitives for <code>RustUse</code>.</strong><br>
    Structural linear-algebra vocabulary for eigenpairs and eigenspaces without claiming to solve arbitrary eigenproblems.
</p>

<p align="center">
    <img alt="Rust 1.95.0+" src="https://img.shields.io/badge/Rust-1.95.0%2B-f46623?logo=rust&logoColor=white">
    <img alt="Edition 2024" src="https://img.shields.io/badge/edition-2024-0f766e">
    <img alt="Eigen primitives" src="https://img.shields.io/badge/eigen-primitives-1d4ed8">
    <img alt="License MIT or Apache-2.0" src="https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-2a9d8f">
</p>

## Install

```toml
[dependencies]
use-eigen = "0.0.6"
```

## Foundation

`use-eigen` provides a deliberately small structural surface for representing
eigenvalues, eigenvectors, eigenpairs, eigensystems, multiplicities, and
eigenspaces. The crate is intended to establish shared vocabulary and explicit
data ownership for `RustUse` code that needs eigen-related concepts without
pulling in a numerical solver stack.

This first version does not compute eigenvalues for arbitrary matrices. It
stores and validates the conceptual pieces you already have, and leaves full
numerical eigensolvers to future specialized crates or later well-tested work.

<table>
    <tr>
        <td width="33%" valign="top">
            <strong>Explicit wrappers</strong><br>
            <code>Eigenvalue</code> and <code>Eigenvector</code> make scalar and coordinate intent visible at the type level.
        </td>
        <td width="33%" valign="top">
            <strong>Structural grouping</strong><br>
            <code>Eigenpair</code>, <code>Eigensystem</code>, and <code>EigenSpace</code> group related eigen data without implying any solver implementation.
        </td>
        <td width="33%" valign="top">
            <strong>Validated construction</strong><br>
            Empty eigenvectors, eigensystems, and eigenspace bases are rejected with explicit <code>EigenError</code> values.
        </td>
    </tr>
</table>

| Helper group         | Primary items                     | Best fit                                                                     |
| -------------------- | --------------------------------- | ---------------------------------------------------------------------------- |
| Scalar wrappers      | `Eigenvalue`                      | Code that wants explicit eigenvalue intent instead of a raw scalar           |
| Vector wrappers      | `Eigenvector`, `EigenError`       | Code that wants validated coordinate storage for eigenvectors                |
| Pair and system data | `Eigenpair`, `Eigensystem`        | Code that already has eigen data and needs a lightweight storage model       |
| Space and counts     | `EigenSpace`, `EigenMultiplicity` | Code that wants eigenspace or multiplicity concepts without solver machinery |

## When to use directly

Choose `use-eigen` directly when you want explicit eigen-related data types but
do not want a broader numerical linear-algebra dependency.

| Scenario                                                 | Use `use-eigen` directly? | Why                                                                           |
| -------------------------------------------------------- | ------------------------- | ----------------------------------------------------------------------------- |
| You need to store an eigenvalue and matching eigenvector | Yes                       | `Eigenpair` keeps the relationship explicit and lightweight                   |
| You need a validated, non-empty eigenvector wrapper      | Yes                       | Construction rejects empty coordinate storage through `EigenError`            |
| You need a full numerical eigensolver                    | No                        | That is intentionally outside this crate's current scope                      |
| You want one facade for multiple math domains            | Usually no                | `use-math` can compose this crate with the rest of the `RustUse` math surface |

## Scope

- `use-eigen` is about small structural types for eigen-related concepts.
- It does not attempt QR, power-iteration, Jacobi, or generalized eigenvalue solvers.
- It does not claim that any stored vector has been verified against a matrix.
- It keeps construction explicit and dependency-free so higher-level crates can compose it later.

## Examples

### Create an eigenvalue

```rust
use use_eigen::Eigenvalue;

let lambda = Eigenvalue::new(3.0);

assert_eq!(*lambda.as_ref(), 3.0);
```

### Create an eigenvector

```rust
use use_eigen::Eigenvector;

let vector = Eigenvector::new(vec![1.0, 0.0])?;

assert_eq!(vector.coordinates(), &[1.0, 0.0]);
# Ok::<(), use_eigen::EigenError>(())
```

### Create an eigenpair

```rust
use use_eigen::{Eigenpair, Eigenvalue, Eigenvector};

let pair = Eigenpair::new(
    Eigenvalue::new(3.0),
    Eigenvector::new(vec![1.0, 0.0])?,
);

assert_eq!(*pair.value().as_ref(), 3.0);
assert_eq!(pair.vector().coordinates(), &[1.0, 0.0]);
# Ok::<(), use_eigen::EigenError>(())
```

### Create an eigensystem

```rust
use use_eigen::{Eigenpair, Eigensystem, Eigenvalue, Eigenvector};

let system = Eigensystem::new(vec![
    Eigenpair::new(
        Eigenvalue::new(2.0),
        Eigenvector::new(vec![1.0, 0.0])?,
    ),
    Eigenpair::new(
        Eigenvalue::new(5.0),
        Eigenvector::new(vec![0.0, 1.0])?,
    ),
])?;

assert_eq!(system.len(), 2);
# Ok::<(), use_eigen::EigenError>(())
```

## Status

`use-eigen` is a concrete pre-1.0 crate in the `RustUse` math workspace. The
current surface is intentionally structural, explicit, and dependency-free.
Future numerical eigensolvers may be added later or delegated to a more
specialized crate once they can be implemented and tested rigorously.
