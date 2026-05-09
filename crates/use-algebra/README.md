# use-algebra

<p align="center">
    <strong>Small finite algebra law helpers for `RustUse`.</strong><br>
    Explicit group-like and ring-like law checks over caller-supplied finite sample sets.
</p>

<p align="center">
    <img alt="Rust 1.95.0+" src="https://img.shields.io/badge/Rust-1.95.0%2B-f46623?logo=rust&logoColor=white">
    <img alt="Edition 2024" src="https://img.shields.io/badge/edition-2024-0f766e">
    <img alt="Algebra laws" src="https://img.shields.io/badge/algebra-laws-1d4ed8">
    <img alt="License MIT or Apache-2.0" src="https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-2a9d8f">
</p>

<p align="center">
    <a href="#what-this-crate-provides">Surface</a> ·
    <a href="#when-to-use-it-directly">When to use it</a> ·
    <a href="#installation">Installation</a> ·
    <a href="#quick-examples">Examples</a> ·
    <a href="#scope">Scope</a>
</p>

`use-algebra` provides a deliberately small finite-law surface. The first concrete slice focuses on explicit checks over a caller-supplied sample set and caller-supplied operations: closure, associativity, commutativity, identity discovery, inverse checks, and combined predicates for monoids, groups, abelian groups, distributive laws, and rings.

<table>
    <tr>
        <td width="33%" valign="top">
            <strong>Binary-law predicates</strong><br>
            <code>is_closed_under</code>, <code>is_associative</code>, and <code>is_commutative</code> keep algebraic-law checks explicit over finite sample sets.
        </td>
        <td width="33%" valign="top">
            <strong>Identity and inverse helpers</strong><br>
            <code>identity_element</code> and <code>has_inverses</code> cover the common building blocks for monoids and groups.
        </td>
        <td width="33%" valign="top">
            <strong>Structure checks</strong><br>
            <code>is_monoid</code>, <code>is_group</code>, <code>is_abelian_group</code>, <code>is_distributive_over</code>, and <code>is_ring</code> provide compact finite checks for common algebraic structures.
        </td>
    </tr>
</table>

## What this crate provides

| Area | Root exports | Best fit |
| --- | --- | --- |
| Basic law checks | `is_closed_under`, `is_associative`, `is_commutative` | Finite structure checks over explicit sample sets and operations |
| Identity and inverse checks | `identity_element`, `has_inverses` | Building monoid and group checks without a trait hierarchy |
| Combined structure predicates | `is_monoid`, `is_group`, `is_abelian_group`, `is_distributive_over`, `is_ring` | Small algebra experiments, education, and validation helpers |

| If you need to... | Start here |
| --- | --- |
| Check whether a binary operation stays inside a sample set | `is_closed_under(...)` |
| Discover a two-sided identity element inside a sample set | `identity_element(...)` |
| Check whether every value has a two-sided inverse | `has_inverses(...)` |
| Validate a finite abelian group or ring model | `is_abelian_group(...)` or `is_ring(...)` |

## When to use it directly

Choose `use-algebra` directly when finite algebra-law checks are the only surface you need and you want to keep that concern narrower than the umbrella facade.

| Scenario | Use `use-algebra` directly? | Why |
| --- | --- | --- |
| You want explicit law checks over a finite sample set | Yes | The crate stays small and does not force a trait hierarchy |
| You are teaching or validating modular arithmetic structures | Yes | The helpers work directly with caller-supplied operations |
| You need symbolic algebra or generic numeric towers | No | Those are intentionally out of scope for this first slice |
| You need matrix-specific or geometry-specific abstractions | Usually no | Those belong in adjacent focused crates |

## Installation

```toml
[dependencies]
use-algebra = "0.0.1"
```

## Quick examples

### Check a finite abelian group

```rust
use use_algebra::{identity_element, is_abelian_group};

let residues = [0_u8, 1, 2];
let add_mod_3 = |left, right| (left + right) % 3;

assert_eq!(identity_element(&residues, add_mod_3), Some(0));
assert!(is_abelian_group(&residues, add_mod_3));
```

### Check a finite ring

```rust
use use_algebra::{is_distributive_over, is_ring};

let residues = [0_u8, 1, 2];
let add_mod_3 = |left, right| (left + right) % 3;
let mul_mod_3 = |left, right| (left * right) % 3;

assert!(is_distributive_over(&residues, mul_mod_3, add_mod_3));
assert!(is_ring(&residues, add_mod_3, mul_mod_3));
```

> [!IMPORTANT]
> These helpers check the laws over the exact finite sample set and exact operations you provide. They do not prove a law for values outside that set.

## Scope

- The current surface is intentionally small and concrete.
- The first slice focuses on finite sample-set checks over caller-supplied binary operations.
- These helpers are law predicates, not symbolic algebra or theorem-proving tools.
- Linear algebra and calculus-specific abstractions belong in adjacent focused crates.

## Status

`use-algebra` is a concrete pre-1.0 crate in the `RustUse` docs surface. The API remains intentionally small while adjacent numeric and linear crates continue to grow around it.
