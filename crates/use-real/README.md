# use-real

<p align="center">
    <strong>Small real-number primitives for direct, explicit Rust code.</strong><br>
    Validated finite values, checked closed intervals, and tolerance-aware comparisons without a broader numeric framework.
</p>

<p align="center">
    <img alt="Rust 1.95.0+" src="https://img.shields.io/badge/Rust-1.95.0%2B-f46623?logo=rust&logoColor=white">
    <img alt="Edition 2024" src="https://img.shields.io/badge/edition-2024-0f766e">
    <img alt="Real primitives" src="https://img.shields.io/badge/real-primitives-1d4ed8">
    <img alt="Explicit validation" src="https://img.shields.io/badge/validation-explicit-c2410c">
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

`use-real` provides a deliberately small real-number surface. The crate prefers explicit wrappers over loose `f64` conventions: `Real` keeps finite-value validation attached to the value itself, `RealInterval` encodes closed intervals with checked ordering, and tolerance-aware comparisons require a caller-provided non-negative tolerance.

<table>
    <tr>
        <td width="33%" valign="top">
            <strong>Validated finite values</strong><br>
            <code>Real</code> turns finite-value checks into a named type rather than an ambient convention.
        </td>
        <td width="33%" valign="top">
            <strong>Closed intervals</strong><br>
            <code>RealInterval</code> provides checked bounds, midpoint, width, containment, and clamping.
        </td>
        <td width="33%" valign="top">
            <strong>Explicit tolerance</strong><br>
            <code>approx_eq</code> compares two finite values with a caller-supplied tolerance instead of a hidden epsilon policy.
        </td>
    </tr>
</table>

## What this crate provides

| Area | Root exports | Best fit |
| --- | --- | --- |
| Finite values | `Real`, `RealError` | Explicit finite-value validation instead of raw `f64` assumptions |
| Closed intervals | `RealInterval` | Range checks, clamping, width, and midpoint helpers |
| Tolerance checks | `approx_eq` | Approximate comparisons where tolerance must stay visible |

| If you need to... | Start here |
| --- | --- |
| Validate one finite floating-point value | `Real::try_new(...)` |
| Model a closed interval with checked ordering | `RealInterval::try_new(...)` |
| Clamp or test membership in an interval | `RealInterval` |
| Compare values with an explicit tolerance | `approx_eq(...)` |

## When to use it directly

Choose `use-real` directly when finite-value validation and real-number helpers are the only math surface you need, or when you want floating-point assumptions to stay local instead of spreading through a broader crate.

| Scenario | Use `use-real` directly? | Why |
| --- | --- | --- |
| You need finite-value wrappers and checked intervals | Yes | The crate stays narrow and explicit |
| You want tolerance-aware comparisons with caller-owned policy | Yes | The tolerance is required at the call site |
| You need geometry-specific tolerance rules | Usually no | Those stay better attached to geometry types |
| You need complex analysis or calculus helpers | No | Those belong in adjacent focused crates |

## Installation

```toml
[dependencies]
use-real = "0.0.1"
```

## Quick examples

### Work with finite values and checked intervals

```rust
use use_real::{Real, RealInterval};

let value = Real::try_new(-3.5)?;
let interval = RealInterval::try_new(-2.0, 6.0)?;

assert_eq!(value.abs(), Real::try_new(3.5)?);
assert!(interval.contains(Real::try_new(1.5)?));
assert_eq!(interval.clamp(value.abs()), Real::try_new(3.5)?);
assert!((interval.midpoint().value() - 2.0).abs() < 1.0e-12);
# Ok::<(), use_real::RealError>(())
```

### Keep tolerance choices explicit

```rust
use use_real::{Real, approx_eq};

let left = Real::try_new(1.0)?;
let right = Real::try_new(1.0 + 1.0e-10)?;

assert!(approx_eq(left, right, 1.0e-9)?);
assert!(!approx_eq(left, right, 1.0e-12)?);
# Ok::<(), use_real::RealError>(())
```

## Validation model

Use `try_new` when values, bounds, or tolerances may come from user input, files, or network payloads. Use infallible constructors like `Real::new(...)` and `RealInterval::new(...)` only when finiteness and ordering are already guaranteed by the surrounding code.

> [!IMPORTANT]
> This crate does not define a global epsilon policy. Approximate comparison requires a caller-provided non-negative tolerance every time.

## Scope

- Small real-number APIs are preferred over broad trait-heavy abstractions.
- The initial concrete surface focuses on finite-value validation, closed intervals, and explicit tolerance checks.
- Symbolic algebra, arbitrary precision, and domain-specific tolerance policies are intentionally out of scope for this first slice.
- Geometry-specific and calculus-specific interpretation rules belong in adjacent focused crates.

## Status

`use-real` is a concrete pre-1.0 crate in the `RustUse` docs surface. The API remains intentionally small while the broader real-number roadmap is still being designed.
