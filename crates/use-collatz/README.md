# use-collatz

<p align="center">
    <strong>Small Collatz trajectory utilities for `RustUse`.</strong><br>
    Dependency-free helpers for exploring Collatz sequences, stopping times, parity patterns, and bounded range verification over positive integers.
</p>

<p align="center">
    <img alt="Rust 1.95.0+" src="https://img.shields.io/badge/Rust-1.95.0%2B-f46623?logo=rust&logoColor=white">
    <img alt="Edition 2024" src="https://img.shields.io/badge/edition-2024-0f766e">
    <img alt="Collatz utilities" src="https://img.shields.io/badge/collatz-utilities-1d4ed8">
    <img alt="License MIT or Apache-2.0" src="https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-2a9d8f">
</p>

## Install

```toml
[dependencies]
use-collatz = "0.0.6"
```

## Foundation

`use-collatz` provides a deliberately small surface for computing with the Collatz map on positive `u64` inputs:

- if `n` is even, the next value is `n / 2`
- if `n` is odd, the next value is `3n + 1`

The Collatz conjecture remains unproven. This crate does not claim to prove it or to settle its global behavior. Instead, it focuses on bounded computational exploration: generating trajectories, measuring stopping behavior, inspecting parity patterns, and summarizing finite inclusive ranges.

<table>
    <tr>
        <td width="33%" valign="top">
            <strong>Trajectory helpers</strong><br>
            <code>collatz_next</code>, <code>collatz_sequence</code>, <code>trajectory_len</code>, and <code>max_value_in_trajectory</code> expose the concrete path followed by a starting value.
        </td>
        <td width="33%" valign="top">
            <strong>Stopping-time helpers</strong><br>
            <code>stopping_time</code>, <code>total_stopping_time</code>, and <code>reaches_one</code> keep step counts and overflow behavior explicit.
        </td>
        <td width="33%" valign="top">
            <strong>Parity and range summaries</strong><br>
            <code>CollatzParity</code>, <code>parity_vector</code>, and <code>verify_range</code> support quick trajectory inspection across bounded inputs.
        </td>
    </tr>
</table>

## When to use directly

Choose `use-collatz` directly when Collatz trajectory analysis is the only math support you need and you want to keep that concern narrow and dependency-free.

| Scenario                                                      | Use `use-collatz` directly? | Why                                                                     |
| ------------------------------------------------------------- | --------------------------- | ----------------------------------------------------------------------- |
| You need the explicit trajectory for one starting value       | Yes                         | The crate keeps the sequence utilities small and concrete               |
| You need stopping times or peak values for bounded inputs     | Yes                         | The API centers on direct trajectory metrics without extra abstractions |
| You want parity patterns for exploratory experiments          | Yes                         | The parity helpers stay aligned with the trajectory surface             |
| You need broader numeric, algebraic, or symbolic abstractions | Usually no                  | Those belong in adjacent focused crates or the `use-math` facade        |

## Scope

- The current surface is intentionally small and concrete.
- All public helpers operate on positive `u64` inputs.
- Odd steps use checked arithmetic so overflow returns `None` instead of wrapping.
- The crate is for bounded computational exploration, not proof, exhaustive classification, or conjecture claims.

## Examples

### Generate a full trajectory

```rust
use use_collatz::{collatz_sequence, total_stopping_time};

assert_eq!(collatz_sequence(6), Some(vec![6, 3, 10, 5, 16, 8, 4, 2, 1]));
assert_eq!(total_stopping_time(6), Some(8));
```

### Inspect parity and range summaries

```rust
use use_collatz::{CollatzParity, parity_vector, verify_range};

assert_eq!(
    parity_vector(6),
    Some(vec![
        CollatzParity::Even,
        CollatzParity::Odd,
        CollatzParity::Even,
        CollatzParity::Odd,
        CollatzParity::Even,
        CollatzParity::Even,
        CollatzParity::Even,
        CollatzParity::Even,
    ])
);

let summary = verify_range(1, 10);

assert_eq!(summary.checked, 10);
assert_eq!(summary.reached_one, 10);
assert_eq!(summary.overflowed, 0);
assert_eq!(summary.max_total_stopping_time, Some((9, 19)));
assert_eq!(summary.max_trajectory_value, Some((7, 52)));
```

## Status

`use-collatz` is a concrete pre-1.0 crate in the `RustUse` math workspace. The API remains intentionally small while adjacent sequence and number-theory crates continue to grow around it.
