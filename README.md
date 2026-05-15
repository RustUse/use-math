# RustUse/use-math

<p align="center">
	<img alt="Rust 1.95.0+" src="https://img.shields.io/badge/Rust-1.95.0%2B-f46623?logo=rust&logoColor=white">
	<img alt="Edition 2024" src="https://img.shields.io/badge/edition-2024-0f766e">
	<img alt="23 workspace crates" src="https://img.shields.io/badge/workspace-23%20crates-1d4ed8">
	<img alt="Status pre-release" src="https://img.shields.io/badge/status-pre--release-c2410c">
	<img alt="License MIT or Apache-2.0" src="https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-2a9d8f">
</p>

<p align="center">
	<strong>Utility-first Rust math crates with concrete focused APIs across the entire workspace.</strong><br>
	Focused crates stay small. The facade crate composes them behind opt-in features and exposes matching nested modules for explicit crate-shaped namespacing.
</p>

<p align="center">
	<a href="#what-this-workspace-ships">Workspace</a> ·
	<a href="#choose-your-entry-point">Choose a crate</a> ·
	<a href="#current-status">Status</a> ·
	<a href="#project-structure">Structure</a> ·
	<a href="#installation">Installation</a> ·
	<a href="#quick-examples">Examples</a> ·
	<a href="#feature-model">Features</a> ·
	<a href="#development">Development</a> ·
	<a href="#community-and-project-policy">Community</a>
</p>

This repository is the source workspace for RustUse's math surface. Today it combines concrete focused crates for arithmetic helpers, vector primitives, matrix primitives, interval primitives, geometry, checked combinatorics, Catalan-family sequences, Geode-array primitives, progression helpers, finite algebra law helpers, integer helpers, boolean algebra helpers, small set helpers, explicit trigonometry helpers, descriptive statistics helpers, solver-style linear helpers, raw-number helpers, complex numbers, numerical calculus, probability, real-number helpers, and rational arithmetic. `use-math` composes the whole workspace behind feature flags while keeping root re-exports aligned with the concrete APIs each focused crate exposes.

## Current status

- The GitHub repository is public before the first crates.io release is live.
- Until then, consume the crates from a pinned Git revision or work from the workspace directly.
- `use-arithmetic`, `use-vector`, `use-matrix`, `use-geometry`, `use-combinatorics`, `use-catalan`, `use-geode`, `use-series`, `use-algebra`, `use-integer`, `use-logic`, `use-set`, `use-trigonometry`, `use-statistics`, `use-linear`, `use-number`, `use-complex`, `use-calculus`, `use-probability`, `use-real`, `use-interval`, and `use-rational` are the concrete focused APIs today.
- The planned first release order is every focused crate first in dependency order, including `use-interval` before `use-real`, `use-vector` before `use-matrix`, `use-matrix` before `use-linear`, and `use-math` after crates.io index propagation.

<table>
	<tr>
		<td width="33%" valign="top">
			<strong>Pull in one facade</strong><br>
			<code>crates/use-math/</code><br>
			Reach for the shared <code>prelude</code>, root re-exports for concrete APIs, and namespace modules for explicit crate-shaped namespacing.
		</td>
		<td width="33%" valign="top">
			<strong>Use concrete focused crates</strong><br>
			<code>crates/use-arithmetic/</code>, <code>crates/use-vector/</code>, <code>crates/use-matrix/</code>, <code>crates/use-geometry/</code>, <code>crates/use-combinatorics/</code>, <code>crates/use-catalan/</code>, <code>crates/use-geode/</code>, <code>crates/use-series/</code>, <code>crates/use-algebra/</code>, <code>crates/use-integer/</code>, <code>crates/use-logic/</code>, <code>crates/use-set/</code>, <code>crates/use-trigonometry/</code>, <code>crates/use-statistics/</code>, <code>crates/use-linear/</code>, <code>crates/use-number/</code>, <code>crates/use-complex/</code>, <code>crates/use-calculus/</code>, <code>crates/use-probability/</code>, <code>crates/use-real/</code>, <code>crates/use-interval/</code>, and <code>crates/use-rational/</code><br>
			Use direct APIs today when you want arithmetic primitives, reusable vector primitives, reusable matrix primitives, reusable interval and bound primitives, geometry primitives, checked counting helpers, Catalan-family sequence counts, Geode-array primitives, arithmetic or geometric progression helpers, finite algebra law checks, integer classifications and divisor helpers, named boolean algebra helpers, explicit set operations, angle conversion and trig helpers, descriptive statistics summaries, small linear-system helpers, raw-number classification or constants, complex-number primitives, numerical-calculus helpers, probability primitives, validated real-number helpers, or exact rational arithmetic without the wider facade.
		</td>
		<td width="33%" valign="top">
			<strong>Use algebra directly</strong><br>
			<code>crates/use-algebra/</code><br>
			Reach for finite law checks when you want group-like or ring-like validation over a sample set without building a trait hierarchy.
		</td>
	</tr>
