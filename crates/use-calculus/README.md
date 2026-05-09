# use-calculus

<p align="center">
    <strong>Small numerical-calculus helpers for direct, explicit Rust code.</strong><br>
    Finite-difference derivatives, numerical integration, and simple symmetric limit estimates without a symbolic framework.
</p>

<p align="center">
    <img alt="Rust 1.95.0+" src="https://img.shields.io/badge/Rust-1.95.0%2B-f46623?logo=rust&logoColor=white">
    <img alt="Edition 2024" src="https://img.shields.io/badge/edition-2024-0f766e">
    <img alt="Numerical calculus" src="https://img.shields.io/badge/calculus-numerical-1d4ed8">
    <img alt="Explicit validation" src="https://img.shields.io/badge/validation-explicit-c2410c">
    <img alt="License MIT or Apache-2.0" src="https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-2a9d8f">
</p>

<p align="center">
    <a href="#what-this-crate-provides">Surface</a> ·
    <a href="#when-to-use-it-directly">When to use it</a> ·
    <a href="#installation">Installation</a> ·
    <a href="#quick-examples">Examples</a> ·
    <a href="#validation-model">Validation</a> ·
    <a href="#scope">Scope</a>
</p>

`use-calculus` provides a deliberately small numerical-calculus surface. The crate prefers explicit assumptions over magic defaults: finite-difference steps are caller-chosen, integration sample counts are explicit, and two-sided limit estimates fail when the left and right samples do not agree within a caller-provided tolerance.

<table>
    <tr>
        <td width="33%" valign="top">
            <strong>Differentiation</strong><br>
            <code>Differentiator</code>, <code>central_difference</code>, and second-derivative helpers for small direct estimates.
        </td>
        <td width="33%" valign="top">
            <strong>Integration</strong><br>
            <code>IntegrationInterval</code>, <code>Integrator</code>, trapezoidal integration, and Simpson integration.
        </td>
        <td width="33%" valign="top">
            <strong>Explicit failure modes</strong><br>
            Non-finite inputs, invalid steps, bad sample counts, and mismatched one-sided limits return <code>CalculusError</code>.
        </td>
    </tr>
</table>

## What this crate provides

| Area | Root exports | Best fit |
| --- | --- | --- |
| Differentiation | `Differentiator`, `central_difference`, `second_central_difference` | Small slope and curvature estimates on `f64` functions |
| Integration | `IntegrationInterval`, `Integrator`, `trapezoidal_integral`, `simpson_integral` | Signed area approximations with explicit sample density |
| Limits | `LimitApproximator`, `symmetric_limit` | Two-sided limits where a single symmetric sample scale is a reasonable model |

| If you need to... | Start here |
| --- | --- |
| Estimate a derivative at one point | `Differentiator::derivative_at(...)` |
| Approximate a definite integral | `Integrator::trapezoidal(...)` or `Integrator::simpson(...)` |
| Model a signed interval directly | `IntegrationInterval::try_new(...)` |
| Reject discontinuities instead of hiding them | `symmetric_limit(...)` or `LimitApproximator` |

## When to use it directly

Choose `use-calculus` directly when numerical-calculus helpers are the only math surface you need, or when you want the approximation policy to stay visible instead of being hidden behind a broader abstraction.

| Scenario | Use `use-calculus` directly? | Why |
| --- | --- | --- |
| You need a few direct derivative or integral estimates | Yes | The crate stays small and explicit |
| You want validation at the boundary of your app | Yes | Invalid steps, bounds, and evaluations become errors |
| You need symbolic manipulation or automatic differentiation | No | This crate intentionally stays numerical and lightweight |
| You also need several other math surfaces | Usually no | `use-math` may become the better integration point |

## Installation

```toml
[dependencies]
use-calculus = "0.0.1"
```

## Quick examples

### Differentiate and integrate small `f64` functions

```rust
use use_calculus::{Differentiator, IntegrationInterval, Integrator};

let differentiator = Differentiator::try_new(1.0e-5)?;
let interval = IntegrationInterval::try_new(0.0, 1.0)?;
let integrator = Integrator::try_new(128)?;

let slope = differentiator.derivative_at(|x| x.powi(3), 2.0)?;
let area = integrator.simpson(|x| x * x, interval)?;

assert!((slope - 12.0).abs() < 1.0e-3);
assert!((area - (1.0 / 3.0)).abs() < 1.0e-6);
# Ok::<(), use_calculus::CalculusError>(())
```

### Explicitly reject mismatched one-sided limits

```rust
use use_calculus::{CalculusError, LimitApproximator};

let limit = LimitApproximator::try_new(1.0e-6, 1.0e-5)?;
let sinc_limit = limit.at(
    |x| {
        if x == 0.0 {
            1.0
        } else {
            x.sin() / x
        }
    },
    0.0,
)?;

assert!((sinc_limit - 1.0).abs() < 1.0e-5);
assert!(matches!(
    limit.at(|x| if x < 0.0 { -1.0 } else { 1.0 }, 0.0),
    Err(CalculusError::LimitMismatch { .. })
));
# Ok::<(), use_calculus::CalculusError>(())
```

## Validation model

Use `try_new` when sample sizes, step sizes, tolerances, and interval bounds may come from user input, files, or network payloads. Use infallible constructors like `Differentiator::new(...)`, `IntegrationInterval::new(...)`, `Integrator::new(...)`, and `LimitApproximator::new(...)` when values are already trusted and you want direct assembly.

> [!IMPORTANT]
> This crate does not hide approximation policy behind a global epsilon or default grid density. Callers choose the differentiation step, integration density, and limit tolerance directly.

## Scope

- Small numerical APIs are preferred over symbolic or trait-heavy abstractions.
- The initial concrete surface is `f64`-based and focused on explicit approximation helpers.
- Adaptive quadrature, automatic differentiation, and symbolic manipulation are intentionally out of scope for this first slice.
- Broader real-number utilities and statistical analysis belong in adjacent focused crates.

## Status

`use-calculus` is a concrete pre-1.0 crate in the `RustUse` docs surface. The API remains intentionally small while the broader calculus roadmap is still being designed.
