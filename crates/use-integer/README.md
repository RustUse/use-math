# use-integer

<p align="center">
    <strong>Small integer classifications and common-divisor helpers for `RustUse`.</strong><br>
    Parity, sign classification, divisibility checks, and explicit gcd/lcm helpers without a broader numeric framework.
</p>

<p align="center">
    <img alt="Rust 1.95.0+" src="https://img.shields.io/badge/Rust-1.95.0%2B-f46623?logo=rust&logoColor=white">
    <img alt="Edition 2024" src="https://img.shields.io/badge/edition-2024-0f766e">
    <img alt="Integer helpers" src="https://img.shields.io/badge/integer-helpers-1d4ed8">
    <img alt="License MIT or Apache-2.0" src="https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-2a9d8f">
</p>

## Install

```toml
[dependencies]
use-integer = "0.0.1"
```

## Foundation

`use-integer` provides a deliberately small integer helper surface. The crate covers sign classification, parity checks, runtime divisibility validation, and non-negative gcd/lcm helpers for signed `i128` inputs. Operations that need a non-zero divisor or that would overflow while forming an `lcm` return `IntegerError` explicitly instead of silently panicking or widening the API surface.

<table>
    <tr>
        <td width="33%" valign="top">
            <strong>Classification helpers</strong><br>
            <code>IntegerSign</code>, <code>classify_sign</code>, <code>is_even</code>, and <code>is_odd</code> keep branching on sign or parity explicit.
        </td>
        <td width="33%" valign="top">
            <strong>Checked divisibility</strong><br>
            <code>is_divisible_by</code> reports <code>IntegerError::DivisionByZero</code> instead of leaving divisor validation to every caller.
        </td>
        <td width="33%" valign="top">
            <strong>Common-divisor helpers</strong><br>
            <code>gcd</code>, <code>lcm</code>, and <code>are_coprime</code> provide small exact helpers for normalization and scheduling-style logic.
        </td>
    </tr>
</table>

| Helper group    | Primary items                                       | Best fit                                                          |
| --------------- | --------------------------------------------------- | ----------------------------------------------------------------- |
| Sign and parity | `IntegerSign`, `classify_sign`, `is_even`, `is_odd` | Branching on signed integer shape without ad hoc match logic      |
| Divisibility    | `is_divisible_by`, `IntegerError`                   | User-supplied divisors or explicit validation paths               |
| Common divisors | `gcd`, `lcm`, `are_coprime`                         | Normalization, factor checks, and least-common-multiple workflows |

## When to use directly

Choose `use-integer` directly when integer helpers are the only surface you need and you want to keep that concern separate from broader numeric or algebraic APIs.

| Scenario                                                    | Use `use-integer` directly? | Why                                                                          |
| ----------------------------------------------------------- | --------------------------- | ---------------------------------------------------------------------------- |
| You only need parity, sign, or divisibility helpers         | Yes                         | The crate stays smaller than the facade and avoids unrelated numeric modules |
| You want exact gcd/lcm helpers over signed `i128` values    | Yes                         | The API already handles sign normalization and explicit error cases          |
| You also need rational, real, probability, or geometry APIs | Usually no                  | `use-math` can unify imports behind features                                 |

## Scope

- The current surface is intentionally small and concrete.
- Integer helpers stay function-oriented instead of introducing wrapper types with no extra invariants.
- Broader numeric abstractions belong in `use-number`, while rational or algebraic APIs belong in their own focused crates.

## Examples

### Classify sign and parity

```rust
use use_integer::{IntegerSign, classify_sign, is_even, is_odd};

assert_eq!(classify_sign(-7), IntegerSign::Negative);
assert!(is_even(12));
assert!(is_odd(7));
```

### Check divisibility and common divisors

```rust
use use_integer::{are_coprime, gcd, is_divisible_by, lcm};

assert!(is_divisible_by(84, 7)?);
assert_eq!(gcd(-54, 24), 6);
assert_eq!(lcm(-6, 15)?, 30);
assert!(are_coprime(35, 64));

# Ok::<(), use_integer::IntegerError>(())
```

## Status

`use-integer` is a concrete pre-1.0 crate in the `RustUse` docs surface. The API remains intentionally small while adjacent numeric crates are built out around it.
