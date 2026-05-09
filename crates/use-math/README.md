# use-math

<p align="center">
	<strong>Feature-gated <code>RustUse</code> facade for concrete geometry, checked counting, complex numbers, numerical calculus, probability, real-number primitives, and rational arithmetic plus scaffolded namespace access to the rest of the workspace.</strong><br>
	One dependency when you want one import surface. Focused crates stay available when you want the narrowest build or a stable future-facing crate boundary.
</p>

<p align="center">
	<img alt="Rust 1.95.0+" src="https://img.shields.io/badge/Rust-1.95.0%2B-f46623?logo=rust&logoColor=white">
	<img alt="Edition 2024" src="https://img.shields.io/badge/edition-2024-0f766e">
	<img alt="Default feature full" src="https://img.shields.io/badge/default-full-1d4ed8">
	<img alt="Features 17 optional modules" src="https://img.shields.io/badge/features-17%20optional%20modules-c2410c">
	<img alt="License MIT or Apache-2.0" src="https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-2a9d8f">
</p>

<p align="center">
	<a href="#what-this-crate-provides">Surface</a> ·
	<a href="#when-to-choose-the-facade">When to use it</a> ·
	<a href="#installation">Installation</a> ·
	<a href="#quick-examples">Examples</a> ·
	<a href="#feature-model">Features</a> ·
	<a href="#design-constraints">Constraints</a>
</p>

`use-math` composes the focused `RustUse` math crates into one entry point while keeping their APIs direct and explicit. It re-exports the currently supported geometry, combinatorics, complex-number, numerical-calculus, probability, real-number, and rational-number surfaces at the crate root, exposes nested modules for every focused crate in the workspace, and keeps the shared `prelude` limited to the items that already have concrete ergonomic value.

<table>
	<tr>
		<td width="33%" valign="top">
			<strong>Root re-exports</strong><br>
			Call functions like <code>factorial</code> or types like <code>Point2</code>, <code>Complex</code>, <code>Differentiator</code>, <code>Probability</code>, <code>Real</code>, and <code>Rational</code> directly from <code>use_math</code>.
		</td>
		<td width="33%" valign="top">
			<strong>Nested modules</strong><br>
			Use <code>use_math::geometry</code>, <code>use_math::combinatorics</code>, or any scaffolded namespace like <code>use_math::number</code> when you want crate-shaped namespacing.
		</td>
		<td width="33%" valign="top">
			<strong>Shared prelude</strong><br>
			Pull common items from <code>use_math::prelude</code> when fast onboarding matters more than fully qualified imports.
		</td>
	</tr>
</table>

## What this crate provides

| Entry point                  | What it exposes                                                                    | Best fit                                                                  |
| ---------------------------- | ---------------------------------------------------------------------------------- | ------------------------------------------------------------------------- |
| Root re-exports              | Direct access to enabled geometry, combinatorics, complex-number, numerical-calculus, probability, real-number, and rational-number items | Call sites that want short imports                                        |
| `use_math::geometry`         | The `use-geometry` crate as a nested module                                        | Code that prefers explicit geometry namespacing                           |
| `use_math::combinatorics`    | The `use-combinatorics` crate as a nested module                                   | Code that prefers explicit combinatorics namespacing                      |
| Scaffolded namespace modules | Focused crates such as `use_math::number`, `use_math::algebra`, or `use_math::set` | Stable crate-shaped namespacing before those focused APIs are implemented |
| `use_math::prelude`          | Common items from enabled concrete features                                        | Small apps, examples, and quick starts                                    |

| If you need to...                                           | Start here                                              |
| ----------------------------------------------------------- | ------------------------------------------------------- |
| Add one dependency and opt into math surfaces with features | `use-math`                                              |
| Keep geometry-only code isolated                            | `use-geometry` directly                                 |
| Keep counting-only code isolated                            | `use-combinatorics` directly                            |
| Keep complex-number primitives isolated                     | `use-complex` directly                                  |
| Keep numerical-calculus helpers isolated                    | `use-calculus` directly                                 |
| Keep explicit probability primitives isolated               | `use-probability` directly                              |
| Keep finite-value and interval helpers isolated             | `use-real` directly                                     |
| Keep exact rational arithmetic isolated                     | `use-rational` directly                                 |
| Depend on a future-focused crate boundary early             | The nested namespace or focused scaffold crate directly |
| Minimize both dependency weight and API width               | The focused crate directly                              |

## When to choose the facade

Use the facade when consumer ergonomics matter more than squeezing the dependency graph to the smallest possible shape.

