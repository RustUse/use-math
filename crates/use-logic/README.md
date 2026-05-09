# use-logic

<p align="center">
    <strong>Small boolean algebra helpers for `RustUse`.</strong><br>
    Explicit implication, equivalence, exclusive-or, NAND, NOR, and majority helpers without a larger predicate framework.
</p>

<p align="center">
    <img alt="Rust 1.95.0+" src="https://img.shields.io/badge/Rust-1.95.0%2B-f46623?logo=rust&logoColor=white">
    <img alt="Edition 2024" src="https://img.shields.io/badge/edition-2024-0f766e">
    <img alt="Boolean helpers" src="https://img.shields.io/badge/logic-helpers-1d4ed8">
    <img alt="License MIT or Apache-2.0" src="https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-2a9d8f">
</p>

## Install

```toml
[dependencies]
use-logic = "0.0.1"
```

## Foundation

`use-logic` provides a deliberately small boolean-helper surface. The crate currently exposes explicit helpers for material implication, equivalence, exclusive-or, NAND, NOR, and three-input majority logic. The API stays close to plain `bool` instead of wrapping truth values in extra types that do not add invariants.

<table>
    <tr>
        <td width="33%" valign="top">
            <strong>Binary boolean algebra</strong><br>
            <code>implication</code>, <code>equivalence</code>, and <code>exclusive_or</code> make truth-table intent explicit at the call site.
        </td>
        <td width="33%" valign="top">
            <strong>Derived gates</strong><br>
            <code>nand</code> and <code>nor</code> cover the two common universal gate helpers directly.
        </td>
        <td width="33%" valign="top">
            <strong>Three-input majority</strong><br>
            <code>majority</code> covers simple voting and quorum logic without leaving boolean space.
        </td>
    </tr>
</table>

| Helper group | Primary items | Best fit |
| ------------ | ------------- | -------- |
| Binary relations | `implication`, `equivalence`, `exclusive_or` | Truth-table oriented code that should stay readable without ad hoc boolean expressions |
| Gate helpers | `nand`, `nor` | Boolean-algebra code that wants named derived operations |
| Majority logic | `majority` | Small vote or quorum rules over three predicates |

## When to use directly

Choose `use-logic` directly when predicate and boolean-structure helpers are the only surface you need and you want to keep that concern narrower than the umbrella facade.

| Scenario | Use `use-logic` directly? | Why |
| -------- | ------------------------ | --- |
| You only need named boolean helpers | Yes | The crate stays smaller than the facade and keeps boolean intent explicit |
| You want small majority or truth-table helpers without a broader predicate framework | Yes | The API already covers the common binary and three-input cases directly |
| You also need geometry, combinatorics, or other math domains | Usually no | `use-math` can unify the concrete surfaces behind features |

## Scope

- The current surface is intentionally small and concrete.
- Helpers stay as `const fn` over `bool` rather than introducing wrapper types without added invariants.
- Set operations and algebraic structures belong in adjacent focused crates.

## Examples

### Binary boolean helpers

```rust
use use_logic::{equivalence, exclusive_or, implication};

assert!(implication(false, true));
assert!(!implication(true, false));
assert!(equivalence(true, true));
assert!(exclusive_or(true, false));
```

### Derived gates and majority logic

```rust
use use_logic::{majority, nand, nor};

assert!(!nand(true, true));
assert!(nor(false, false));
assert!(majority(true, true, false));
assert!(!majority(true, false, false));
```

## Status

`use-logic` is a concrete pre-1.0 crate in the `RustUse` docs surface. The API remains intentionally small while adjacent set and algebra crates continue to grow around it.
