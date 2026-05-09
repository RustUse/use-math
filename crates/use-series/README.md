# use-series

<p align="center">
    <strong>Composable series primitives for finite and truncated power series.</strong><br>
    Coefficient access, evaluation, truncation, shifting, differentiation, integration, and basic arithmetic.
</p>

<p align="center">
    <img alt="Rust 1.95.0+" src="https://img.shields.io/badge/Rust-1.95.0%2B-f46623?logo=rust&logoColor=white">
    <img alt="Edition 2024" src="https://img.shields.io/badge/edition-2024-0f766e">
    <img alt="f64 coefficients" src="https://img.shields.io/badge/coefficients-f64-1d4ed8">
    <img alt="License MIT or Apache-2.0" src="https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-2a9d8f">
</p>

<p align="center">
    <a href="#what-this-crate-provides">Surface</a> ·
    <a href="#when-to-use-it-directly">When to use it</a> ·
    <a href="#installation">Installation</a> ·
    <a href="#quick-examples">Examples</a> ·
    <a href="#design-notes">Design</a> ·
    <a href="#relationship-to-other-crates">Relationships</a> ·
    <a href="#scope">Scope</a>
</p>

`use-series` provides small utilities for finite and truncated power series. A series is stored as
a `Vec<f64>` of coefficients in ascending degree order: index `i` holds the coefficient of `x^i`.
For example, `[1.0, 2.0, 3.0]` represents `1 + 2x + 3x²`.

## What this crate provides

| Item        | Purpose                                                                          |
| ----------- | -------------------------------------------------------------------------------- |
| `Series`    | Finite or truncated power series stored as `f64` coefficients in ascending order |

### Constructors

| Constructor                                  | Meaning                                     |
| -------------------------------------------- | ------------------------------------------- |
| `Series::new(coefficients)`                  | Build from a coefficient vector             |
| `Series::zero()`                             | The zero series (empty coefficient vector)  |
| `Series::constant(value)`                    | A degree-0 constant series                  |
| `Series::from_coefficients(coefficients)`    | Alias for `new`                             |

### Accessors

| Accessor                     | Returns                                                  |
| ---------------------------- | -------------------------------------------------------- |
| `coefficients()`             | `&[f64]` slice in ascending degree order                 |
| `coefficient(order)`         | Coefficient at the given order, or `0.0` if out of range |
| `order()`                    | `Some(highest_degree)` or `None` for the zero series     |
| `len()`                      | Number of stored coefficients                            |
| `is_empty()`                 | `true` when the series has no coefficients               |
| `is_zero()`                  | `true` when the series is the zero series                |

### Operations

| Method                   | Meaning                                                         |
| ------------------------ | --------------------------------------------------------------- |
| `evaluate(x)`            | Evaluate at `x` using Horner's method                          |
| `truncate(max_order)`    | Keep terms through `max_order` (inclusive)                     |
| `add(other)`             | Elementwise sum                                                 |
| `sub(other)`             | Elementwise difference                                          |
| `mul(other)`             | Cauchy product (term-by-term convolution)                       |
| `scale(scalar)`          | Multiply every coefficient by `scalar`                         |
| `shift(amount)`          | Multiply by `x^amount` (prepend zeros)                         |
| `derivative()`           | Formal derivative                                               |
| `integral(constant)`     | Formal integral with the given constant term                    |

## When to use it directly

Choose `use-series` directly when finite or truncated power series primitives are the only math
surface you need.

| Scenario                                                     | Use `use-series` directly? | Why                                          |
| ------------------------------------------------------------ | -------------------------- | -------------------------------------------- |
| You need series evaluation, truncation, or arithmetic        | Yes                        | The crate is focused and purpose-built       |
| You also need geometry or combinatorics helpers              | Usually no                 | `use-math` may be the cleaner surface        |
| You need convergence analysis or infinite lazy series        | No                         | Intentionally out of scope for this crate    |
| You need symbolic algebra or exact rational coefficients     | No                         | Intentionally deferred                       |

## Installation