</table>

## What this workspace ships

RustUse/use-math is now a 23-crate workspace. Each crate is usable on its own, and the facade crate composes the focused crates when you want one import surface.

| Crate               | Path                        | Purpose                                                                                                               | Best fit                                                                             |
| ------------------- | --------------------------- | --------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------ |
| `use-math`          | `crates/use-math/`          | Feature-gated facade with direct re-exports for concrete APIs plus namespace access to every focused crate            | One dependency and one import surface                                                |
| `use-arithmetic`    | `crates/use-arithmetic/`    | Operation-level arithmetic helpers for gcd/lcm, floor division, divisibility, parity, and explicit overflow modes     | You want arithmetic primitives without a broader numeric abstraction                 |
| `use-vector`        | `crates/use-vector/`        | Reusable 2D, 3D, and 4D vector primitives with dot products, `Vector3::cross`, normalization, distance, and lerp      | You want vector math without geometry-specific or matrix-specific abstractions       |
| `use-matrix`        | `crates/use-matrix/`        | Reusable 2x2, 3x3, and 4x4 matrix primitives with transpose, determinant, inverse, and matrix-vector products         | You want reusable matrix math without geometry-specific transforms                   |
| `use-geometry`      | `crates/use-geometry/`      | Utility-first 2D geometry primitives, shapes, bounds, orientation, and distance helpers                               | Geometry is the only math surface you need                                           |
| `use-combinatorics` | `crates/use-combinatorics/` | Checked counting helpers for factorials, permutations, and combinations                                               | You only need combinatorics helpers                                                  |
| `use-catalan`       | `crates/use-catalan/`       | Checked Catalan and Fuss-Catalan counting helpers                                                                     | You want exact Catalan-family counts without a broader sequence crate                |
| `use-geode`         | `crates/use-geode/`         | Geode-array type vectors, hyper-Catalan coefficients, polygon subdivision counts, and small exact Geode recurrences   | You want exact Geode-array helpers without a broader series engine                   |
| `use-series`        | `crates/use-series/`        | Checked arithmetic and geometric progression helpers                                                                  | You want exact nth-term and partial-sum helpers without a broader sequence framework |
| `use-algebra`       | `crates/use-algebra/`       | Finite-sample algebraic law helpers for closure-defined operations and structures                                     | You want explicit group-like or ring-like checks without a trait hierarchy           |
| `use-integer`       | `crates/use-integer/`       | Sign classification, parity checks, validated divisibility, and exact gcd/lcm helpers                                 | You want small integer helpers without a broader numeric framework                   |
| `use-logic`         | `crates/use-logic/`         | Named boolean algebra helpers such as implication, equivalence, XOR, NAND, NOR, and majority                          | You want explicit truth-table helpers without a broader predicate framework          |
| `use-set`           | `crates/use-set/`           | Slice-based membership, subset, disjointness, and order-preserving set-operation helpers                              | You want explicit set helpers without a broader collection framework                 |
| `use-trigonometry`  | `crates/use-trigonometry/`  | Explicit angle values, degree/radian conversion helpers, normalization helpers, and direct trig wrappers              | You want explicit trig helpers without a broader numeric framework                   |
| `use-statistics`    | `crates/use-statistics/`    | Mean, median, variance, and standard-deviation helpers over `f64` slices                                              | You want descriptive summaries without a broader data-analysis framework             |
| `use-linear`        | `crates/use-linear/`        | Solver-style 2x2 linear-system helpers that compose `use-matrix` and `use-vector`                                     | You want small linear algorithms without owning primitive matrix/vector types        |
| `use-number`        | `crates/use-number/`        | Floating-point classification helpers, lightweight finiteness checks, and shared numeric constants                    | You want raw-number helpers without a broader numeric framework                      |
| `use-complex`       | `crates/use-complex/`       | Composable complex-number and imaginary-number primitives with rectangular and polar helpers                          | You want a small complex-number layer without a heavy framework                      |
| `use-calculus`      | `crates/use-calculus/`      | Small numerical-calculus helpers for finite-difference derivatives, definite integrals, and symmetric limit estimates | You want explicit approximation helpers without symbolic math                        |
| `use-probability`   | `crates/use-probability/`   | Validated probability values, independent-event helpers, and a compact Bernoulli model                                | You want explicit normalized probabilities without a larger statistics framework     |
| `use-real`          | `crates/use-real/`          | Validated finite values, real-number abstractions, and explicit tolerance-based comparisons                           | You want floating-point validation without a broader numeric framework               |
| `use-interval`      | `crates/use-interval/`      | Reusable interval and bound primitives with containment, overlap, emptiness, and intersection operations              | You want generic interval semantics without real-specific or domain-specific policy  |
| `use-rational`      | `crates/use-rational/`      | Exact normalized fractions with checked arithmetic and explicit conversion to approximate floating-point values       | You want exact fraction arithmetic without a broader numeric framework               |

