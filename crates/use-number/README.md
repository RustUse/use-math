# use-number

<p align="center">
    <strong>Numeric building blocks and classification scaffolding for `RustUse`.</strong><br>
    This crate is intentionally minimal today so the workspace can grow around a stable crate boundary before the concrete APIs land.
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
use-number = "0.0.1"
```

## Foundation

`use-number` is the planned home for numeric traits, reusable constants, and number-classification helpers that should stay smaller and more explicit than a broad numeric toolbox. This initial scaffold keeps the crate boundary publishable while leaving room for the concrete API to grow deliberately.

## When to use directly

Choose `use-number` directly when you want a dedicated crate boundary for general numeric utilities instead of depending on the wider `use-math` facade.

## Scope

- The current scaffold only establishes the crate boundary, docs surface, example, and test layout.
- Numeric traits, constants, and classification helpers are intended future additions.
- Domain-specific integer, rational, complex, and statistical APIs belong in their own focused crates.

## Status

`use-number` is a scaffolded public crate in the `RustUse` docs surface. The API remains pre-1.0, and this crate currently exposes only its module structure while the concrete numeric surface is designed.