# use-math

<p align="center">
	<strong>Feature-gated <code>RustUse</code> facade for concrete geometry, checked counting, Catalan-family sequences, progression helpers, integer helpers, boolean algebra helpers, small set helpers, explicit trigonometry helpers, descriptive statistics helpers, compact linear algebra helpers, finite algebra law helpers, raw-number helpers, complex numbers, numerical calculus, probability, real-number primitives, and rational arithmetic.</strong><br>
	One dependency when you want one import surface. Focused crates stay available when you want the narrowest build or an explicit direct crate for one math domain.
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

`use-math` composes the focused `RustUse` math crates into one entry point while keeping their APIs direct and explicit. It re-exports the currently supported geometry, combinatorics, Catalan-family, progression, integer-helper, boolean-algebra, set-helper, trigonometry, descriptive statistics, compact linear algebra, finite algebra law, raw-number, complex-number, numerical-calculus, probability, real-number, and rational-number surfaces at the crate root, exposes nested modules for every focused crate in the workspace, and keeps the shared `prelude` limited to the items that already have concrete ergonomic value.

<table>
	<tr>
		<td width="33%" valign="top">
			<strong>Root re-exports</strong><br>
			Call functions like <code>factorial</code>, <code>catalan</code>, <code>arithmetic_sum</code>, <code>classify_number</code>, <code>gcd</code>, <code>identity_element</code>, <code>implication</code>, <code>is_ring</code>, <code>set_union</code>, <code>sin_deg</code>, <code>mean</code>, <code>solve_2x2</code>, or types like <code>Point2</code>, <code>IntegerSign</code>, <code>NumberCategory</code>, <code>Angle</code>, <code>LinearVector2</code>, <code>Matrix2</code>, <code>Complex</code>, <code>Differentiator</code>, <code>Probability</code>, <code>Real</code>, and <code>Rational</code> directly from <code>use_math</code>.
		</td>
		<td width="33%" valign="top">
			<strong>Nested modules</strong><br>
			Use <code>use_math::geometry</code>, <code>use_math::combinatorics</code>, <code>use_math::number</code>, or <code>use_math::algebra</code> when you want crate-shaped namespacing.
		</td>
		<td width="33%" valign="top">
			<strong>Shared prelude</strong><br>
			Pull common items from <code>use_math::prelude</code> when fast onboarding matters more than fully qualified imports.
		</td>
	</tr>
</table>

## What this crate provides

| Entry point               | What it exposes                                                                                                                                                                                                                                                                                                   | Best fit                                             |
| ------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------- |
| Root re-exports           | Direct access to enabled geometry, combinatorics, Catalan-family, progression, integer-helper, boolean-algebra, set-helper, trigonometry, descriptive statistics, compact linear algebra, finite algebra law, raw-number, complex-number, numerical-calculus, probability, real-number, and rational-number items | Call sites that want short imports                   |
| `use_math::geometry`      | The `use-geometry` crate as a nested module                                                                                                                                                                                                                                                                       | Code that prefers explicit geometry namespacing      |
| `use_math::combinatorics` | The `use-combinatorics` crate as a nested module                                                                                                                                                                                                                                                                  | Code that prefers explicit combinatorics namespacing |
| `use_math::algebra`       | The `use-algebra` crate as a nested module                                                                                                                                                                                                                                                                        | Code that prefers explicit algebra namespacing       |
| `use_math::prelude`       | Common items from enabled concrete features                                                                                                                                                                                                                                                                       | Small apps, examples, and quick starts               |

