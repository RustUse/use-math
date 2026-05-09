# use-logic

<p align="center">
    <strong>Boolean algebra and predicate scaffolding for `RustUse`.</strong><br>
    This crate is intentionally minimal today so the logic-focused crate boundary exists before the concrete boolean and predicate APIs are finalized.
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
use-logic = "0.0.1"
```

## Foundation

`use-logic` is the planned home for boolean algebra, predicate composition, and truth-table-oriented helpers that should stay more explicit than a generic utility crate. This initial scaffold keeps the crate boundary publishable while the concrete logic surface is designed deliberately.

## When to use directly

Choose `use-logic` directly when predicate and boolean-structure helpers are the only surface you need and you want to keep that concern narrower than the umbrella facade.

## Scope

- The current scaffold establishes the crate boundary, docs surface, example, and test layout.
- Boolean algebra helpers, predicates, and truth-table utilities are intended future additions.
- Set operations and algebraic structures belong in adjacent focused crates.

## Status

`use-logic` is a scaffolded public crate in the `RustUse` docs surface. The API remains pre-1.0, and this crate currently exposes only its module structure while the concrete logic surface is designed.
