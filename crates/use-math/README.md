# use-math

<p align="center">
	<strong>Feature-gated <code>RustUse</code> facade for geometry and checked counting.</strong><br>
	One dependency when you want one import surface. Focused crates stay available when you want the narrowest build.
</p>

<p align="center">
	<img alt="Rust 1.95.0+" src="https://img.shields.io/badge/Rust-1.95.0%2B-f46623?logo=rust&logoColor=white">
	<img alt="Edition 2024" src="https://img.shields.io/badge/edition-2024-0f766e">
	<img alt="Default feature full" src="https://img.shields.io/badge/default-full-1d4ed8">
	<img alt="Features geometry combinatorics" src="https://img.shields.io/badge/features-geometry%20%7C%20combinatorics-c2410c">
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

`use-math` composes the focused `RustUse` math crates into one entry point while keeping their APIs direct and explicit. It re-exports the currently supported geometry and combinatorics surfaces at the crate root, exposes nested modules when crate-scoped access reads better, and provides a shared `prelude` for quick integration.

<table>
	<tr>
		<td width="33%" valign="top">
			<strong>Root re-exports</strong><br>
			Call functions like <code>factorial</code> or types like <code>Point2</code> directly from <code>use_math</code>.
		</td>
		<td width="33%" valign="top">
			<strong>Nested modules</strong><br>
			Use <code>use_math::geometry</code> or <code>use_math::combinatorics</code> when you want crate-shaped namespacing.
		</td>
		<td width="33%" valign="top">
			<strong>Shared prelude</strong><br>
			Pull common items from <code>use_math::prelude</code> when fast onboarding matters more than fully qualified imports.
		</td>
	</tr>
</table>

## What this crate provides

| Entry point               | What it exposes                                           | Best fit                                             |
| ------------------------- | --------------------------------------------------------- | ---------------------------------------------------- |
| Root re-exports           | Direct access to enabled geometry and combinatorics items | Call sites that want short imports                   |
| `use_math::geometry`      | The `use-geometry` crate as a nested module               | Code that prefers explicit geometry namespacing      |
| `use_math::combinatorics` | The `use-combinatorics` crate as a nested module          | Code that prefers explicit combinatorics namespacing |
| `use_math::prelude`       | Common items from enabled features                        | Small apps, examples, and quick starts               |

| If you need to...                                           | Start here                   |
| ----------------------------------------------------------- | ---------------------------- |
| Add one dependency and opt into math surfaces with features | `use-math`                   |
| Keep geometry-only code isolated                            | `use-geometry` directly      |
| Keep counting-only code isolated                            | `use-combinatorics` directly |
| Minimize both dependency weight and API width               | The focused crate directly   |

## When to choose the facade

Use the facade when consumer ergonomics matter more than squeezing the dependency graph to the smallest possible shape.

| Scenario                                               | Choose `use-math`? | Why                                                     |
| ------------------------------------------------------ | ------------------ | ------------------------------------------------------- |
| You want one dependency for both geometry and counting | Yes                | The facade keeps imports unified behind features        |
| You are building a small app or example project        | Yes                | Root re-exports and the `prelude` reduce setup friction |
| You only need geometry                                 | Usually no         | `use-geometry` stays narrower and more explicit         |
| You only need combinatorics                            | Usually no         | `use-combinatorics` avoids unrelated modules            |

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

## Feature model

| Feature         | Enables                                                                                   | Default |
| --------------- | ----------------------------------------------------------------------------------------- | ------- |
| `geometry`      | Re-exports from `use-geometry`, including `Aabb2` and tolerance-aware orientation helpers | No      |
| `combinatorics` | Re-exports from `use-combinatorics`                                                       | No      |
| `full`          | `geometry` and `combinatorics` together                                                   | Yes     |

> [!NOTE]
> `full` is the default today because the facade exists to smooth over multi-crate integration. Disable defaults when you need tighter control over compile surface.

## Design constraints

- The facade stays close to the focused crates instead of inventing a separate object model.
- Small APIs are preferred over broad trait-heavy abstractions.
- Depend on the focused crates directly when the facade would be wider than you need.
- Facade-only wrapper types, macros, and a second abstraction layer are intentionally out of scope.

## Status

`use-math` is a scaffolded public facade crate in the `RustUse` docs surface. The API remains pre-1.0, and the `RustUse`-hosted generated rustdocs stay canonical while external crates.io and docs.rs pages remain staged.