| If you need to...                                           | Start here                   |
| ----------------------------------------------------------- | ---------------------------- |
| Add one dependency and opt into math surfaces with features | `use-math`                   |
| Keep geometry-only code isolated                            | `use-geometry` directly      |
| Keep counting-only code isolated                            | `use-combinatorics` directly |
| Keep Catalan-family sequence helpers isolated               | `use-catalan` directly       |
| Keep progression helpers isolated                           | `use-series` directly        |
| Keep finite algebra law helpers isolated                    | `use-algebra` directly       |
| Keep integer-helper code isolated                           | `use-integer` directly       |
| Keep boolean algebra helpers isolated                       | `use-logic` directly         |
| Keep set helpers isolated                                   | `use-set` directly           |
| Keep trigonometry helpers isolated                          | `use-trigonometry` directly  |
| Keep descriptive statistics helpers isolated                | `use-statistics` directly    |
| Keep compact linear-algebra helpers isolated                | `use-linear` directly        |
| Keep raw-number helpers isolated                            | `use-number` directly        |
| Keep complex-number primitives isolated                     | `use-complex` directly       |
| Keep numerical-calculus helpers isolated                    | `use-calculus` directly      |
| Keep explicit probability primitives isolated               | `use-probability` directly   |
| Keep finite-value and interval helpers isolated             | `use-real` directly          |
| Keep exact rational arithmetic isolated                     | `use-rational` directly      |
| Minimize both dependency weight and API width               | The focused crate directly   |

## When to choose the facade

Use the facade when consumer ergonomics matter more than squeezing the dependency graph to the smallest possible shape.

| Scenario                                                                                                                                                                                                                                                                                                                                                          | Choose `use-math`? | Why                                                                                    |
| ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------ | -------------------------------------------------------------------------------------- |
| You want one dependency for geometry, counting, Catalan-family sequences, progression helpers, integer helpers, boolean algebra helpers, set helpers, trigonometry helpers, descriptive statistics helpers, compact linear-algebra helpers, raw-number helpers, complex primitives, numerical calculus, probability, real-number helpers, and rational arithmetic | Yes                | The facade keeps imports unified behind features                                       |
| You are building a small app or example project                                                                                                                                                                                                                                                                                                                   | Yes                | Root re-exports and the `prelude` reduce setup friction                                |
| You want namespace access to every focused crate                                                                                                                                                                                                                                                                                                                  | Usually yes        | The facade exposes every focused crate name consistently today                         |
| You only need geometry                                                                                                                                                                                                                                                                                                                                            | Usually no         | `use-geometry` stays narrower and more explicit                                        |
| You only need combinatorics                                                                                                                                                                                                                                                                                                                                       | Usually no         | `use-combinatorics` avoids unrelated modules                                           |
| You only need Catalan-family sequence helpers                                                                                                                                                                                                                                                                                                                     | Usually no         | `use-catalan` keeps that counting surface narrow and explicit                          |
| You only need progression helpers                                                                                                                                                                                                                                                                                                                                 | Usually no         | `use-series` keeps nth-term and partial-sum helpers narrow and explicit                |
| You only need finite algebra law helpers                                                                                                                                                                                                                                                                                                                          | Usually no         | `use-algebra` keeps closure-based structure checks explicit and local                  |
| You only need integer helpers                                                                                                                                                                                                                                                                                                                                     | Usually no         | `use-integer` keeps parity, divisibility, and gcd/lcm logic local                      |
| You only need boolean algebra helpers                                                                                                                                                                                                                                                                                                                             | Usually no         | `use-logic` keeps named truth-table helpers explicit and local                         |
| You only need set helpers                                                                                                                                                                                                                                                                                                                                         | Usually no         | `use-set` keeps membership and set-operation intent explicit and local                 |
| You only need trigonometry helpers                                                                                                                                                                                                                                                                                                                                | Usually no         | `use-trigonometry` keeps degree/radian handling and trig evaluation explicit and local |
| You only need descriptive statistics helpers                                                                                                                                                                                                                                                                                                                      | Usually no         | `use-statistics` keeps mean, median, and variance summaries explicit and local         |
| You only need compact linear-algebra helpers                                                                                                                                                                                                                                                                                                                      | Usually no         | `use-linear` keeps small vectors, matrices, and singular solves explicit and local     |
| You only need raw-number helpers                                                                                                                                                                                                                                                                                                                                  | Usually no         | `use-number` keeps plain `f64` classification and shared constants explicit and local  |
| You only need numerical calculus                                                                                                                                                                                                                                                                                                                                  | Usually no         | `use-calculus` keeps the approximation policy local and direct                         |
| You only need probability primitives                                                                                                                                                                                                                                                                                                                              | Usually no         | `use-probability` keeps event assumptions local and direct                             |
| You only need finite-value or interval helpers                                                                                                                                                                                                                                                                                                                    | Usually no         | `use-real` keeps floating-point validation and tolerance policy local                  |
| You only need exact rational arithmetic                                                                                                                                                                                                                                                                                                                           | Usually no         | `use-rational` keeps exact fraction normalization and arithmetic local                 |

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

