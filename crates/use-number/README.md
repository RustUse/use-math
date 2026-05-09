# use-number

<p align="center">
    <strong>Small numeric constants and floating-point classification helpers for `RustUse`.</strong><br>
    Explicit raw-number helpers that complement `use-real` without pulling in a broader numeric framework.
</p>

<p align="center">
    <img alt="Rust 1.95.0+" src="https://img.shields.io/badge/Rust-1.95.0%2B-f46623?logo=rust&logoColor=white">
    <img alt="Edition 2024" src="https://img.shields.io/badge/edition-2024-0f766e">
    <img alt="Number helpers" src="https://img.shields.io/badge/number-helpers-1d4ed8">
    <img alt="License MIT or Apache-2.0" src="https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-2a9d8f">
</p>

<p align="center">
    <a href="#what-this-crate-provides">Surface</a> ·
    <a href="#when-to-use-it-directly">When to use it</a> ·
    <a href="#installation">Installation</a> ·
    <a href="#quick-examples">Examples</a> ·
    <a href="#scope">Scope</a>
</p>

`use-number` provides a deliberately small raw-number surface. The first concrete slice focuses on two things that are useful across the workspace without overlapping the validated wrappers in `use-real`: reusable numeric constants and explicit classification of plain `f64` values.

<table>
    <tr>
        <td width="33%" valign="top">
            <strong>Floating-point classification</strong><br>
            <code>NumberCategory</code>, <code>NumberSign</code>, <code>classify_number</code>, and <code>classify_number_sign</code> make branching on raw <code>f64</code> shape explicit.
        </td>
        <td width="33%" valign="top">
            <strong>Finite checks</strong><br>
            <code>is_finite_number</code> keeps lightweight guard clauses available when introducing a full wrapper type would be too heavy.
        </td>
        <td width="33%" valign="top">
            <strong>Reusable constants</strong><br>
            <code>GOLDEN_RATIO</code> and <code>SQRT_3</code> provide small workspace-owned constants alongside their <code>f32</code> variants.
        </td>
    </tr>
</table>

## What this crate provides

| Area | Root exports | Best fit |
| --- | --- | --- |
| Number classification | `NumberCategory`, `NumberSign`, `classify_number`, `classify_number_sign` | Raw `f64` branching without ad hoc `match` logic on `FpCategory` |
| Lightweight finite checks | `is_finite_number` | Guard clauses before opting into a heavier validated wrapper |
| Constants | `GOLDEN_RATIO`, `GOLDEN_RATIO_F32`, `SQRT_3`, `SQRT_3_F32` | Small numeric formulas and geometric ratios |

| If you need to... | Start here |
| --- | --- |
| Branch on whether a raw `f64` is normal, zero, subnormal, infinite, or NaN | `classify_number(...)` |
| Separate negative, zero, and positive raw values while keeping NaN explicit | `classify_number_sign(...)` |
| Add a fast finiteness guard without introducing a wrapper type | `is_finite_number(...)` |
| Reuse the golden ratio or `sqrt(3)` in small formulas | `GOLDEN_RATIO` or `SQRT_3` |

## When to use it directly

Choose `use-number` directly when raw-number helpers are the only surface you need and you want to stay narrower than `use-math` or `use-real`.

| Scenario | Use `use-number` directly? | Why |
| --- | --- | --- |
| You need raw floating-point classification helpers | Yes | The crate stays small and explicit |
| You need a couple of reusable numeric constants in one place | Yes | The constants are available without broader wrappers |
| You want validated finite values or checked intervals | Usually no | `use-real` keeps those invariants attached to types |
| You need domain-specific integer, rational, probability, or geometry APIs | No | Those belong in adjacent focused crates |

## Installation

```toml
[dependencies]
use-number = "0.0.1"
```

## Quick examples

### Classify raw floating-point values

```rust
use use_number::{NumberCategory, NumberSign, classify_number, classify_number_sign, is_finite_number};

assert_eq!(classify_number(f64::NAN), NumberCategory::Nan);
assert_eq!(classify_number(f64::from_bits(1)), NumberCategory::Subnormal);
assert_eq!(classify_number(0.0), NumberCategory::Zero);
assert_eq!(classify_number_sign(-12.5), Some(NumberSign::Negative));
assert_eq!(classify_number_sign(f64::NAN), None);
assert!(is_finite_number(3.5));
assert!(!is_finite_number(f64::INFINITY));
```

### Reuse common numeric constants

```rust
use use_number::{GOLDEN_RATIO, SQRT_3};

assert!(((GOLDEN_RATIO * GOLDEN_RATIO) - (GOLDEN_RATIO + 1.0)).abs() < 1.0e-12);
assert!(((SQRT_3 * SQRT_3) - 3.0).abs() < 1.0e-12);
```

## Scope

- The initial surface stays concrete and small.
- `use-number` focuses on raw-number helpers that do not need a validated wrapper type.
- Validated finite values and intervals still belong in `use-real`.
- Broader numeric traits and heavier abstractions remain out of scope for this first slice.

## Status

`use-number` is a concrete pre-1.0 crate in the `RustUse` docs surface. The API remains intentionally small while adjacent numeric crates continue to grow around it.
