# RustUse/use-math

<p align="center">
	<img alt="Rust 1.95.0+" src="https://img.shields.io/badge/Rust-1.95.0%2B-f46623?logo=rust&logoColor=white">
	<img alt="Edition 2024" src="https://img.shields.io/badge/edition-2024-0f766e">
	<img alt="18 workspace crates" src="https://img.shields.io/badge/workspace-18%20crates-1d4ed8">
	<img alt="Status pre-release" src="https://img.shields.io/badge/status-pre--release-c2410c">
	<img alt="License MIT or Apache-2.0" src="https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-2a9d8f">
</p>

<p align="center">
	<strong>Utility-first Rust math crates with concrete geometry and counting APIs plus scaffolded focused boundaries for the rest of the workspace.</strong><br>
	Focused crates stay small. The facade crate composes them behind opt-in features and keeps implemented root re-exports separate from namespace-only scaffolds.
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

This repository is the source workspace for RustUse's math surface. Today it combines concrete focused crates for geometry, checked combinatorics, complex numbers, numerical calculus, probability, real-number helpers, and rational arithmetic with ten additional scaffolded crate boundaries for future numeric, algebraic, statistical, trigonometric, and logical surfaces. `use-math` composes the whole workspace behind feature flags while keeping root re-exports limited to the crates that already expose real APIs.

## Current status

- The GitHub repository may be public before the first crates.io release is live.
- Until then, consume the crates from a pinned Git revision or work from the workspace directly.
- `use-geometry`, `use-combinatorics`, `use-complex`, `use-calculus`, `use-probability`, `use-real`, and `use-rational` are the concrete focused APIs today; the remaining focused crates are scaffold-only publishable boundaries.
- The planned first release order is every focused crate first, then `use-math` after crates.io index propagation.

<table>
	<tr>
		<td width="33%" valign="top">
			<strong>Pull in one facade</strong><br>
			<code>crates/use-math/</code><br>
			Reach for the shared <code>prelude</code>, root re-exports for implemented APIs, and namespace modules for the scaffolded crate boundaries.
		</td>
		<td width="33%" valign="top">
			<strong>Use concrete focused crates</strong><br>
			<code>crates/use-geometry/</code>, <code>crates/use-combinatorics/</code>, <code>crates/use-complex/</code>, <code>crates/use-calculus/</code>, <code>crates/use-probability/</code>, <code>crates/use-real/</code>, and <code>crates/use-rational/</code><br>
			Use direct APIs today when you want geometry primitives, checked counting helpers, complex-number primitives, numerical-calculus helpers, probability primitives, finite real-number helpers, or exact rational arithmetic without the wider facade.
		</td>
		<td width="33%" valign="top">
			<strong>Stabilize future boundaries</strong><br>
			<code>crates/use-number/</code> through <code>crates/use-set/</code><br>
			Depend on the scaffolded focused crates when stable crate names and release plumbing matter before the concrete APIs land.
		</td>
	</tr>
</table>

## What this workspace ships

RustUse/use-math is now an 18-crate workspace. Each crate is usable on its own, and the facade crate composes the focused crates when you want one import surface.

| Crate                     | Path                                           | Purpose                                                                                                                    | Best fit                                                        |
| ------------------------- | ---------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------- |
| `use-math`                | `crates/use-math/`                             | Feature-gated facade with direct re-exports for implemented APIs plus namespace access to scaffold crates                  | One dependency and one import surface                           |
| `use-geometry`            | `crates/use-geometry/`                         | Utility-first 2D geometry primitives, shapes, bounds, orientation, and distance helpers                                    | Geometry is the only math surface you need                      |
| `use-combinatorics`       | `crates/use-combinatorics/`                    | Checked counting helpers for factorials, permutations, and combinations                                                    | You only need combinatorics helpers                             |
| `use-complex`             | `crates/use-complex/`                          | Composable complex-number and imaginary-number primitives with rectangular and polar helpers                               | You want a small complex-number layer without a heavy framework |
| `use-calculus`            | `crates/use-calculus/`                         | Small numerical-calculus helpers for finite-difference derivatives, definite integrals, and symmetric limit estimates      | You want explicit approximation helpers without symbolic math   |
| `use-probability`         | `crates/use-probability/`                      | Validated probability values, independent-event helpers, and a compact Bernoulli model                                     | You want explicit normalized probabilities without a larger statistics framework |
| `use-real`                | `crates/use-real/`                             | Validated finite values, checked real intervals, and explicit tolerance-based comparisons                                  | You want floating-point validation without a broader numeric framework |
| `use-rational`            | `crates/use-rational/`                         | Exact normalized fractions with checked arithmetic and explicit conversion to approximate floating-point values             | You want exact fraction arithmetic without a broader numeric framework |
| Scaffolded focused crates | `crates/use-number/` through `crates/use-set/` | Publishable crate boundaries with README, example, test, and facade namespace wiring, but no concrete public math APIs yet | Stable crate naming and release plumbing before API expansion   |

The scaffolded focused crates are `use-number`, `use-integer`, `use-series`, `use-catalan`, `use-algebra`, `use-linear`, `use-statistics`, `use-trigonometry`, `use-logic`, and `use-set`.

| If you need to...                                              | Start here                            |
| -------------------------------------------------------------- | ------------------------------------- |
| Add one dependency and opt into math surfaces with features    | `use-math`                            |
| Validate 2D coordinates and shapes from user or file input     | `use-geometry`                        |
| Do checked counting without geometry types                     | `use-combinatorics`                   |
| Work with small complex-number primitives directly             | `use-complex`                         |
| Work with explicit numerical-calculus helpers directly         | `use-calculus`                        |
| Work with explicit probability primitives directly             | `use-probability`                     |
| Work with explicit finite-value and interval helpers directly  | `use-real`                            |
| Work with exact normalized fractions directly                  | `use-rational`                        |
| Lock in a focused crate boundary before its concrete API ships | The scaffolded focused crate directly |
| Keep the dependency and API surface as narrow as possible      | The focused crate directly            |

