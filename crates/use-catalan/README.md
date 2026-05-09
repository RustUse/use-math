# use-catalan

<p align="center">
    <strong>Composable Catalan number primitives for Rust.</strong><br>
    Exact integer utilities for Catalan numbers, including checked computation,
    sequence generation, membership testing, and an ergonomic iterator.
</p>

<p align="center">
    <img alt="Rust 1.95.0+" src="https://img.shields.io/badge/Rust-1.95.0%2B-f46623?logo=rust&logoColor=white">
    <img alt="Edition 2024" src="https://img.shields.io/badge/edition-2024-0f766e">
    <img alt="Arithmetic u128" src="https://img.shields.io/badge/arithmetic-u128-1d4ed8">
    <img alt="Status pre-release" src="https://img.shields.io/badge/status-pre--release-c2410c">
    <img alt="License MIT or Apache-2.0" src="https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-2a9d8f">
</p>

<p align="center">
    <a href="#what-this-crate-provides">Surface</a> ·
    <a href="#when-to-use-it-directly">When to use it</a> ·
    <a href="#installation">Installation</a> ·
    <a href="#quick-examples">Examples</a> ·
    <a href="#overflow-model">Overflow</a> ·
    <a href="#scope-and-related-crates">Scope</a>
</p>

`use-catalan` provides overflow-aware Catalan number utilities as focused building blocks.
The crate is intentionally small: exact results when they fit in `u128`, and `None` when they do not.
No floating-point formulas, no panics, no external dependencies.

<table>
    <tr>
        <td width="25%" valign="top">
            <strong>Checked computation</strong><br>
            <code>catalan(n)</code> computes C(n) with safe integer arithmetic.
        </td>
        <td width="25%" valign="top">
            <strong>Sequence generation</strong><br>
            <code>catalan_sequence(count)</code> returns the first N Catalan numbers.
        </td>
        <td width="25%" valign="top">
            <strong>Membership testing</strong><br>
            <code>is_catalan_number(v)</code> checks whether a value is a Catalan number.
        </td>
        <td width="25%" valign="top">
            <strong>Iterator</strong><br>
            <code>CatalanSequence</code> yields Catalan numbers and stops cleanly before overflow.
        </td>
    </tr>
</table>

## What this crate provides

| Helper                   | Meaning                                                   | Failure mode              |
| ------------------------ | --------------------------------------------------------- | ------------------------- |
| `catalan(n)`             | Returns C(n) via recurrence with checked arithmetic       | `None` on overflow        |
| `catalan_by_binomial(n)` | Returns C(n) via the closed-form binomial formula         | `None` on overflow        |
| `catalan_recursive(n)`   | Returns C(n) via direct recurrence (same as `catalan`)    | `None` on overflow        |
| `catalan_sequence(k)`    | Returns the first `k` Catalan numbers as a `Vec<u128>`    | `None` on overflow        |
| `is_catalan_number(v)`   | Returns `true` iff `v` is a Catalan number                | Always returns a `bool`   |
| `CatalanSequence`        | Iterator yielding Catalan numbers in order                | Stops before overflow     |

## When to use it directly

Choose `use-catalan` directly when Catalan number primitives are the only math surface you need, or when you want the narrowest possible dependency and API surface.

| Scenario                                              | Use `use-catalan` directly? | Why                                              |
| ----------------------------------------------------- | --------------------------- | ------------------------------------------------ |
| You only need Catalan number computation or checking  | Yes                         | The crate is tiny and purpose-built              |
| You want overflow safety without panics               | Yes                         | All helpers return `Option<u128>` or `bool`      |
| You also need factorial or combination helpers        | Maybe                       | Consider adding `use-combinatorics` alongside    |
| You need arbitrary-precision Catalan numbers          | No                          | This crate intentionally stops at `u128`         |

## Installation

```toml
[dependencies]
use-catalan = "0.0.1"
```

## Quick examples

### Checked Catalan number

```rust
use use_catalan::catalan;

assert_eq!(catalan(0), Some(1));
assert_eq!(catalan(5), Some(42));
assert_eq!(catalan(10), Some(16_796));
assert!(catalan(70).is_none()); // overflows u128
```

### Sequence generation

```rust
use use_catalan::catalan_sequence;

assert_eq!(catalan_sequence(0), Some(vec![]));
assert_eq!(catalan_sequence(6), Some(vec![1, 1, 2, 5, 14, 42]));
```

### Membership test

```rust
use use_catalan::is_catalan_number;

assert!(is_catalan_number(42));
assert!(!is_catalan_number(43));
```

### Iterator

```rust
use use_catalan::CatalanSequence;

let first_six: Vec<u128> = CatalanSequence::new().take(6).collect();
assert_eq!(first_six, vec![1, 1, 2, 5, 14, 42]);
```

## Overflow model

All helpers use checked `u128` integer arithmetic throughout.

| Concern                  | Behavior                                              |
| ------------------------ | ----------------------------------------------------- |
| Result fits in `u128`    | Returns `Some(value)` or pushes value to `Vec`        |
| Result overflows `u128`  | Returns `None` or stops iteration                     |
| Invalid input            | No panics; overflow is the only failure mode          |

The largest Catalan number that fits in `u128` is C(69).
C(70) and beyond return `None`.

## Scope and related crates

`use-catalan` is a direct crate under `use-math`. `RustUse` set repositories stay one layer deep,
so this crate does not contain nested child crates.

| Related crate       | Relationship                                                                              |
| ------------------- | ----------------------------------------------------------------------------------------- |
| `use-combinatorics` | Provides checked factorials and binomial coefficients used by adjacent counting helpers   |
| `use-series`        | Provides series and sequence primitives; Catalan numbers are a natural series instance    |
| `use-fuss-catalan`  | Planned: Fuss-Catalan numbers generalize Catalan numbers to higher-order recurrences      |
| `use-hyper-catalan` | Planned: Hyper-Catalan numbers extend the Fuss-Catalan family further                    |
| `use-geode`         | Planned: Geode logic may use Catalan number primitives from this crate                    |

## Status

`use-catalan` is a pre-1.0 crate in the `RustUse/use-math` workspace.
The API surface is intentionally small and may grow incrementally.