All focused crates currently ship concrete APIs.

| If you need to...                                             | Start here                 |
| ------------------------------------------------------------- | -------------------------- |
| Add one dependency and opt into math surfaces with features   | `use-math`                 |
| Work with operation-level arithmetic helpers directly         | `use-arithmetic`           |
| Work with reusable vector primitives directly                 | `use-vector`               |
| Work with reusable matrix primitives directly                 | `use-matrix`               |
| Validate 2D coordinates and shapes from user or file input    | `use-geometry`             |
| Do checked counting without geometry types                    | `use-combinatorics`        |
| Work with exact Catalan-family sequence helpers directly      | `use-catalan`              |
| Work with exact Geode-array primitives directly               | `use-geode`                |
| Work with progression nth-term and sum helpers directly       | `use-series`               |
| Work with finite algebra law helpers directly                 | `use-algebra`              |
| Work with sign, parity, and common-divisor helpers directly   | `use-integer`              |
| Work with named boolean algebra helpers directly              | `use-logic`                |
| Work with explicit set helpers directly                       | `use-set`                  |
| Work with explicit angle conversion and trig helpers directly | `use-trigonometry`         |
| Work with descriptive statistics helpers directly             | `use-statistics`           |
| Work with small linear-system helpers directly                | `use-linear`               |
| Work with raw-number classification or constants directly     | `use-number`               |
| Work with small complex-number primitives directly            | `use-complex`              |
| Work with explicit numerical-calculus helpers directly        | `use-calculus`             |
| Work with explicit probability primitives directly            | `use-probability`          |
| Work with validated real-number helpers directly              | `use-real`                 |
| Work with reusable interval and bound primitives directly     | `use-interval`             |
| Work with exact normalized fractions directly                 | `use-rational`             |
| Keep the dependency and API surface as narrow as possible     | The focused crate directly |

> [!TIP]
> Prefer the focused crates when you want the narrowest dependency footprint. Choose `use-math` when consumer ergonomics and one-dependency integration matter more than shaving every unused surface.

## Choose your entry point

Pick the crate based on the integration shape you want, not just the total feature count.

