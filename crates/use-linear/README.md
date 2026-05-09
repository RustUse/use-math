# use-linear

<p align="center">
    <strong>Vector and matrix scaffolding for `RustUse` linear algebra.</strong><br>
    This crate is intentionally minimal today so the linear-algebra crate boundary exists before the concrete vector, matrix, and transform APIs are finalized.
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
use-linear = "0.0.1"
```

## Foundation

`use-linear` is the planned home for vectors, matrices, and related linear-algebra helpers that deserve a more focused crate than the general facade. This initial scaffold keeps the crate boundary publishable while the concrete linear surface is designed deliberately.

## When to use directly

Choose `use-linear` directly when vector and matrix utilities are the only surface you need and you want to keep that concern narrower than the umbrella facade.

## Scope

- The current scaffold establishes the crate boundary, docs surface, example, and test layout.
- Vector, matrix, and transform helpers are intended future additions.
- Geometry-specific spatial types and broader algebraic traits belong in adjacent focused crates.

## Status

`use-linear` is a scaffolded public crate in the `RustUse` docs surface. The API remains pre-1.0, and this crate currently exposes only its module structure while the concrete linear surface is designed.
