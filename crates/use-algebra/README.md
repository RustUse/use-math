# use-algebra

<p align="center">
    <strong>Algebraic-structure scaffolding for `RustUse`.</strong><br>
    This crate is intentionally minimal today so the algebra-focused crate boundary exists before its traits and structure APIs are finalized.
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
use-algebra = "0.0.1"
```

## Foundation

`use-algebra` is the planned home for basic algebraic structures such as groups, rings, and fields where explicit traits and laws matter more than broad symbolic manipulation. This initial scaffold keeps the crate boundary publishable while the concrete algebra surface is designed deliberately.

## When to use directly

Choose `use-algebra` directly when algebraic structures are the only surface you need and you want to keep that concern narrower than the umbrella facade.

## Scope

- The current scaffold establishes the crate boundary, docs surface, example, and test layout.
- Algebraic structure traits, helper laws, and related utilities are intended future additions.
- Linear algebra and calculus-specific abstractions belong in adjacent focused crates.

## Status

`use-algebra` is a scaffolded public crate in the `RustUse` docs surface. The API remains pre-1.0, and this crate currently exposes only its module structure while the concrete algebra surface is designed.
