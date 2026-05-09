# use-rational

<p align="center">
    <strong>Small rational-number primitives for direct, explicit Rust code.</strong><br>
    Validated fractions, canonical sign normalization, and checked exact arithmetic without a generic numeric tower.
</p>

<p align="center">
    <img alt="Rust 1.95.0+" src="https://img.shields.io/badge/Rust-1.95.0%2B-f46623?logo=rust&logoColor=white">
    <img alt="Edition 2024" src="https://img.shields.io/badge/edition-2024-0f766e">
    <img alt="Rational primitives" src="https://img.shields.io/badge/rational-primitives-1d4ed8">
    <img alt="Canonical normalization" src="https://img.shields.io/badge/normalization-canonical-c2410c">
    <img alt="License MIT or Apache-2.0" src="https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-2a9d8f">
</p>

<p align="center">
    <a href="#what-this-crate-provides">Surface</a> ·
    <a href="#when-to-use-it-directly">When to use it</a> ·
    <a href="#installation">Installation</a> ·
    <a href="#quick-examples">Examples</a> ·
    <a href="#validation-model">Validation</a> ·
    <a href="#scope">Scope</a>
</p>

`use-rational` provides a deliberately small exact-fraction surface. The crate prefers one canonical representation: denominators are kept positive, reducible fractions are normalized, and arithmetic that cannot stay exact within the current integer representation returns explicit errors instead of silently widening into floating-point behavior.

<table>
    <tr>
        <td width="33%" valign="top">
            <strong>Canonical fractions</strong><br>
            <code>Rational</code> stores reduced numerator and denominator pairs with a positive denominator.
        </td>
        <td width="33%" valign="top">
            <strong>Checked exact arithmetic</strong><br>
            Addition, subtraction, multiplication, division, and reciprocals return <code>RationalError</code> on invalid or overflowing operations.
        </td>
        <td width="33%" valign="top">
            <strong>Explicit conversions</strong><br>
            Integer construction stays exact, and <code>as_f64</code> is an explicit opt-in when approximation is acceptable.
        </td>
    </tr>
</table>

## What this crate provides

| Area | Root exports | Best fit |
| --- | --- | --- |
| Canonical fractions | `Rational`, `RationalError` | Exact reduced fractions without a broader numeric framework |
| Checked arithmetic | `checked_add`, `checked_sub`, `checked_mul`, `checked_div`, `reciprocal` | Exact operations where invalid denominators and overflow stay explicit |
| Integer and floating conversions | `from_integer`, `as_f64`, `is_integer` | Boundaries between exact and approximate numeric workflows |

| If you need to... | Start here |
| --- | --- |
| Build a validated fraction from two integers | `Rational::try_new(...)` |
| Lift an integer into an exact rational | `Rational::from_integer(...)` |
| Keep exact arithmetic explicit | `checked_add(...)`, `checked_mul(...)`, and friends |
| Move to an approximate representation deliberately | `as_f64()` |

## When to use it directly

Choose `use-rational` directly when fractions or exact rational-number support are the only math surface you need, or when you want exact arithmetic to stay separate from the broader facade.

| Scenario | Use `use-rational` directly? | Why |
| --- | --- | --- |
| You need exact normalized fractions | Yes | The crate stays narrow and explicit |
| You want division-by-zero and overflow surfaced as errors | Yes | Arithmetic stays checked instead of implicit |
| You need generic numeric traits across many number kinds | Usually no | Those belong in adjacent focused crates |
| You are happy to lose exactness immediately | Usually no | `use-real` may be the better fit |

## Installation

```toml
[dependencies]
use-rational = "0.0.1"
```

## Quick examples

### Build and normalize exact fractions

```rust
use use_rational::Rational;

let half = Rational::try_new(2, 4)?;
let negative = Rational::try_new(3, -9)?;

assert_eq!(half, Rational::try_new(1, 2)?);
assert_eq!(negative, Rational::try_new(-1, 3)?);
assert_eq!(half.numerator(), 1);
assert_eq!(half.denominator(), 2);
# Ok::<(), use_rational::RationalError>(())
```

### Keep exact arithmetic explicit

```rust
use use_rational::Rational;

let half = Rational::try_new(1, 2)?;
let third = Rational::try_new(1, 3)?;

assert_eq!(half.checked_add(third)?, Rational::try_new(5, 6)?);
assert_eq!(half.checked_div(third)?, Rational::try_new(3, 2)?);
assert!((half.as_f64() - 0.5).abs() < 1.0e-12);
# Ok::<(), use_rational::RationalError>(())
```

## Validation model

Use `try_new` when numerators and denominators may come from user input, files, or network payloads. Use exact helpers like `from_integer`, `zero`, and `one` when the value is already known to be valid.

> [!IMPORTANT]
> This crate does not silently cross into floating-point arithmetic. Exact arithmetic stays exact until a caller explicitly asks for `as_f64()`.

## Scope

- Small exact-fraction APIs are preferred over broad trait-heavy abstractions.
- The initial concrete surface focuses on canonical normalization and checked exact arithmetic.
- Generic numeric hierarchies and symbolic algebra are intentionally out of scope for this first slice.
- Broader integer and algebra abstractions belong in adjacent focused crates.

## Status

`use-rational` is a concrete pre-1.0 crate in the `RustUse` docs surface. The API remains intentionally small while the broader rational-number roadmap is still being designed.
