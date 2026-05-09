# use-real

<p align="center">
    <strong>Real-number utilities and floating-point boundary scaffolding for `RustUse`.</strong><br>
    This crate is intentionally minimal today so the real-number crate boundary exists before its precision and validation APIs are finalized.
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
use-real = "0.0.1"
```

## Foundation

`use-real` is the planned home for real-number helpers, finite-value validation, tolerance-aware comparisons, and related utilities that should stay explicit about floating-point behavior. This initial scaffold keeps the crate boundary publishable while the concrete surface is designed deliberately.

## When to use directly

Choose `use-real` directly when floating-point and real-number utilities are the only surface you need and you want to keep that concern isolated from broader math crates.

## Scope

- The current scaffold establishes the crate boundary, docs surface, example, and test layout.
- Real-number validation, comparison, and utility helpers are intended future additions.
- Geometry-specific tolerance rules and calculus-specific analysis helpers belong in their own focused crates.

## Status

`use-real` is a scaffolded public crate in the `RustUse` docs surface. The API remains pre-1.0, and this crate currently exposes only its module structure while the concrete real-number surface is designed.