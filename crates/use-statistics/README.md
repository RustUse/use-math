# use-statistics

<p align="center">
    <strong>Statistical summary and distribution scaffolding for `RustUse`.</strong><br>
    This crate is intentionally minimal today so the statistics-focused crate boundary exists before the concrete summary and distribution APIs are finalized.
</p>

<p align="center">
    <img alt="Rust 1.95.0+" src="https://img.shields.io/badge/Rust-1.95.0%2B-f46623?logo=rust&logoColor=white">
    <img alt="Edition 2024" src="https://img.shields.io/badge/edition-2024-0f766e">
    <img alt="Status scaffold" src="https://img.shields.io/badge/status-scaffold-1d4ed8">
    <img alt="License MIT or Apache-2.0" src="https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-2a9d8f">
</p>

## Install

```toml
[dependencies]
use-statistics = "0.0.1"
```

## Foundation

`use-statistics` is the planned home for summaries such as mean and variance, descriptive statistics helpers, and explicit distribution utilities where they make sense. This initial scaffold keeps the crate boundary publishable while the concrete statistics surface is designed deliberately.

## When to use directly

Choose `use-statistics` directly when statistical summaries or distributions are the only surface you need and you want to keep that concern narrower than the full facade.

## Scope

- The current scaffold establishes the crate boundary, docs surface, example, and test layout.
- Summary statistics, descriptive helpers, and distribution utilities are intended future additions.
- Probability primitives and broader numerical analysis belong in adjacent focused crates.

## Status

`use-statistics` is a scaffolded public crate in the `RustUse` docs surface. The API remains pre-1.0, and this crate currently exposes only its module structure while the concrete statistics surface is designed.