> [!TIP]
> Prefer the focused crates when you want the narrowest dependency footprint. Choose `use-math` when consumer ergonomics and one-dependency integration matter more than shaving every unused surface.

## Choose your entry point

Pick the crate based on the integration shape you want, not just the total feature count.

| You want...                                                | Choose...                                        | Why                                                                                                 |
| ---------------------------------------------------------- | ------------------------------------------------ | --------------------------------------------------------------------------------------------------- |
| One dependency for the current workspace surface           | `use-math`                                       | The facade unifies the concrete APIs and exposes scaffolded crates as nested namespaces             |
| Geometry-only code with direct type access                 | `use-geometry`                                   | You avoid facade indirection and keep dependencies minimal                                          |
| Counting helpers only                                      | `use-combinatorics`                              | You get checked math helpers without bringing in geometry modules                                   |
| Complex-number primitives without the rest of the facade   | `use-complex`                                    | You get rectangular, imaginary, and polar helpers directly                                          |
| Numerical-calculus helpers without the rest of the facade  | `use-calculus`                                   | You keep derivative, integral, and limit approximations explicit and local                          |
| Probability primitives without the rest of the facade      | `use-probability`                                | You keep normalization and independence assumptions explicit and local                              |
| Finite-value and interval helpers without the rest of the facade | `use-real`                                   | You keep floating-point validation and tolerance policy explicit and local                          |
| Exact rational arithmetic without the rest of the facade   | `use-rational`                                   | You keep fraction normalization and exact arithmetic explicit and local                             |
| A stable future-focused crate boundary while APIs incubate | The scaffolded focused crate directly            | You can depend on the crate name now without implying more concrete API than the crate actually has |
| Maximum control over enabled API surface                   | A focused crate, or `use-math` with defaults off | You choose exactly which modules compile into the final build                                       |

## Project structure

```text
.
├── Cargo.toml
├── README.md
├── crates/
│   ├── use-algebra/
│   ├── use-calculus/
│   ├── use-catalan/
│   ├── use-combinatorics/
│   ├── use-complex/
│   ├── use-geometry/
│   ├── use-integer/
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
│   └── use-trigonometry/
└── scripts/
```

| Path                                           | Role                                                                                       |
| ---------------------------------------------- | ------------------------------------------------------------------------------------------ |
| `Cargo.toml`                                   | Workspace membership, shared package metadata, release metadata, and lint policy           |
| `crates/use-math/`                             | Feature-gated facade crate with root re-exports for implemented APIs and nested namespaces |
| `crates/use-geometry/`                         | Direct 2D geometry APIs, validated constructors, and invariant checks                      |
| `crates/use-combinatorics/`                    | Direct checked counting APIs                                                               |
| `crates/use-number/` through `crates/use-set/` | Scaffolded focused crate boundaries for the remaining planned math domains                 |
| `scripts/`                                     | Workspace automation and mirror sync helpers                                               |

## Installation

This repository may become public before the first crates.io publish. Until the
first release wave is live, use the workspace directly or depend on a Git
revision. The versioned snippets below apply after the published release line
exists on crates.io.

Git dependency before the first crates.io release:

```toml
[dependencies]
use-math = { git = "https://github.com/RustUse/use-math", rev = "<commit>" }
```

For focused crates, replace `use-math` with the focused crate you need, such as
`use-geometry`, `use-combinatorics`, or one of the scaffolded crates like
`use-number`. Pin a commit or future tag instead of following the moving default
branch.

When consuming the published release line, pull in the smallest surface that matches your application.

Facade crate with default features:

```toml
[dependencies]
use-math = "0.0.1"
```

Facade crate with geometry only:

```toml
[dependencies]
use-math = { version = "0.0.1", default-features = false, features = ["geometry"] }
```

Focused crates directly:

```toml
[dependencies]
use-geometry = "0.0.1"
use-combinatorics = "0.0.1"
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

| Feature                                                                                                                                                    | Enables                                                                     | Default |
| ---------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------- | ------- |
| `geometry`                                                                                                                                                 | Re-exports from `use-geometry` and geometry facade examples/tests           | No      |
| `combinatorics`                                                                                                                                            | Re-exports from `use-combinatorics` and combinatorics facade examples/tests | No      |
| `complex`                                                                                                                                                  | Re-exports from `use-complex`, including `Complex` and `Imaginary`          | No      |
| `number`, `integer`, `rational`, `real`, `series`, `catalan`, `algebra`, `linear`, `calculus`, `probability`, `statistics`, `trigonometry`, `logic`, `set` | Exposes the corresponding focused crate as a nested namespace module only   | No      |
| `full`                                                                                                                                                     | Enables every focused crate feature in the workspace                        | Yes     |

If you want the facade but only one module, disable defaults and enable the feature you need:

```toml
[dependencies]
use-math = { version = "0.0.1", default-features = false, features = ["combinatorics"] }
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

- Use `SUPPORT.md` for help and routing guidance.
- Use `SECURITY.md` for private vulnerability reporting.
- Use `CODE_OF_CONDUCT.md` for collaboration expectations.
- Use `GOVERNANCE.md` and `MAINTAINERS.md` for decision-making and release authority.

## License

Licensed under MIT OR Apache-2.0. See `LICENSE-MIT` and `LICENSE-APACHE`.