```toml
[dependencies]
use-series = { git = "https://github.com/RustUse/use-math", rev = "<commit>" }
```

After the first crates.io release:

```toml
[dependencies]
use-series = "0.0.1"
```

## Quick examples

### Build and evaluate a series

```rust
use use_series::Series;

let s = Series::new(vec![1.0, 2.0, 3.0]); // 1 + 2x + 3x²

let value = s.evaluate(2.0);
assert_eq!(value, 17.0); // 1 + 4 + 12 = 17
```

### Differentiation and integration

```rust
use use_series::Series;

let s = Series::new(vec![1.0, 2.0, 3.0]); // 1 + 2x + 3x²

let d = s.derivative();                    // 2 + 6x
assert_eq!(d.coefficients(), &[2.0, 6.0]);

let i = s.integral(0.0);                   // x + x² + x³
assert_eq!(i.coefficients(), &[0.0, 1.0, 1.0, 1.0]);
```

### Arithmetic

```rust
use use_series::Series;

let a = Series::new(vec![1.0, 2.0]);       // 1 + 2x
let b = Series::new(vec![3.0, 4.0]);       // 3 + 4x

let sum = a.add(&b);                        // 4 + 6x
assert_eq!(sum.coefficients(), &[4.0, 6.0]);

let product = a.mul(&b);                    // (1 + 2x)(3 + 4x) = 3 + 10x + 8x²
assert_eq!(product.coefficients(), &[3.0, 10.0, 8.0]);
```

### Truncation and shifting

```rust
use use_series::Series;

let s = Series::new(vec![1.0, 2.0, 3.0, 4.0]); // 1 + 2x + 3x² + 4x³

let t = s.truncate(2);                           // 1 + 2x + 3x²
assert_eq!(t.coefficients(), &[1.0, 2.0, 3.0]);

let shifted = Series::new(vec![1.0, 2.0]).shift(2); // x² + 2x³
assert_eq!(shifted.coefficients(), &[0.0, 0.0, 1.0, 2.0]);
```

## Design notes

### Coefficient storage and normalization

Coefficients are stored in ascending degree order. `Series::new` normalizes trailing zero
coefficients so that the degree can be read directly from `coefficients().len() - 1`. The zero
series is represented as an empty coefficient vector.

### Zero series

`Series::zero()` and any series whose coefficients normalize to all zeros are represented with an
empty coefficient vector. `is_zero()` and `is_empty()` both return `true` for the zero series, and
`order()` returns `None`.

### Evaluation

`evaluate(x)` uses Horner's method for numerical stability and efficiency:
`(...((aₙ · x + aₙ₋₁) · x + aₙ₋₂) · ... + a₀)`.

### Floating-point coefficients

`use-series` v0.1 accepts `f64` coefficients without validation. NaN and infinite coefficients are
stored and propagated as received. Exact rational coefficients are a deferred follow-up.

## Relationship to other crates

| Crate               | Relationship                                                                                       |
| ------------------- | -------------------------------------------------------------------------------------------------- |
| `use-polynomial`    | Planned sibling crate for evaluated polynomial forms; `use-series` handles the coefficient layer   |
| `use-catalan`       | Will build on `use-series` to express Catalan generating functions as series                       |
| `use-hyper-catalan` | Will build on `use-series` for hyper-Catalan series representations                               |
| `use-geode`         | Will build on `use-series` for geode series expansions                                             |

> [!NOTE]
> `use-series` is a direct crate under `use-math`. RustUse set repositories stay one layer deep:
> this crate does not contain nested child crates.

## Scope

- Finite and truncated power series with `f64` coefficients
- Coefficient access, evaluation (Horner's method), truncation, and shifting
- Basic arithmetic: add, subtract, multiply, scale
- Formal differentiation and integration
- Infinite lazy series are intentionally out of scope
- Convergence analysis is intentionally out of scope
- Symbolic algebra and exact rational coefficients are deferred
- Generating function utilities are deferred
- Fourier series are intentionally out of scope

## Status

`use-series` is a pre-release direct crate under `RustUse/use-math`. The API remains pre-1.0.
