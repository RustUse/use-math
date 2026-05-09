# use-series

<p align="center">
    <strong>Small arithmetic and geometric progression helpers for `RustUse`.</strong><br>
    Exact checked nth-term and partial-sum helpers for common progression workflows.
</p>

<p align="center">
    <img alt="Rust 1.95.0+" src="https://img.shields.io/badge/Rust-1.95.0%2B-f46623?logo=rust&logoColor=white">
    <img alt="Edition 2024" src="https://img.shields.io/badge/edition-2024-0f766e">
    <img alt="Series progressions" src="https://img.shields.io/badge/series-progressions-1d4ed8">
    <img alt="License MIT or Apache-2.0" src="https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-2a9d8f">
</p>

## Install

```toml
[dependencies]
use-series = "0.0.1"
```

## Foundation

`use-series` provides a deliberately small exact progression surface. The crate currently exposes zero-based arithmetic and geometric nth-term helpers plus checked partial sums over signed `i128` values. Overflow remains explicit through `SeriesError` rather than relying on panics or silently widening into floating-point behavior.

<table>
    <tr>
        <td width="33%" valign="top">
            <strong>Arithmetic progression helpers</strong><br>
            <code>arithmetic_nth_term</code> and <code>arithmetic_sum</code> cover exact linear progressions with signed steps.
        </td>
        <td width="33%" valign="top">
            <strong>Geometric progression helpers</strong><br>
            <code>geometric_nth_term</code> and <code>geometric_sum</code> cover exact signed-ratio progressions.
        </td>
        <td width="33%" valign="top">
            <strong>Explicit overflow</strong><br>
            <code>SeriesError</code> reports the operation that overflowed instead of hiding arithmetic limits.
        </td>
    </tr>
</table>

| Helper group | Primary items | Best fit |
| ------------ | ------------- | -------- |
| Arithmetic progressions | `arithmetic_nth_term`, `arithmetic_sum` | Linear sequences, step-based schedules, and exact partial sums |
| Geometric progressions | `geometric_nth_term`, `geometric_sum` | Exponential growth or decay with exact signed ratios |

## When to use directly

Choose `use-series` directly when sequence and series helpers are the only surface you need and you want to keep that concern narrower than the full facade crate.

| Scenario | Use `use-series` directly? | Why |
| -------- | ------------------------- | --- |
| You only need arithmetic or geometric progression helpers | Yes | The crate stays smaller than the facade and keeps progression arithmetic explicit |
| You want exact signed partial sums without floating-point rounding | Yes | The current surface is checked and stays in `i128` |
| You also need combinatorics, geometry, or other math domains | Usually no | `use-math` can unify the concrete surfaces behind features |

## Scope

- The current surface is intentionally small and concrete.
- Progression helpers stay function-oriented instead of introducing wrapper types without extra invariants.
- Catalan-family sequences and calculus-specific analysis helpers belong in adjacent focused crates.

## Examples

### Arithmetic progressions

```rust
use use_series::{arithmetic_nth_term, arithmetic_sum};

assert_eq!(arithmetic_nth_term(3, 2, 4)?, 11);
assert_eq!(arithmetic_sum(3, 2, 5)?, 35);

# Ok::<(), use_series::SeriesError>(())
```

### Geometric progressions

```rust
use use_series::{geometric_nth_term, geometric_sum};

assert_eq!(geometric_nth_term(2, 3, 4)?, 162);
assert_eq!(geometric_sum(2, 3, 4)?, 80);

# Ok::<(), use_series::SeriesError>(())
```

## Status

`use-series` is a concrete pre-1.0 crate in the `RustUse` docs surface. The API remains intentionally small while broader sequence and analysis crates continue to grow around it.