| Scenario                                                               | Choose `use-math`? | Why                                                            |
| ---------------------------------------------------------------------- | ------------------ | -------------------------------------------------------------- |
| You want one dependency for geometry, counting, complex primitives, numerical calculus, probability, real-number helpers, and rational arithmetic | Yes | The facade keeps imports unified behind features               |
| You are building a small app or example project                        | Yes                | Root re-exports and the `prelude` reduce setup friction        |
| You want namespace access to scaffolded future crate boundaries        | Usually yes        | The facade exposes every focused crate name consistently today |
| You only need geometry                                                 | Usually no         | `use-geometry` stays narrower and more explicit                |
| You only need combinatorics                                            | Usually no         | `use-combinatorics` avoids unrelated modules                   |
| You only need numerical calculus                                       | Usually no         | `use-calculus` keeps the approximation policy local and direct |
| You only need probability primitives                                   | Usually no         | `use-probability` keeps event assumptions local and direct     |
| You only need finite-value or interval helpers                         | Usually no         | `use-real` keeps floating-point validation and tolerance policy local |
| You only need exact rational arithmetic                                | Usually no         | `use-rational` keeps exact fraction normalization and arithmetic local |

> [!TIP]
> The facade is intentionally thin. It is not a second abstraction layer over the focused crates.

## Installation

Default features enable the current full surface:

```toml
[dependencies]
use-math = "0.0.1"
```

Geometry only:

```toml
[dependencies]
use-math = { version = "0.0.1", default-features = false, features = ["geometry"] }
```

Combinatorics only:

```toml
[dependencies]
use-math = { version = "0.0.1", default-features = false, features = ["combinatorics"] }
```

## Quick examples

### Checked counting from the root

```rust
# #[cfg(feature = "combinatorics")]
# fn main() -> Result<(), use_math::CombinatoricsError> {
use use_math::{combinations, factorial, permutations};

assert_eq!(factorial(5)?, 120);
assert_eq!(permutations(5, 3)?, 60);
assert_eq!(combinations(5, 2)?, 10);
# Ok::<(), use_math::CombinatoricsError>(())
# }
#
# #[cfg(not(feature = "combinatorics"))]
# fn main() {}
```

### Geometry from the root

```rust
# #[cfg(feature = "geometry")]
# fn main() -> Result<(), use_math::GeometryError> {
use use_math::{Orientation2, Point2, Triangle, distance_2d, midpoint_2d, try_orientation_2d};

let a = Point2::try_new(0.0, 0.0)?;
let b = Point2::try_new(4.0, 0.0)?;
let c = Point2::try_new(0.0, 3.0)?;
let triangle = Triangle::try_new(a, b, c)?;

assert_eq!(distance_2d(a, b), 4.0);
assert_eq!(midpoint_2d(a, c), Point2::try_new(0.0, 1.5)?);
assert_eq!(try_orientation_2d(a, b, c)?, Orientation2::CounterClockwise);
assert_eq!(triangle.area(), 6.0);
# Ok::<(), use_math::GeometryError>(())
# }
#
# #[cfg(not(feature = "geometry"))]
# fn main() {}
```

### Geometry extras behind the feature gate

```rust
# #[cfg(feature = "geometry")]
# fn main() -> Result<(), use_math::GeometryError> {
use use_math::{Aabb2, Orientation2, Point2, orientation_2d_with_tolerance};

let a = Point2::try_new(0.0, 0.0)?;
let b = Point2::try_new(4.0, 0.0)?;
let c = Point2::try_new(0.0, 3.0)?;
let bounds = Aabb2::from_points(a, c);

assert!(bounds.contains_point(Point2::new(0.0, 1.5)));
assert_eq!(orientation_2d_with_tolerance(a, b, c, 0.0)?, Orientation2::CounterClockwise);
# Ok::<(), use_math::GeometryError>(())
# }
#
# #[cfg(not(feature = "geometry"))]
# fn main() {}
```

### Numerical calculus from the root

```rust
# #[cfg(feature = "calculus")]
# fn main() -> Result<(), use_math::CalculusError> {
use use_math::{Differentiator, IntegrationInterval, Integrator, LimitApproximator};

let differentiator = Differentiator::try_new(1.0e-5)?;
let interval = IntegrationInterval::try_new(0.0, 1.0)?;
let integrator = Integrator::try_new(128)?;
let limit = LimitApproximator::try_new(1.0e-6, 1.0e-5)?;

let slope = differentiator.derivative_at(|x| x.powi(2), 3.0)?;
let area = integrator.simpson(|x| x * x, interval)?;
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

assert!((slope - 6.0).abs() < 1.0e-6);
assert!((area - (1.0 / 3.0)).abs() < 1.0e-6);
assert!((sinc_limit - 1.0).abs() < 1.0e-5);
# Ok::<(), use_math::CalculusError>(())
# }
#
# #[cfg(not(feature = "calculus"))]
# fn main() {}
```

