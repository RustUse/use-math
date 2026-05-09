# use-trigonometry

<p align="center">
    <strong>Small angle and trigonometry helpers for `RustUse`.</strong><br>
    Explicit degree/radian handling, normalization, and direct sine, cosine, and tangent helpers.
</p>

<p align="center">
    <img alt="Rust 1.95.0+" src="https://img.shields.io/badge/Rust-1.95.0%2B-f46623?logo=rust&logoColor=white">
    <img alt="Edition 2024" src="https://img.shields.io/badge/edition-2024-0f766e">
    <img alt="Angle helpers" src="https://img.shields.io/badge/angle-helpers-1d4ed8">
    <img alt="License MIT or Apache-2.0" src="https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-2a9d8f">
</p>

## Install

```toml
[dependencies]
use-trigonometry = "0.0.1"
```

## Foundation

`use-trigonometry` provides a deliberately small trigonometry surface around explicit angle units. The crate exposes an `Angle` type stored in radians, conversion helpers for degrees and radians, normalization helpers for wrapped inputs, and direct `sin`, `cos`, and `tan` utilities for degree-based call sites. The first slice stays close to ordinary `f64` values instead of introducing a larger symbolic or identity-heavy framework.

<table>
    <tr>
        <td width="33%" valign="top">
            <strong>Explicit units</strong><br>
            <code>Angle</code>, <code>degrees_to_radians</code>, and <code>radians_to_degrees</code> keep unit intent visible at the call site.
        </td>
        <td width="33%" valign="top">
            <strong>Wrapping helpers</strong><br>
            <code>normalized</code>, <code>normalized_signed</code>, <code>normalize_degrees</code>, and <code>normalize_radians</code> handle wrapped inputs directly.
        </td>
        <td width="33%" valign="top">
            <strong>Basic trig evaluation</strong><br>
            <code>sin</code>, <code>cos</code>, <code>tan</code>, <code>sin_deg</code>, <code>cos_deg</code>, and <code>tan_deg</code> cover the common numeric cases.
        </td>
    </tr>
</table>

| Helper group | Primary items | Best fit |
| ------------ | ------------- | -------- |
| Angle values and unit conversion | `Angle`, `degrees_to_radians`, `radians_to_degrees` | Code that should stay explicit about whether an input is degrees or radians |
| Normalization | `Angle::normalized`, `Angle::normalized_signed`, `normalize_degrees`, `normalize_radians` | Handling wrapped or cyclic inputs before downstream math |
| Direct trig helpers | `Angle::sin`, `Angle::cos`, `Angle::tan`, `sin_deg`, `cos_deg`, `tan_deg` | Small apps and utilities that want direct numeric trig without a larger abstraction |

## When to use directly

Choose `use-trigonometry` directly when angle or trigonometric utilities are the only surface you need and you want to keep that concern narrower than the full facade.

| Scenario | Use `use-trigonometry` directly? | Why |
| -------- | ------------------------------- | --- |
| You need explicit degree/radian handling without a broader math facade | Yes | The crate keeps unit intent local and direct |
| You need wrapped-angle normalization and basic trig evaluation | Yes | The current surface already covers those cases with a small API |
| You also need geometry, calculus, or other math domains | Usually no | `use-math` can compose the concrete surfaces behind features |

## Scope

- The current surface is intentionally small and concrete.
- Helpers stay numeric and explicit instead of introducing symbolic identities or generalized expression trees.
- Geometry-specific orientation and calculus-specific analytic helpers belong in adjacent focused crates.

## Examples

### Angle conversion and normalization

```rust
use use_trigonometry::{Angle, degrees_to_radians, normalize_degrees, radians_to_degrees};

let acute = Angle::from_degrees(30.0);
let wrapped = Angle::from_degrees(765.0).normalized();

assert!((acute.radians() - degrees_to_radians(30.0)).abs() < 1.0e-12);
assert!((acute.degrees() - radians_to_degrees(acute.radians())).abs() < 1.0e-12);
assert!((wrapped.degrees() - 45.0).abs() < 1.0e-12);
assert!((normalize_degrees(-90.0) - 270.0).abs() < 1.0e-12);
```

### Direct trig helpers

```rust
use use_trigonometry::{Angle, cos_deg, sin_deg, tan_deg};

let acute = Angle::from_degrees(30.0);

assert!((acute.sin() - 0.5).abs() < 1.0e-12);
assert!((acute.cos() - cos_deg(30.0)).abs() < 1.0e-12);
assert!((sin_deg(30.0) - 0.5).abs() < 1.0e-12);
assert!((tan_deg(45.0) - 1.0).abs() < 1.0e-12);
```

## Status

`use-trigonometry` is a concrete pre-1.0 crate in the `RustUse` docs surface. The API remains intentionally small while adjacent geometry and calculus crates continue to grow around it.