| You want...                                                  | Choose...                                        | Why                                                                                  |
| ------------------------------------------------------------ | ------------------------------------------------ | ------------------------------------------------------------------------------------ |
| One dependency for the current workspace surface             | `use-math`                                       | The facade unifies the concrete APIs and exposes matching nested namespaces          |
| Arithmetic helpers only                                      | `use-arithmetic`                                 | You keep floor division, divisibility, and overflow-mode helpers explicit and local  |
| Reusable vector primitives only                              | `use-vector`                                     | You keep dot products, interpolation, distance, and normalization explicit and local |
| Matrix primitives only                                       | `use-matrix`                                     | You keep direct matrix construction and operations explicit and local                |
| Geometry-only code with direct type access                   | `use-geometry`                                   | You avoid facade indirection and keep dependencies minimal                           |
| Counting helpers only                                        | `use-combinatorics`                              | You get checked math helpers without bringing in geometry modules                    |
| Catalan-family sequence helpers only                         | `use-catalan`                                    | You keep Catalan and Fuss-Catalan counting explicit and local                        |
| Geode-array primitives only                                  | `use-geode`                                      | You keep finite type vectors and small exact Geode recurrences explicit and local    |
| Progression helpers only                                     | `use-series`                                     | You keep arithmetic and geometric nth-term and sum helpers explicit and local        |
| Finite algebra law helpers only                              | `use-algebra`                                    | You keep closure-based structure checks explicit and local                           |
| Integer helpers only                                         | `use-integer`                                    | You keep parity, divisibility, and gcd/lcm logic explicit and local                  |
| Boolean algebra helpers only                                 | `use-logic`                                      | You keep implication, equivalence, and related helpers explicit and local            |
| Set helpers only                                             | `use-set`                                        | You keep membership and set-operation intent explicit and local                      |
| Trigonometry helpers only                                    | `use-trigonometry`                               | You keep degree/radian handling and direct trig evaluation explicit and local        |
| Descriptive statistics only                                  | `use-statistics`                                 | You keep central-tendency and variance summaries explicit and local                  |
| Solver-style linear helpers only                             | `use-linear`                                     | You keep solve logic explicit while matrix/vector ownership stays in focused crates  |
| Raw-number helpers only                                      | `use-number`                                     | You keep plain `f64` classification and shared constants explicit and local          |
| Complex-number primitives without the rest of the facade     | `use-complex`                                    | You get rectangular, imaginary, and polar helpers directly                           |
| Numerical-calculus helpers without the rest of the facade    | `use-calculus`                                   | You keep derivative, integral, and limit approximations explicit and local           |
| Probability primitives without the rest of the facade        | `use-probability`                                | You keep normalization and independence assumptions explicit and local               |
| Validated real-number helpers without the rest of the facade | `use-real`                                       | You keep floating-point validation and real-specific policy explicit and local       |
| Reusable interval and bound primitives only                  | `use-interval`                                   | You keep containment, overlap, and intersection semantics explicit and local         |
| Exact rational arithmetic without the rest of the facade     | `use-rational`                                   | You keep fraction normalization and exact arithmetic explicit and local              |
| Maximum control over enabled API surface                     | A focused crate, or `use-math` with defaults off | You choose exactly which modules compile into the final build                        |

## Project structure

```text
.
├── Cargo.toml
├── README.md
├── crates/
│   ├── use-algebra/
│   ├── use-arithmetic/
│   ├── use-calculus/
│   ├── use-catalan/
│   ├── use-combinatorics/
│   ├── use-complex/
│   ├── use-geometry/
│   ├── use-geode/
│   ├── use-integer/
│   ├── use-interval/
│   ├── use-matrix/
│   ├── use-linear/
│   ├── use-logic/
│   ├── use-math/
│   ├── use-number/
│   ├── use-probability/
│   ├── use-rational/
│   ├── use-real/
│   ├── use-series/
│   ├── use-set/
│   ├── use-statistics/
│   ├── use-vector/
│   └── use-trigonometry/
└── scripts/
```

