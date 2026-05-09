# use-set

<p align="center">
    <strong>Mathematical-set scaffolding for `RustUse`.</strong><br>
    This crate is intentionally minimal today so the set-focused crate boundary exists before the concrete membership and set-operation APIs are finalized.
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
use-set = "0.0.1"
```

## Foundation

`use-set` is the planned home for mathematical-set helpers, explicit membership operations, and related set-theoretic utilities that should stay narrower than a general collection crate. This initial scaffold keeps the crate boundary publishable while the concrete set surface is designed deliberately.

## When to use directly

Choose `use-set` directly when mathematical set helpers are the only surface you need and you want to keep that concern narrower than the umbrella facade.

## Scope

- The current scaffold establishes the crate boundary, docs surface, example, and test layout.
- Set operations, membership helpers, and related abstractions are intended future additions.
- Boolean logic and general-purpose collection utilities belong in adjacent or external crates.

## Status

`use-set` is a scaffolded public crate in the `RustUse` docs surface. The API remains pre-1.0, and this crate currently exposes only its module structure while the concrete set surface is designed.
