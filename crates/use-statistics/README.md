# use-statistics

<p align="center">
    <strong>Small statistical summary helpers for `RustUse`.</strong><br>
    Explicit mean, median, variance, and standard-deviation helpers over numeric slices.
</p>

<p align="center">
    <img alt="Rust 1.95.0+" src="https://img.shields.io/badge/Rust-1.95.0%2B-f46623?logo=rust&logoColor=white">
    <img alt="Edition 2024" src="https://img.shields.io/badge/edition-2024-0f766e">
    <img alt="Summary helpers" src="https://img.shields.io/badge/summary-helpers-1d4ed8">
    <img alt="License MIT or Apache-2.0" src="https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-2a9d8f">
</p>

## Install

```toml
[dependencies]
use-statistics = "0.0.1"
```

## Foundation

`use-statistics` provides a deliberately small descriptive-statistics surface over `f64` slices. The crate currently exposes arithmetic mean, median, population variance, sample variance, and matching standard-deviation helpers, along with an explicit `StatisticsError` for empty inputs and too-small samples. The first slice stays simple and direct instead of introducing a full dataframe or distribution framework.

<table>
    <tr>
        <td width="33%" valign="top">
            <strong>Central tendency</strong><br>
            <code>mean</code> and <code>median</code> cover the two common location summaries directly.
        </td>
        <td width="33%" valign="top">
            <strong>Dispersion</strong><br>
            <code>population_variance</code>, <code>sample_variance</code>, <code>population_std_dev</code>, and <code>sample_std_dev</code> keep the estimator choice explicit.
        </td>
        <td width="33%" valign="top">
            <strong>Explicit invalid-input handling</strong><br>
            <code>StatisticsError</code> distinguishes empty-input cases from undersized sample calculations.
        </td>
    </tr>
</table>

| Helper group     | Primary items                                                                    | Best fit                                                                     |
| ---------------- | -------------------------------------------------------------------------------- | ---------------------------------------------------------------------------- |
| Central tendency | `mean`, `median`                                                                 | Small apps and utilities that need direct descriptive summaries              |
| Dispersion       | `population_variance`, `sample_variance`, `population_std_dev`, `sample_std_dev` | Call sites that should remain explicit about population vs. sample summaries |
| Error handling   | `StatisticsError`                                                                | Code that needs predictable handling for empty or undersized inputs          |

## When to use directly

Choose `use-statistics` directly when statistical summaries or distributions are the only surface you need and you want to keep that concern narrower than the full facade.

| Scenario                                                   | Use `use-statistics` directly? | Why                                                                              |
| ---------------------------------------------------------- | ------------------------------ | -------------------------------------------------------------------------------- |
| You need descriptive summaries over `f64` slices           | Yes                            | The current API already covers the common location and dispersion cases directly |
| You want explicit population-vs-sample summary functions   | Yes                            | The function names keep estimator choice visible at the call site                |
| You also need probability, calculus, or other math domains | Usually no                     | `use-math` can compose the concrete surfaces behind features                     |

## Scope

- The current surface is intentionally small and concrete.
- Helpers operate over borrowed `f64` slices instead of introducing dataset wrapper types.
- Probability primitives and broader numerical analysis belong in adjacent focused crates.

## Examples

### Central tendency and population summaries

```rust
use use_statistics::{mean, median, population_std_dev, population_variance};

let values = [2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];

assert!((mean(&values)? - 5.0).abs() < 1.0e-12);
assert!((median(&values)? - 4.5).abs() < 1.0e-12);
assert!((population_variance(&values)? - 4.0).abs() < 1.0e-12);
assert!((population_std_dev(&values)? - 2.0).abs() < 1.0e-12);
# Ok::<(), use_statistics::StatisticsError>(())
```

### Sample summaries

```rust
use use_statistics::{sample_std_dev, sample_variance};

let sample = [1.0, 2.0, 3.0, 4.0];

assert!((sample_variance(&sample)? - 1.666_666_666_666_666_7).abs() < 1.0e-12);
assert!((sample_std_dev(&sample)? - 1.290_994_448_735_805_6).abs() < 1.0e-12);
# Ok::<(), use_statistics::StatisticsError>(())
```

## Status

`use-statistics` is a concrete pre-1.0 crate in the `RustUse` docs surface. The API remains intentionally small while adjacent probability and numeric-analysis crates continue to grow around it.
