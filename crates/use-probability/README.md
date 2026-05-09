# use-probability

<p align="center">
    <strong>Small probability primitives for direct, explicit Rust code.</strong><br>
    Validated probabilities, independent-event helpers, and a compact Bernoulli model without a larger statistics framework.
</p>

<p align="center">
    <img alt="Rust 1.95.0+" src="https://img.shields.io/badge/Rust-1.95.0%2B-f46623?logo=rust&logoColor=white">
    <img alt="Edition 2024" src="https://img.shields.io/badge/edition-2024-0f766e">
    <img alt="Probability primitives" src="https://img.shields.io/badge/probability-primitives-1d4ed8">
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

`use-probability` provides a deliberately small probability surface. The crate prefers explicit value types over loose `f64` conventions: probabilities are validated to stay in the closed interval `[0, 1]`, independent-event composition is spelled out directly, and Bernoulli probabilities keep success and failure semantics attached to a named type.

<table>
    <tr>
        <td width="33%" valign="top">
            <strong>Validated probability values</strong><br>
            <code>Probability</code> keeps normalization explicit and provides complements and ratio construction.
        </td>
        <td width="33%" valign="top">
            <strong>Event-combination helpers</strong><br>
            <code>independent_intersection</code> and <code>independent_union</code> encode the independence assumption directly.
        </td>
        <td width="33%" valign="top">
            <strong>Bernoulli workflows</strong><br>
            <code>Bernoulli</code> exposes success and failure probabilities, mean, variance, and mass values for binary outcomes.
        </td>
    </tr>
</table>

## What this crate provides

| Area                    | Root exports                                    | Best fit                                                                    |
| ----------------------- | ----------------------------------------------- | --------------------------------------------------------------------------- |
| Probability values      | `Probability`, `ProbabilityError`               | Explicit normalized probabilities instead of raw floating-point conventions |
| Independent-event rules | `independent_intersection`, `independent_union` | Binary event composition when independence is part of the model             |
| Bernoulli outcomes      | `Bernoulli`                                     | Success/failure probabilities with direct mean, variance, and PMF helpers   |

| If you need to...                                | Start here                                                   |
| ------------------------------------------------ | ------------------------------------------------------------ |
| Validate a probability value from external input | `Probability::try_new(...)`                                  |
| Build a probability from counts                  | `Probability::from_fraction(...)`                            |
| Combine independent events explicitly            | `independent_intersection(...)` and `independent_union(...)` |
| Model a binary success/failure process           | `Bernoulli`                                                  |

## When to use it directly

Choose `use-probability` directly when probability primitives are the only math surface you need, or when you want probability assumptions to stay local and visible instead of being diffused through a broader crate.

| Scenario                                                       | Use `use-probability` directly? | Why                                                         |
| -------------------------------------------------------------- | ------------------------------- | ----------------------------------------------------------- |
| You only need normalized probabilities and small binary models | Yes                             | The crate stays narrow and explicit                         |
| You want validation at the boundary of your app                | Yes                             | Out-of-range probabilities and invalid ratios become errors |
| You need descriptive statistics or inference helpers           | Usually no                      | Those belong in adjacent focused crates                     |
| You need random sampling or RNG integration                    | No                              | This crate intentionally stops short of a randomness API    |

## Installation

```toml
[dependencies]
use-probability = "0.0.1"
```

## Quick examples

### Compose small independent-event probabilities

```rust
use use_probability::{Probability, independent_intersection, independent_union};

let rain = Probability::from_fraction(1, 4)?;
let traffic = Probability::try_new(0.5)?;

let joint = independent_intersection(rain, traffic);
let either = independent_union(rain, traffic);

assert!((joint.value() - 0.125).abs() < 1.0e-12);
assert!((either.value() - 0.625).abs() < 1.0e-12);
# Ok::<(), use_probability::ProbabilityError>(())
```

### Work with Bernoulli success and failure directly

```rust
use use_probability::{Bernoulli, Probability};

let success = Probability::from_fraction(1, 4)?;
let trial = Bernoulli::new(success);

assert_eq!(trial.success_probability(), success);
assert_eq!(trial.failure_probability(), Probability::try_new(0.75)?);
assert!((trial.mean() - 0.25).abs() < 1.0e-12);
assert!((trial.variance() - 0.1875).abs() < 1.0e-12);
# Ok::<(), use_probability::ProbabilityError>(())
```

## Validation model

Use `try_new` and `from_fraction` when values may come from user input, files, or other external sources. Use infallible constructors like `Probability::new(...)` and `Bernoulli::new(...)` only when normalization is already guaranteed by the surrounding code.

> [!IMPORTANT]
> Independent-event helpers assume independence because the function name says so. This crate does not hide modeling assumptions behind generic event-combination helpers.

## Scope

- Small probability APIs are preferred over broad distribution frameworks.
- The initial concrete surface focuses on normalized values, direct independent-event rules, and Bernoulli outcomes.
- Random sampling, RNG integration, and descriptive statistics are intentionally out of scope for this first slice.
- Broader inference and summary APIs belong in adjacent focused crates.

## Status

`use-probability` is a concrete pre-1.0 crate in the `RustUse` docs surface. The API remains intentionally small while the broader probability roadmap is still being designed.