### Probability from the root

```rust
# #[cfg(feature = "probability")]
# fn main() -> Result<(), use_math::ProbabilityError> {
use use_math::{Bernoulli, Probability, independent_intersection, independent_union};

let rain = Probability::from_fraction(1, 4)?;
let traffic = Probability::try_new(0.5)?;
let commute = Bernoulli::new(rain);

assert!((independent_intersection(rain, traffic).value() - 0.125).abs() < 1.0e-12);
assert!((independent_union(rain, traffic).value() - 0.625).abs() < 1.0e-12);
assert_eq!(commute.failure_probability(), Probability::try_new(0.75)?);
# Ok::<(), use_math::ProbabilityError>(())
# }
#
# #[cfg(not(feature = "probability"))]
# fn main() {}
```

### Real-number helpers from the root

```rust
# #[cfg(feature = "real")]
# fn main() -> Result<(), use_math::RealError> {
use use_math::{Real, RealInterval, approx_eq};

let interval = RealInterval::try_new(-2.0, 6.0)?;
let midpoint = interval.midpoint();
let clamped = interval.clamp(Real::try_new(8.0)?);

assert_eq!(clamped, Real::try_new(6.0)?);
assert!(approx_eq(midpoint, Real::try_new(2.0)?, 1.0e-12)?);
# Ok::<(), use_math::RealError>(())
# }
#
# #[cfg(not(feature = "real"))]
# fn main() {}
```

### Rational arithmetic from the root

```rust
# #[cfg(feature = "rational")]
# fn main() -> Result<(), use_math::RationalError> {
use use_math::Rational;

let half = Rational::try_new(1, 2)?;
let third = Rational::try_new(1, 3)?;

assert_eq!(half.checked_add(third)?, Rational::try_new(5, 6)?);
assert_eq!(half.checked_div(third)?, Rational::try_new(3, 2)?);
# Ok::<(), use_math::RationalError>(())
# }
#
# #[cfg(not(feature = "rational"))]
# fn main() {}
```

## Feature model

| Feature                                                                                                                                                    | Enables                                                                                   | Default |
| ---------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------- | ------- |
| `geometry`                                                                                                                                                 | Re-exports from `use-geometry`, including `Aabb2` and tolerance-aware orientation helpers | No      |
| `combinatorics`                                                                                                                                            | Re-exports from `use-combinatorics`                                                       | No      |
| `complex`                                                                                                                                                  | Re-exports from `use-complex`, including `Complex` and `Imaginary`                        | No      |
| `calculus`                                                                                                                                                 | Re-exports from `use-calculus`, including `Differentiator`, `Integrator`, and limit helpers | No   |
| `probability`                                                                                                                                              | Re-exports from `use-probability`, including `Probability`, `Bernoulli`, and independent-event helpers | No |
| `rational`                                                                                                                                                 | Re-exports from `use-rational`, including `Rational` and `RationalError`                   | No      |
| `real`                                                                                                                                                     | Re-exports from `use-real`, including `Real`, `RealInterval`, and `approx_eq`            | No      |
| `number`, `integer`, `series`, `catalan`, `algebra`, `linear`, `statistics`, `trigonometry`, `logic`, `set` | The corresponding focused crate as a nested namespace module only                         | No      |
| `full`                                                                                                                                                     | Every focused crate feature in the workspace                                              | Yes     |

> [!NOTE]
> `full` is the default today because the facade exists to smooth over multi-crate integration. Disable defaults when you need tighter control over compile surface.
> The scaffold-only features currently expose only nested namespace modules, not root-level item re-exports or additional `prelude` items.

## Design constraints

- The facade stays close to the focused crates instead of inventing a separate object model.
- Small APIs are preferred over broad trait-heavy abstractions.
- Depend on the focused crates directly when the facade would be wider than you need.
- Facade-only wrapper types, macros, and a second abstraction layer are intentionally out of scope.

## Status

`use-math` is a scaffolded public facade crate in the `RustUse` docs surface. The API remains pre-1.0, and the facade intentionally distinguishes between concrete root-level APIs and namespace-only scaffold features while the rest of the workspace grows.
