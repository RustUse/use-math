# use-set

<p align="center">
    <strong>Small mathematical set helpers for `RustUse`.</strong><br>
    Explicit membership, subset, disjointness, and order-preserving set operations over slices.
</p>

<p align="center">
    <img alt="Rust 1.95.0+" src="https://img.shields.io/badge/Rust-1.95.0%2B-f46623?logo=rust&logoColor=white">
    <img alt="Edition 2024" src="https://img.shields.io/badge/edition-2024-0f766e">
    <img alt="Set helpers" src="https://img.shields.io/badge/set-helpers-1d4ed8">
    <img alt="License MIT or Apache-2.0" src="https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-2a9d8f">
</p>

## Install

```toml
[dependencies]
use-set = "0.0.1"
```

## Foundation

`use-set` provides a deliberately small set-theoretic surface over ordinary slices. It treats duplicates as membership noise, exposes direct predicates such as `contains_member`, `is_subset`, and `are_disjoint`, and returns unique `Vec<T>` values for set operations while preserving first-seen order. The API stays explicit and dependency-free instead of introducing a wrapper collection type.

<table>
    <tr>
        <td width="33%" valign="top">
            <strong>Membership and relations</strong><br>
            <code>contains_member</code>, <code>is_subset</code>, and <code>are_disjoint</code> make set intent explicit at the call site.
        </td>
        <td width="33%" valign="top">
            <strong>Order-preserving operations</strong><br>
            <code>set_union</code>, <code>set_intersection</code>, and <code>set_difference</code> return unique results without losing the order of first appearance.
        </td>
        <td width="33%" valign="top">
            <strong>Symmetric difference</strong><br>
            <code>set_symmetric_difference</code> covers the common "in exactly one side" case directly.
        </td>
    </tr>
</table>

| Helper group                 | Primary items                                     | Best fit                                                                                          |
| ---------------------------- | ------------------------------------------------- | ------------------------------------------------------------------------------------------------- |
| Membership and relations     | `contains_member`, `is_subset`, `are_disjoint`    | Call sites that want readable set checks over existing slices                                     |
| Set operations               | `set_union`, `set_intersection`, `set_difference` | Small apps and utilities that need explicit, unique results without introducing a custom set type |
| Exclusive-membership helpers | `set_symmetric_difference`                        | Comparing two slices for values that appear on exactly one side                                   |

## When to use directly

Choose `use-set` directly when mathematical set helpers are the only surface you need and you want to keep that concern narrower than the umbrella facade.

| Scenario                                                               | Use `use-set` directly? | Why                                                                       |
| ---------------------------------------------------------------------- | ----------------------- | ------------------------------------------------------------------------- |
| You already have slices and want explicit set predicates               | Yes                     | The API avoids extra collection wrappers and keeps intent obvious         |
| You need unique set-operation results but still care about input order | Yes                     | The operation helpers preserve first-seen order while removing duplicates |
| You also need geometry, probability, or other math domains             | Usually no              | `use-math` can compose the concrete surfaces behind features              |

## Scope

- The current surface is intentionally small and concrete.
- Helpers work over ordinary slices and cloned output vectors instead of a custom set type.
- Boolean logic and general-purpose collection utilities belong in adjacent or external crates.

## Examples

### Membership and relations

```rust
use use_set::{are_disjoint, contains_member, is_subset};

let left = [1, 2, 2, 3];
let right = [2, 3, 4];

assert!(contains_member(&left, &2));
assert!(is_subset(&[2, 3], &right));
assert!(!are_disjoint(&left, &right));
```

### Order-preserving set operations

```rust
use use_set::{set_difference, set_intersection, set_symmetric_difference, set_union};

let left = [1, 2, 2, 3];
let right = [3, 4, 2, 5];

assert_eq!(set_union(&left, &right), vec![1, 2, 3, 4, 5]);
assert_eq!(set_intersection(&left, &right), vec![2, 3]);
assert_eq!(set_difference(&left, &right), vec![1]);
assert_eq!(set_symmetric_difference(&left, &right), vec![1, 4, 5]);
```

## Status

`use-set` is a concrete pre-1.0 crate in the `RustUse` docs surface. The API remains intentionally small while adjacent collection and algebra crates continue to grow around it.