### Catalan-family counting from the root

```rust
# #[cfg(feature = "catalan")]
# fn main() -> Result<(), use_math::CatalanError> {
use use_math::{catalan, fuss_catalan};

assert_eq!(catalan(4)?, 14);
assert_eq!(fuss_catalan(3, 3)?, 12);
# Ok::<(), use_math::CatalanError>(())
# }
#
# #[cfg(not(feature = "catalan"))]
# fn main() {}
```

### Progression helpers from the root

```rust
# #[cfg(feature = "series")]
# fn main() -> Result<(), use_math::SeriesError> {
use use_math::{arithmetic_nth_term, arithmetic_sum, geometric_nth_term, geometric_sum};

assert_eq!(arithmetic_nth_term(3, 2, 4)?, 11);
assert_eq!(arithmetic_sum(3, 2, 5)?, 35);
assert_eq!(geometric_nth_term(2, 3, 4)?, 162);
assert_eq!(geometric_sum(2, 3, 4)?, 80);
# Ok::<(), use_math::SeriesError>(())
# }
#
# #[cfg(not(feature = "series"))]
# fn main() {}
```

### Boolean algebra helpers from the root

```rust
# #[cfg(feature = "logic")]
# fn main() {
use use_math::{equivalence, exclusive_or, implication, majority, nand, nor};

assert!(implication(false, true));
assert!(equivalence(true, true));
assert!(exclusive_or(true, false));
assert!(!nand(true, true));
assert!(nor(false, false));
assert!(majority(true, true, false));
# }
#
# #[cfg(not(feature = "logic"))]
# fn main() {}
```

### Set helpers from the root

```rust
# #[cfg(feature = "set")]
# fn main() {
use use_math::{
	are_disjoint, contains_member, is_subset, set_difference, set_intersection,
	set_symmetric_difference, set_union,
};

let left = [1, 2, 2, 3];
let right = [3, 4, 2, 5];

assert!(contains_member(&left, &1));
assert!(is_subset(&[2, 3], &right));
assert!(!are_disjoint(&left, &right));
assert_eq!(set_union(&left, &right), vec![1, 2, 3, 4, 5]);
assert_eq!(set_intersection(&left, &right), vec![2, 3]);
assert_eq!(set_difference(&left, &right), vec![1]);
assert_eq!(set_symmetric_difference(&left, &right), vec![1, 4, 5]);
# }
#
# #[cfg(not(feature = "set"))]
# fn main() {}
```

### Trigonometry helpers from the root

```rust
# #[cfg(feature = "trigonometry")]
# fn main() {
use use_math::{Angle, cos_deg, degrees_to_radians, normalize_degrees, radians_to_degrees, sin_deg, tan_deg};

let acute = Angle::from_degrees(30.0);
let wrapped = Angle::from_degrees(765.0).normalized();

assert!((acute.radians() - degrees_to_radians(30.0)).abs() < 1.0e-12);
assert!((acute.degrees() - radians_to_degrees(acute.radians())).abs() < 1.0e-12);
assert!((wrapped.degrees() - 45.0).abs() < 1.0e-12);
assert!((normalize_degrees(-90.0) - 270.0).abs() < 1.0e-12);
assert!((acute.sin() - 0.5).abs() < 1.0e-12);
assert!((cos_deg(60.0) - 0.5).abs() < 1.0e-12);
assert!((sin_deg(30.0) - 0.5).abs() < 1.0e-12);
assert!((tan_deg(45.0) - 1.0).abs() < 1.0e-12);
# }
#
# #[cfg(not(feature = "trigonometry"))]
# fn main() {}
```

