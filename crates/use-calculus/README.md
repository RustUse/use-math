# use-calculus

<p align="center">
    <strong>Derivative, integral, and limit scaffolding for `RustUse`.</strong><br>
    This crate is intentionally minimal today so the calculus-focused crate boundary exists before the concrete analytic and numerical APIs are finalized.
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
use-calculus = "0.0.1"
```

## Foundation

`use-calculus` is the planned home for limits, derivatives, integrals, and numerical-calculus helpers that should stay explicit about approximation and assumptions. This initial scaffold keeps the crate boundary publishable while the concrete calculus surface is designed deliberately.

## When to use directly

Choose `use-calculus` directly when calculus and numerical analysis helpers are the only surface you need and you want to keep that concern narrower than the full facade.

## Scope

- The current scaffold establishes the crate boundary, docs surface, example, and test layout.
- Derivative, integral, limit, and numerical approximation helpers are intended future additions.
- Broader real-number utilities and statistical analysis belong in adjacent focused crates.

## Status

`use-calculus` is a scaffolded public crate in the `RustUse` docs surface. The API remains pre-1.0, and this crate currently exposes only its module structure while the concrete calculus surface is designed.