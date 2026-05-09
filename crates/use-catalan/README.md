# use-catalan

<p align="center">
    <strong>Catalan-family sequence scaffolding for `RustUse`.</strong><br>
    This crate is intentionally minimal today so the Catalan-focused crate boundary exists before the exact counting and sequence APIs are committed.
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
use-catalan = "0.0.1"
```

## Foundation

`use-catalan` is the planned home for Catalan, Fuss-Catalan, and related counting sequences that deserve a more focused crate than general combinatorics helpers. This initial scaffold keeps the crate boundary publishable while the concrete sequence surface is designed deliberately.

## When to use directly

Choose `use-catalan` directly when Catalan-family sequence helpers are the only surface you need and you want to keep that concern explicit and narrow.

## Scope

- The current scaffold establishes the crate boundary, docs surface, example, and test layout.
- Catalan-family counting helpers and related sequence utilities are intended future additions.
- General combinatorics helpers and broader sequence APIs belong in adjacent focused crates.

## Status

`use-catalan` is a scaffolded public crate in the `RustUse` docs surface. The API remains pre-1.0, and this crate currently exposes only its module structure while the concrete Catalan-family surface is designed.