| Path                        | Role                                                                                       |
| --------------------------- | ------------------------------------------------------------------------------------------ |
| `Cargo.toml`                | Workspace membership, shared package metadata, release metadata, and lint policy           |
| `crates/use-math/`          | Feature-gated facade crate with root re-exports for implemented APIs and nested namespaces |
| `crates/use-arithmetic/`    | Direct arithmetic helpers for gcd/lcm, floor division, divisibility, and overflow modes    |
| `crates/use-geometry/`      | Direct 2D geometry APIs, validated constructors, and invariant checks                      |
| `crates/use-combinatorics/` | Direct checked counting APIs                                                               |
| `crates/use-catalan/`       | Direct Catalan-family counting APIs                                                        |
| `crates/use-geode/`         | Direct Geode-array type-vector and coefficient APIs                                        |
| `crates/use-series/`        | Direct arithmetic and geometric progression APIs                                           |
| `crates/use-integer/`       | Direct integer classification and common-divisor helpers                                   |
| `crates/use-logic/`         | Direct boolean algebra helper APIs                                                         |
| `crates/use-set/`           | Direct set helper APIs                                                                     |
| `crates/use-trigonometry/`  | Direct angle-conversion and trigonometry helper APIs                                       |
| `crates/use-statistics/`    | Direct descriptive-statistics helper APIs                                                  |
| `crates/use-matrix/`        | Direct matrix primitive and matrix-operation APIs                                          |
| `crates/use-linear/`        | Direct solver-style linear helper APIs                                                     |
| `crates/use-vector/`        | Direct reusable vector primitives and vector-operation APIs                                |
| `crates/use-number/`        | Direct raw-number helper APIs                                                              |
| `crates/use-interval/`      | Direct interval and bound primitive APIs                                                   |
| `crates/use-algebra/`       | Direct finite algebra law helper APIs                                                      |
| `scripts/`                  | Workspace automation and mirror sync helpers                                               |

## Installation

This repository is public before the first crates.io publish. Until the
first release wave is live, use the workspace directly or depend on a Git
revision. The versioned snippets below apply after the published release line
exists on crates.io.

Git dependency before the first crates.io release:

```toml
[dependencies]
use-math = { git = "https://github.com/RustUse/use-math", rev = "<commit>" }
```

For focused crates, replace `use-math` with the focused crate you need, such as
`use-geometry`, `use-combinatorics`, or another focused crate like
`use-algebra`. Pin a commit or future tag instead of following the moving default
branch.

When consuming the published release line, pull in the smallest surface that matches your application.

Facade crate with default features:

```toml
[dependencies]
use-math = "0.0.5"
```

Facade crate with geometry only:

```toml
[dependencies]
use-math = { version = "0.0.5", default-features = false, features = ["geometry"] }
```

Focused crates directly:

```toml
[dependencies]
use-geometry = "0.0.5"
use-combinatorics = "0.0.5"
use-modular = "0.0.5"
```

> [!NOTE]
> The workspace is still pre-1.0. Release sequencing matters because the `use-math` facade depends on matching focused-crate versions.

## Quick examples

### Geometry through the facade

```rust
use use_math::prelude::*;

let origin = Point2::try_new(0.0, 0.0)?;
let point = Point2::try_new(3.0, 4.0)?;
let distance = distance_2d(origin, point);
let midpoint = midpoint_2d(origin, point);

assert_eq!(distance, 5.0);
assert_eq!(midpoint, Point2::try_new(1.5, 2.0)?);
# Ok::<(), use_math::geometry::GeometryError>(())
```

### Checked counting through the facade

```rust
use use_math::prelude::*;

assert_eq!(factorial(5)?, 120);
assert_eq!(permutations(5, 3)?, 60);
assert_eq!(combinations(5, 2)?, 10);
# Ok::<(), use_math::combinatorics::CombinatoricsError>(())
```

### Direct geometry types

```rust
use use_math::geometry::{Circle, Point2, Triangle};

let a = Point2::try_new(0.0, 0.0)?;
let b = Point2::try_new(4.0, 0.0)?;
let c = Point2::try_new(0.0, 3.0)?;
let triangle = Triangle::try_new(a, b, c)?;
let circle = Circle::try_new(a, 3.0)?;

assert_eq!(triangle.area(), 6.0);
assert_eq!(circle.radius(), 3.0);
# Ok::<(), use_math::geometry::GeometryError>(())
```

