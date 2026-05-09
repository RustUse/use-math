# use-series

<p align="center">
    <strong>Arithmetic, geometric, and power-series scaffolding for `RustUse`.</strong><br>
    This crate is intentionally minimal today so the series-focused crate boundary exists before the concrete summation and progression APIs are finalized.
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
use-series = "0.0.1"
```

## Foundation

`use-series` is the planned home for arithmetic progressions, geometric series, power-series utilities, and explicit convergence-related helpers where they make sense. This initial scaffold keeps the crate boundary publishable while the concrete series surface is designed deliberately.

## When to use directly

Choose `use-series` directly when sequence and series helpers are the only surface you need and you want to keep that concern narrower than the full facade crate.

## Scope

- The current scaffold establishes the crate boundary, docs surface, example, and test layout.
- Progression helpers, partial-sum utilities, and related series APIs are intended future additions.
- Catalan-family sequences and calculus-specific analysis helpers belong in adjacent focused crates.

## Status

`use-series` is a scaffolded public crate in the `RustUse` docs surface. The API remains pre-1.0, and this crate currently exposes only its module structure while the concrete series surface is designed.