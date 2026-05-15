# use-arithmetic

<p align="center">
    <strong>Small arithmetic primitives for `RustUse`.</strong><br>
    Exact gcd and lcm helpers, divisibility and parity checks, floor-division helpers, and lightweight checked, saturating, and wrapping arithmetic wrappers over primitive integers.
</p>

<p align="center">
    <img alt="Rust 1.95.0+" src="https://img.shields.io/badge/Rust-1.95.0%2B-f46623?logo=rust&logoColor=white">
    <img alt="Edition 2024" src="https://img.shields.io/badge/edition-2024-0f766e">
    <img alt="Arithmetic helpers" src="https://img.shields.io/badge/arithmetic-helpers-1d4ed8">
    <img alt="License MIT or Apache-2.0" src="https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-2a9d8f">
</p>

## Install

```toml
[dependencies]
use-arithmetic = "0.0.2"
```

## Foundation

`use-arithmetic` provides a deliberately small operation-level arithmetic surface for primitive integers. The current slice focuses on exact common-divisor helpers, divisibility and parity checks, mathematical floor-division behavior for signed integers, and lightweight wrappers around primitive checked, saturating, and wrapping arithmetic.

This crate is about concrete arithmetic operations. It intentionally avoids integer classification, number taxonomy, rational types, real-number abstractions, or algebraic law checking.

<table>
    <tr>
        <td width="33%" valign="top">
            <strong>Common-divisor helpers</strong><br>
            <code>gcd</code>, <code>checked_lcm</code>, and <code>lcm</code> keep exact divisor and multiple calculations explicit.
        </td>
        <td width="33%" valign="top">
            <strong>Division semantics</strong><br>
            <code>div_floor</code>, <code>div_ceil</code>, and <code>mod_floor</code> follow mathematical floor semantics instead of Rust's truncating signed division.
        </td>
        <td width="33%" valign="top">
            <strong>Overflow-mode wrappers</strong><br>
            The checked, saturating, and wrapping helpers make primitive integer overflow behavior explicit without forcing callers to reach for methods directly.
        </td>
    </tr>
</table>

| Area                     | Primary items                                                                     | Best fit                                                                    |
| ------------------------ | --------------------------------------------------------------------------------- | --------------------------------------------------------------------------- |
| Common divisors          | `gcd`, `checked_lcm`, `lcm`                                                       | Exact multiple and normalization workflows over primitive integers          |
| Divisibility and parity  | `checked_is_divisible_by`, `is_divisible_by`, `is_even`, `is_odd`                 | Small runtime checks without ad hoc `%` handling at every call site         |
| Floor division           | `checked_div_floor`, `checked_div_ceil`, `checked_mod_floor`, plain variants      | Signed integer code that needs mathematical floor semantics                 |
| Overflow-mode operations | `checked_add`, `checked_sub`, `checked_mul`, saturating helpers, wrapping helpers | Call sites that want consistent overflow behavior across primitive integers |

| Neighboring crate | Best fit there                                                             |
| ----------------- | -------------------------------------------------------------------------- |
| `use-integer`     | Sign classification, signed integer helpers, and integer-specific concerns |
| `use-number`      | Broader number classification and shared numeric constants                 |
| `use-rational`    | Exact fraction types and rational arithmetic                               |
| `use-real`        | Finite-value, interval, and approximate real-number helpers                |
| `use-algebra`     | Finite algebra law checks over caller-supplied operations                  |

## When to use directly

Choose `use-arithmetic` directly when operation-level helpers are the only surface you need and you want to keep that concern narrower than the full facade crate.

| Scenario                                                            | Use `use-arithmetic` directly? | Why                                                                       |
| ------------------------------------------------------------------- | ------------------------------ | ------------------------------------------------------------------------- |
| You need exact gcd and lcm helpers over primitive unsigned integers | Yes                            | The crate stays small and focused on direct arithmetic operations         |
| You need signed floor-division helpers with mathematical semantics  | Yes                            | The division helpers make the quotient and modulo policy explicit         |
| You want reusable checked, saturating, or wrapping wrappers         | Yes                            | The free functions keep primitive overflow modes visible at the call site |
| You need signed classification or rational/real abstractions        | Usually no                     | Those concerns already live in adjacent focused crates                    |

## Scope

- The current surface is intentionally small and concrete.
- The first slice focuses on primitive arithmetic operations rather than wrapper types or trait-heavy abstractions.
- `use-integer` remains the home for signed classification and integer-specific descriptive helpers.
- Slice-based statistical means, rational arithmetic, and algebraic structures belong in adjacent focused crates.

## Examples

### Compute gcd, lcm, divisibility, and parity

```rust
use use_arithmetic::{gcd, is_divisible_by, is_even, is_odd, lcm};

assert_eq!(gcd(54, 24), 6);
assert_eq!(lcm(6, 15), 30);
assert!(is_divisible_by(84, 7));
assert!(!is_divisible_by(84, 0));
assert!(is_even(12));
assert!(is_odd(7));
```

### Use floor-division semantics for signed integers

```rust
use use_arithmetic::{div_ceil, div_floor, mod_floor};

assert_eq!(div_floor(-7, 3), -3);
assert_eq!(div_ceil(-7, 3), -2);
assert_eq!(mod_floor(-7, 3), 2);
```

### Choose an overflow mode explicitly

```rust
use use_arithmetic::{checked_add, saturating_add, wrapping_mul};

assert_eq!(checked_add(u8::MAX, 1), None);
assert_eq!(saturating_add(u8::MAX, 1), u8::MAX);
assert_eq!(wrapping_mul(200_u8, 2), 144);
```

## Status

`use-arithmetic` is a concrete pre-1.0 crate in the `RustUse` math workspace. The API remains intentionally small while adjacent numeric, integer, and algebra crates continue to grow around it.