## Validated input path

Use `try_new` constructor variants when coordinates or shapes originate outside your codebase, such as user input, configuration files, or network payloads. Infallible constructors like `Point2::new(...)` or `Aabb2::from_points(...)` remain available for values you already trust.

> [!IMPORTANT]
> Keep validation at the edge. Once data is trusted, the APIs stay lightweight and composable.

## Feature model

The facade crate exposes a small feature surface:

| Feature         | Enables                                                                                                                                                        | Default |
| --------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------- |
| `arithmetic`    | Re-exports from `use-arithmetic`, including checked and overflow-mode arithmetic helpers, floor-division helpers, and the nested `use_math::arithmetic` module | No      |
| `geometry`      | Re-exports from `use-geometry` and geometry facade examples/tests                                                                                              | No      |
| `combinatorics` | Re-exports from `use-combinatorics` and combinatorics facade examples/tests                                                                                    | No      |
| `catalan`       | Re-exports from `use-catalan`, including Catalan and Fuss-Catalan helpers                                                                                      | No      |
| `geode`         | Re-exports from `use-geode`, including `TypeVector`, `hyper_catalan`, `geode_memoized`, and the nested `use_math::geode` module                                | No      |
| `algebra`       | Re-exports from `use-algebra`, including finite algebra-law helpers such as `identity_element`, `is_abelian_group`, and `is_ring`                              | No      |
| `modular`       | Re-exports from `use-modular`, including normalized residues, `Modular`, congruence checks, inverses, exponentiation, and the `use_math::modular` namespace    | No      |
| `series`        | Re-exports from `use-series`, including arithmetic and geometric progression helpers                                                                           | No      |
| `integer`       | Re-exports from `use-integer`, including sign, parity, divisibility, and gcd/lcm helpers                                                                       | No      |
| `logic`         | Re-exports from `use-logic`, including implication, equivalence, XOR, NAND, NOR, and majority helpers                                                          | No      |
| `set`           | Re-exports from `use-set`, including membership predicates and order-preserving set operations                                                                 | No      |
| `trigonometry`  | Re-exports from `use-trigonometry`, including `Angle`, unit-conversion helpers, normalization helpers, and direct trig wrappers                                | No      |
| `statistics`    | Re-exports from `use-statistics`, including `StatisticsError`, mean/median, variance, and standard-deviation helpers                                           | No      |
| `matrix`        | Re-exports from `use-matrix`, including `Matrix2`, `Matrix3`, `Matrix4`, transpose helpers, determinants, and inverses for 2x2 and 3x3 matrices                | No      |
| `linear`        | Re-exports from `use-linear`, including `solve_2x2` and `LinearError`; the `linear` feature also enables `matrix` and `vector`                                 | No      |
| `number`        | Re-exports from `use-number`, including floating-point classification helpers and shared numeric constants                                                     | No      |
| `complex`       | Re-exports from `use-complex`, including `Complex` and `Imaginary`                                                                                             | No      |
| `calculus`      | Re-exports from `use-calculus`, including derivative, integral, and limit helpers                                                                              | No      |
| `probability`   | Re-exports from `use-probability`, including `Probability`, `Bernoulli`, and event helpers                                                                     | No      |
| `rational`      | Re-exports from `use-rational`, including `Rational` and `RationalError`                                                                                       | No      |
| `real`          | Re-exports from `use-real`, including `Real`, `RealInterval`, and `approx_eq`                                                                                  | No      |
| `interval`      | Re-exports from `use-interval`, including `Bound`, `Interval`, containment checks, overlap tests, intersections, and the `use_math::interval` namespace        | No      |
| `full`          | Enables every focused crate feature in the workspace                                                                                                           | Yes     |

If you want the facade but only one module, disable defaults and enable the feature you need:

```toml
[dependencies]
use-math = { version = "0.0.5", default-features = false, features = ["combinatorics"] }
```

## Maturity and release model

RustUse/use-math is intentionally pre-1.0. The current release line is `0.0.x`, and the facade crate should publish only after the focused crates for that same version are available to the registry index.

