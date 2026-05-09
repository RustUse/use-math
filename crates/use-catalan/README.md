# use-catalan

<p align="center">
    <strong>Small Catalan-family counting helpers for `RustUse`.</strong><br>
    Checked Catalan and Fuss-Catalan numbers for exact sequence-counting workflows.
</p>

<p align="center">
    <img alt="Rust 1.95.0+" src="https://img.shields.io/badge/Rust-1.95.0%2B-f46623?logo=rust&logoColor=white">
    <img alt="Edition 2024" src="https://img.shields.io/badge/edition-2024-0f766e">
    <img alt="Catalan counting" src="https://img.shields.io/badge/catalan-counting-1d4ed8">
    <img alt="License MIT or Apache-2.0" src="https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-2a9d8f">
</p>

## Install

```toml
[dependencies]
use-catalan = "0.0.1"
```

## Foundation

`use-catalan` provides a deliberately small exact-counting surface for Catalan-family sequences. The crate currently exposes `catalan(n)` for standard Catalan numbers and `fuss_catalan(order, n)` for the generalized Fuss-Catalan family. Results stay in checked `u128` arithmetic, and overflow or invalid order values surface as explicit `CatalanError` values.

<table>
    <tr>
        <td width="33%" valign="top">
            <strong>Standard Catalan numbers</strong><br>
            <code>catalan(n)</code> covers balanced-parenthesization and binary-tree style counts with exact <code>u128</code> results.
        </td>
        <td width="33%" valign="top">
            <strong>Generalized Fuss-Catalan</strong><br>
            <code>fuss_catalan(order, n)</code> extends the same counting surface to the wider Catalan family.
        </td>
        <td width="33%" valign="top">
            <strong>Explicit failure modes</strong><br>
            <code>CatalanError</code> reports zero-order inputs and overflow instead of silently truncating counts.
        </td>
    </tr>
</table>

| Helper group | Primary items | Best fit |
| ------------ | ------------- | -------- |
| Standard Catalan | `catalan`, `CatalanError` | Exact counts for binary-tree, Dyck-word, and balanced-parenthesization style problems |
| Generalized Catalan | `fuss_catalan` | Wider Catalan-family counts without a broader sequence toolbox |

## When to use directly

Choose `use-catalan` directly when Catalan-family sequence helpers are the only surface you need and you want to keep that concern explicit and narrow.

| Scenario | Use `use-catalan` directly? | Why |
| -------- | -------------------------- | --- |
| You only need exact Catalan counts | Yes | The crate stays smaller than the facade and keeps the counting domain explicit |
| You need generalized Fuss-Catalan counts alongside standard Catalan numbers | Yes | The current surface already covers both without a broader sequence abstraction |
| You also need combinatorics, geometry, or other math domains | Usually no | `use-math` can unify the concrete surfaces behind feature flags |

## Scope

- The current surface is intentionally small and concrete.
- Sequence values use checked `u128` arithmetic and return explicit errors at the boundary.
- General combinatorics helpers and broader sequence APIs belong in adjacent focused crates.

## Examples

### Standard Catalan numbers

```rust
use use_catalan::catalan;

assert_eq!(catalan(4)?, 14);
assert_eq!(catalan(10)?, 16_796);

# Ok::<(), use_catalan::CatalanError>(())
```

### Generalized Fuss-Catalan numbers

```rust
use use_catalan::fuss_catalan;

assert_eq!(fuss_catalan(2, 4)?, 14);
assert_eq!(fuss_catalan(3, 3)?, 12);

# Ok::<(), use_catalan::CatalanError>(())
```

## Status

`use-catalan` is a concrete pre-1.0 crate in the `RustUse` docs surface. The API remains intentionally small while adjacent sequence and combinatorics crates continue to grow around it.