### Descriptive statistics from the root

```rust
# #[cfg(feature = "statistics")]
# fn main() -> Result<(), use_math::StatisticsError> {
use use_math::{mean, median, population_std_dev, population_variance, sample_std_dev, sample_variance};

let values = [2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
let sample = [1.0, 2.0, 3.0, 4.0];

assert!((mean(&values)? - 5.0).abs() < 1.0e-12);
assert!((median(&values)? - 4.5).abs() < 1.0e-12);
assert!((population_variance(&values)? - 4.0).abs() < 1.0e-12);
assert!((population_std_dev(&values)? - 2.0).abs() < 1.0e-12);
assert!((sample_variance(&sample)? - 1.666_666_666_666_666_7).abs() < 1.0e-12);
assert!((sample_std_dev(&sample)? - 1.290_994_448_735_805_6).abs() < 1.0e-12);
# Ok::<(), use_math::StatisticsError>(())
# }
#
# #[cfg(not(feature = "statistics"))]
# fn main() {}
```

### Linear algebra helpers from the root

```rust
# #[cfg(feature = "linear")]
# fn main() -> Result<(), use_math::LinearError> {
use use_math::{LinearVector2, Matrix2, dot, solve_2x2};

let vector = LinearVector2::new(3.0, 4.0);
let other = LinearVector2::new(-2.0, 1.0);
let matrix = Matrix2::new(2.0, 1.0, 5.0, 3.0);

assert_eq!(vector + other, LinearVector2::new(1.0, 5.0));
assert!((dot(vector, other) + 2.0).abs() < 1.0e-12);
assert!((vector.magnitude() - 5.0).abs() < 1.0e-12);
assert_eq!(matrix.mul_vector(LinearVector2::new(1.0, -1.0)), LinearVector2::new(1.0, 2.0));
assert_eq!(solve_2x2(matrix, LinearVector2::new(1.0, 2.0))?, LinearVector2::new(1.0, -1.0));
# Ok::<(), use_math::LinearError>(())
# }
#
# #[cfg(not(feature = "linear"))]
# fn main() {}
```

### Raw-number helpers from the root

```rust
# #[cfg(feature = "number")]
# fn main() {
use use_math::{
	GOLDEN_RATIO, NumberCategory, NumberSign, SQRT_3, classify_number,
	classify_number_sign, is_finite_number,
};

assert_eq!(classify_number(f64::NAN), NumberCategory::Nan);
assert_eq!(classify_number(f64::from_bits(1)), NumberCategory::Subnormal);
assert_eq!(classify_number_sign(-12.5), Some(NumberSign::Negative));
assert_eq!(classify_number_sign(f64::NAN), None);
assert!(is_finite_number(3.5));
assert!(!is_finite_number(f64::INFINITY));
assert!(((GOLDEN_RATIO * GOLDEN_RATIO) - (GOLDEN_RATIO + 1.0)).abs() < 1.0e-12);
assert!(((SQRT_3 * SQRT_3) - 3.0).abs() < 1.0e-12);
# }
#
# #[cfg(not(feature = "number"))]
# fn main() {}
```

### Finite algebra law helpers from the root

```rust
# #[cfg(feature = "algebra")]
# fn main() {
use use_math::{identity_element, is_abelian_group, is_distributive_over, is_ring};

let residues = [0_u8, 1, 2];
let add_mod_3 = |left, right| (left + right) % 3;
let mul_mod_3 = |left, right| (left * right) % 3;

assert_eq!(identity_element(&residues, add_mod_3), Some(0));
assert!(is_abelian_group(&residues, add_mod_3));
assert!(is_distributive_over(&residues, mul_mod_3, add_mod_3));
assert!(is_ring(&residues, add_mod_3, mul_mod_3));
# }
#
# #[cfg(not(feature = "algebra"))]
# fn main() {}
```

