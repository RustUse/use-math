# use-integer

<p align="center">
    <strong>Integer-oriented helpers and classifications for `RustUse`.</strong><br>
    This crate is intentionally minimal today so the integer-focused crate boundary exists before the concrete helper APIs are committed.
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
use-integer = "0.0.1"
```

## Foundation

`use-integer` is the planned home for integer-specific helpers such as divisibility checks, parity, gcd-style utilities, and explicit integer classifications. This initial scaffold keeps the crate boundary publishable while leaving the precise integer surface for a later pass.

## When to use directly

Choose `use-integer` directly when integer helpers are the only surface you need and you want to keep that concern separate from broader numeric or algebraic APIs.

## Scope

- The current scaffold establishes the crate boundary, docs surface, example, and test layout.
- Integer helper functions and traits are intended future additions.
- Broader numeric abstractions belong in `use-number`, while rational or algebraic APIs belong in their own focused crates.

## Status

`use-integer` is a scaffolded public crate in the `RustUse` docs surface. The API remains pre-1.0, and this crate currently exposes only its module structure while the concrete integer surface is designed.