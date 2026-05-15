# use-geode

<p align="center">
    <strong>Geode-array primitives for `RustUse`: finite type vectors, hyper-Catalan coefficients, polygon subdivision counts, and small exact Geode coefficient calculations.</strong><br>
    Checked `u128` helpers for small exact Geode-array and hyper-Catalan workflows.
</p>

<p align="center">
    <img alt="Rust 1.95.0+" src="https://img.shields.io/badge/Rust-1.95.0%2B-f46623?logo=rust&logoColor=white">
    <img alt="Edition 2024" src="https://img.shields.io/badge/edition-2024-0f766e">
    <img alt="Geode arrays" src="https://img.shields.io/badge/geode-array-1d4ed8">
    <img alt="License MIT or Apache-2.0" src="https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-2a9d8f">
</p>

## Install

```toml
[dependencies]
use-geode = "0.0.1"
```

## Foundation

`use-geode` provides a deliberately small exact-counting surface for finite type vectors `[m2, m3, m4, ...]`, hyper-Catalan coefficients, and Geode coefficients defined by the factorization `S - 1 = (t2 + t3 + t4 + ...) * G`. The crate focuses on primitive `u128` computations, small recursive Geode values, and structural polygon counts derived from a type vector.

This crate provides small, exact, educational and computational primitives inspired by the Geode series. It does not claim to implement the full theory, optimized large-scale Geode computation, or a closed form for the general Geode coefficient.

<table>
    <tr>
        <td width="33%" valign="top">
            <strong>Finite type vectors</strong><br>
            <code>TypeVector</code> models finite vectors such as <code>[m2, m3, m4, ...]</code> with checked index updates and explicit trimming.
        </td>
        <td width="33%" valign="top">
            <strong>Exact counting</strong><br>
            <code>hyper_catalan</code>, <code>geode</code>, and <code>geode_memoized</code> use exact checked integer arithmetic instead of floating-point approximations.
        </td>
        <td width="33%" valign="top">
            <strong>Small-scale helpers</strong><br>
            Polygon edge and vertex counts plus a few axis and diagonal helpers keep the crate useful without turning it into a symbolic series engine.
        </td>
    </tr>
</table>

| Helper group            | Primary items                                                                   | Best fit                                                                              |
| ----------------------- | ------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------- |
| Type vectors            | `TypeVector`, `face_count`                                                      | Modeling small finite hyper-Catalan index vectors                                     |
| Structural counts       | `polygon_edge_count`, `polygon_vertex_count`                                    | Exact subdivision counts derived from a type vector                                   |
| Hyper-Catalan counts    | `hyper_catalan`, `catalan_from_geode_dimension`                                 | Exact coefficients for small hyper-Catalan workflows                                  |
| Geode coefficients      | `geode`, `geode_memoized`, `geode_on_first_axis`, diagonal helpers              | Small recursive Geode-array calculations with checked arithmetic                      |
| Checked integer helpers | `checked_factorial`, `checked_product_factorials`, `exact_divide`, `GeodeError` | Call sites that need explicit overflow and divisibility failures instead of panicking |

## When to use directly

Choose `use-geode` directly when you want a small exact surface for Geode-array primitives and hyper-Catalan vectors without pulling in the wider facade crate.

| Scenario                                                      | Use `use-geode` directly? | Why                                                                               |
| ------------------------------------------------------------- | ------------------------- | --------------------------------------------------------------------------------- |
| You need exact small hyper-Catalan coefficients               | Yes                       | The crate keeps the type-vector and coefficient surface narrow and explicit       |
| You need small recursive Geode coefficients                   | Yes                       | The crate exposes direct checked helpers plus memoization for the same recurrence |
| You need broad math APIs beyond Geode-style counting          | Usually no                | `use-math` can compose this crate with the other focused math crates              |
| You need large-scale symbolic or asymptotic Geode computation | No                        | That is intentionally outside the crate's scope                                   |

## Scope

- This crate is about Geode arrays, hyper-Catalan coefficients, and small exact coefficient calculations.
- It is not geodesy, GIS, geospatial geometry, or coordinate-distance math.
- Calculations stay in checked `u128` arithmetic and return `GeodeError` values on overflow or invalid exact division.
- Recursive Geode helpers are intended for small exact inputs, not large-scale enumeration or symbolic algebra.

## Examples

### Build a type vector and inspect its counts

```rust
use use_geode::{TypeVector, face_count, polygon_edge_count, polygon_vertex_count};

let vector = TypeVector::new(vec![2, 1])?;

assert_eq!(face_count(&vector), 3);
assert_eq!(polygon_edge_count(&vector)?, 8);
assert_eq!(polygon_vertex_count(&vector)?, 6);

# Ok::<(), use_geode::GeodeError>(())
```

### Compute hyper-Catalan and Geode coefficients

```rust
use use_geode::{TypeVector, geode, geode_memoized, hyper_catalan};

let vector = TypeVector::new(vec![0, 1])?;

assert_eq!(hyper_catalan(&vector)?, 1);
assert_eq!(geode(&vector)?, 3);
assert_eq!(geode_memoized(&vector)?, 3);

# Ok::<(), use_geode::GeodeError>(())
```

## Status

`use-geode` is a concrete pre-1.0 crate in the `RustUse` math workspace. The current surface is intentionally small, exact, and focused on finite Geode-array primitives rather than full formal-series machinery.
