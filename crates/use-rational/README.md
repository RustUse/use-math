# use-rational

<p align="center">
    <strong>Rational-number and fraction scaffolding for `RustUse`.</strong><br>
    This crate is intentionally minimal today so the rational crate boundary exists before the exact fraction and normalization APIs are locked in.
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
use-rational = "0.0.1"
```

## Foundation

`use-rational` is the planned home for fractions, normalized ratios, and rational-number utilities that should stay explicit about representation and reduction. This initial scaffold keeps the crate boundary publishable while the concrete rational surface is designed deliberately.

## When to use directly

Choose `use-rational` directly when fraction or rational-number support is the only math surface you need and you want to avoid a broader facade dependency.

## Scope

- The current scaffold establishes the crate boundary, docs surface, example, and test layout.
- Rational-number types, normalization rules, and arithmetic helpers are intended future additions.
- Generic numeric, integer-only, and algebra-wide abstractions belong in adjacent focused crates.

## Status

`use-rational` is a scaffolded public crate in the `RustUse` docs surface. The API remains pre-1.0, and this crate currently exposes only its module structure while the concrete rational surface is designed.