| Release concern   | Current posture                                       |
| ----------------- | ----------------------------------------------------- |
| Versioning        | Lockstep `0.0.x` releases across the workspace        |
| Publish order     | Publish focused crates first, then publish `use-math` |
| API growth        | Favor small, composable surfaces over rapid expansion |
| Dependency policy | Keep dependencies minimal and predictable             |

## Development

Run commands from the repository root.

Requirements:

```text
Rust 1.95.0 or newer
cargo
```

Repo-owned DX shortcuts:

```sh
cargo xcheck
cargo xlint
cargo xtest
cargo xtest-minimal
cargo xexamples
cargo xdoc
```

These aliases live in `.cargo/config.toml`, so contributors do not need `make`
for the common workspace validation path. If you are using VS Code, the same
flows are available through the checked-in tasks in `.vscode/tasks.json`, and
the repository recommends the Rust, TOML, YAML, and GitHub Actions extensions it
expects through `.vscode/extensions.json`.

Open-source intake and onboarding:

- GitHub issue forms now cover bugs, feature requests, and docs/onboarding gaps.
- The issue chooser routes questions and design exploration toward Discussions once the repository enables them.
- The pull request template now asks for linked issue or discussion context, change type, and the repo-owned Cargo alias checks.

Optional maintainer and contributor tooling bootstrap:

```sh
bash scripts/bootstrap-dev-tools.sh
pwsh -File scripts/bootstrap-dev-tools.ps1
```

Both scripts install the optional Cargo tools used across local advisory,
release, and supply-chain workflows: `cargo-deny`, `cargo-audit`,
`cargo-cyclonedx`, `release-plz`, and `cargo-machete`.

If you are preparing a clean-history open-source launch, use
`docs/first-public-commit.md` as the checklist for the first public revision and
the first public push, and use `docs/history-reset-and-republish.md` as the
exact operator runbook for rewriting history in the existing private repository.

If you prefer a containerized setup, the repository also ships a checked-in
devcontainer in `.devcontainer/` with the Rust toolchain, recommended VS Code
extensions, and an initial `cargo xcheck` post-create validation.

Primary validation commands:

```sh
cargo fmt --all --check
cargo check --workspace --all-features
cargo check --workspace --all-features --examples
cargo test --workspace --all-features
cargo test --workspace --no-default-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo doc --workspace --all-features --no-deps
cargo deny check
cargo audit
```

Convenience targets for the release path:

```sh
make help
make verify
make examples
make test-minimal
make publish-dry-run-focused
make release-readiness
```

`make release-readiness` intentionally validates the focused crates' publish
surface first. The `use-math` facade still needs its final `cargo publish
--dry-run` only after matching focused-crate versions exist in the crates.io
index.

If you prefer not to use `make`, the Cargo aliases and VS Code tasks cover the
same day-to-day validation flows cross-platform.

`Cargo.lock` is committed intentionally for reproducible CI, advisory checks, and release dry runs in this library workspace.

## Mirrors

The canonical repository is hosted on GitHub. Additional mirrors can be activated for redundancy and contributor access once their repository variables and SSH key material are configured.

See `FORGES.md` for the cross-forge sync model and mirror activation details.

## Contributing

Contributions should favor correctness, clear naming, small APIs, and minimal dependencies over broad surface area. See `CONTRIBUTING.md` for validation, release, and cross-forge expectations.

## Community and project policy

- Use the RustUse [support guidance](https://github.com/RustUse/.github/blob/main/SUPPORT.md) for help and routing.
- Use the RustUse [security policy](https://github.com/RustUse/.github/blob/main/SECURITY.md) for private vulnerability reporting.
- Use the RustUse [code of conduct](https://github.com/RustUse/.github/blob/main/CODE_OF_CONDUCT.md) for collaboration expectations.
- Use `GOVERNANCE.md` and `MAINTAINERS.md` for decision-making and release authority.

## License

Licensed under MIT OR Apache-2.0. See `LICENSE-MIT` and `LICENSE-APACHE`.