### Integer helpers from the root

```rust
# #[cfg(feature = "integer")]
# fn main() -> Result<(), use_math::IntegerError> {
use use_math::{IntegerSign, classify_sign, gcd, is_divisible_by, lcm};

assert_eq!(classify_sign(-12), IntegerSign::Negative);
assert!(is_divisible_by(84, 7)?);
assert_eq!(gcd(-54, 24), 6);
assert_eq!(lcm(-6, 15)?, 30);
# Ok::<(), use_math::IntegerError>(())
# }
#
# #[cfg(not(feature = "integer"))]
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

| Feature         | Enables                                                                                                                                           | Default |
| --------------- | ------------------------------------------------------------------------------------------------------------------------------------------------- | ------- |
| `geometry`      | Re-exports from `use-geometry`, including `Aabb2` and tolerance-aware orientation helpers                                                         | No      |
| `combinatorics` | Re-exports from `use-combinatorics`                                                                                                               | No      |
| `catalan`       | Re-exports from `use-catalan`, including `catalan`, `fuss_catalan`, and `CatalanError`                                                            | No      |
| `algebra`       | Re-exports from `use-algebra`, including finite algebra-law helpers such as `identity_element`, `is_abelian_group`, and `is_ring`                 | No      |
| `series`        | Re-exports from `use-series`, including arithmetic and geometric progression helpers                                                              | No      |
| `integer`       | Re-exports from `use-integer`, including `IntegerSign`, divisibility helpers, and gcd/lcm                                                         | No      |
| `logic`         | Re-exports from `use-logic`, including implication, equivalence, XOR, NAND, NOR, and majority helpers                                             | No      |
| `set`           | Re-exports from `use-set`, including membership predicates and order-preserving set operations                                                    | No      |
| `trigonometry`  | Re-exports from `use-trigonometry`, including `Angle`, degree/radian conversion helpers, normalization helpers, and `sin_deg`/`cos_deg`/`tan_deg` | No      |
| `statistics`    | Re-exports from `use-statistics`, including `StatisticsError`, mean/median, variance, and standard-deviation helpers                              | No      |
| `linear`        | Re-exports from `use-linear`, including `LinearVector2`, `Matrix2`, `dot`, `solve_2x2`, and `LinearError`                                         | No      |
| `number`        | Re-exports from `use-number`, including floating-point classification helpers and shared numeric constants                                        | No      |
| `complex`       | Re-exports from `use-complex`, including `Complex` and `Imaginary`                                                                                | No      |
| `calculus`      | Re-exports from `use-calculus`, including `Differentiator`, `Integrator`, and limit helpers                                                       | No      |
| `probability`   | Re-exports from `use-probability`, including `Probability`, `Bernoulli`, and independent-event helpers                                            | No      |
| `rational`      | Re-exports from `use-rational`, including `Rational` and `RationalError`                                                                          | No      |
| `real`          | Re-exports from `use-real`, including `Real`, `RealInterval`, and `approx_eq`                                                                     | No      |
| `full`          | Every focused crate feature in the workspace                                                                                                      | Yes     |

> [!NOTE]
> `full` is the default today because the facade exists to smooth over multi-crate integration. Disable defaults when you need tighter control over compile surface.
> Every focused crate feature now exposes both nested modules and concrete root-level re-exports.

## Design constraints

- The facade stays close to the focused crates instead of inventing a separate object model.
- Small APIs are preferred over broad trait-heavy abstractions.
- Depend on the focused crates directly when the facade would be wider than you need.
- Facade-only wrapper types, macros, and a second abstraction layer are intentionally out of scope.

## Status

`use-math` is a concrete pre-1.0 facade crate in the `RustUse` docs surface. The API remains intentionally thin while every focused crate in the workspace now exposes a concrete public surface.
