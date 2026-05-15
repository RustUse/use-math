# Changelog

## Unreleased

### Added

- Added the `use-modular` crate with normalized modular arithmetic helpers, congruence checks, modular inverses, exponentiation, and the optional `Modular` helper type for the RustUse math workspace.
- Added the `modular` feature to the `use-math` facade with root modular reexports plus the `use_math::modular` namespace.
- Added the `use-prime` crate with primality checks, prime search, sieve helpers, deterministic factorization, and prime-factor utilities for the RustUse math workspace.
- Added the `prime` feature to the `use-math` facade with root prime reexports plus the `use_math::prime` namespace.
- Added the `use-polynomial` crate with coefficient-based polynomial primitives, evaluation, derivatives, arithmetic, integrals, and low-degree real-root helpers for the RustUse math workspace.
- Added the `polynomial` feature to the `use-math` facade with root polynomial reexports plus the `use_math::polynomial` namespace.
- Added the `use-equation` crate with explicit linear and quadratic equation helpers, small `2x2` system solving, reusable `Roots` outputs, and a `RootSolver` trait for the RustUse math workspace.
- Added the `equation` feature to the `use-math` facade with the `use_math::equation` namespace plus non-conflicting root equation reexports.

### Changed

- Extended workspace release metadata, publish-readiness lists, maintainer publish-order docs, and facade documentation to include `use-modular` as a focused crate in the lockstep release surface.
- Extended workspace release metadata, publish-readiness lists, maintainer publish-order docs, and facade documentation to include `use-prime` as a focused crate in the lockstep release surface.
- Extended workspace release metadata, publish-readiness lists, maintainer publish-order docs, and facade documentation to include `use-polynomial` as a focused crate in the lockstep release surface.
- Extended workspace release metadata, publish-readiness lists, maintainer publish-order docs, and facade documentation to include `use-equation` as a focused crate in the lockstep release surface.

## [0.0.5](https://github.com/RustUse/use-math/compare/use-math-v0.0.4...use-math-v0.0.5) - 2026-05-15

### Added

- Added the `use-interval` crate with small bound and interval primitives, containment checks, overlap logic, and interval intersections for the RustUse math workspace.
- Added the `interval` feature to the `use-math` facade with root `Bound` and `Interval` reexports plus the `use_math::interval` namespace.

### Changed

- Made `use-real::RealInterval` compose `use-interval` for its closed-interval representation while keeping finite-value and tolerance policy in `use-real`.
- Extended workspace release metadata, publish-readiness lists, and maintainer publish-order docs to include `use-interval` as a focused crate in the lockstep release surface, with `use-interval` now ordered before `use-real`.

## [0.0.4](https://github.com/RustUse/use-math/compare/use-math-v0.0.3...use-math-v0.0.4) - 2026-05-15

### Added

- Added the `use-matrix` crate with focused 2x2, 3x3, and 4x4 matrix primitives, direct matrix operations, and matrix-vector multiplication for the RustUse math workspace.
- Added the `matrix` feature to the `use-math` facade with root matrix reexports plus the `use_math::matrix` namespace.

### Changed

- Narrowed `use-linear` so it now composes `use-matrix` and `use-vector` for primitive ownership while keeping `solve_2x2` and `LinearError` as the focused linear helper surface.
- Extended publish-readiness lists, release metadata, and maintainer publish-order docs to include `use-vector`, plus the dependency-ordered `use-vector` -> `use-matrix` -> `use-linear` publish path ahead of `use-math`.

## [0.0.3](https://github.com/RustUse/use-math/compare/use-math-v0.0.2...use-math-v0.0.3) - 2026-05-15

### Added

- Added the `use-arithmetic` crate with gcd/lcm, divisibility, parity, floor-division, and explicit checked, saturating, and wrapping arithmetic helpers.
- Added the `arithmetic` feature to the `use-math` facade with nested `use_math::arithmetic` namespacing plus non-conflicting root and prelude reexports.

### Changed

- Extended workspace docs, publish-readiness lists, and release metadata to include `use-arithmetic` as a focused crate in the lockstep release surface.

## [0.0.2](https://github.com/RustUse/use-math/compare/use-math-v0.0.1...use-math-v0.0.2) - 2026-05-15

### Changed

- Add use-geode crate and integrate into workspace

### Added

- Added a manual `Facade Publish Readiness` workflow and matching local facade dry-run target so `use-math` can be revalidated once focused crates are live on crates.io.
- Added `release-plz` configuration and maintainer workflows for lockstep version PR automation and future publish automation across `use-geometry`, `use-combinatorics`, and `use-math`.
- Added cross-platform Cargo aliases, VS Code task definitions, and extension recommendations so contributors can run the main validation flows without depending on `make`.
- Added issue forms, a stricter pull request template, optional Cargo tool bootstrap scripts, and a checked-in devcontainer for open-source contributor onboarding.
- Added `docs/first-public-commit.md` so a clean-history public launch can preserve the current repo, release, and DX decisions.
- Added `docs/history-reset-and-republish.md` with the exact same-repository history-reset, GitHub verification, and first crates.io publish sequence.
- Added `docs/maintainer-release-flow.md` with the exact release-plz review, changelog, and publish sequence maintainers should follow after the initial manual release wave.

### Changed

- Made the first publish-wave crate manifests explicitly publishable while keeping the workspace-level default opt-out, so the initial crates.io release can happen in dependency order.
- Made `use-math` facade examples, doctests, and integration tests feature-aware so `cargo test --workspace --no-default-features` passes cleanly.
- Clarified release policy, mirror activation requirements, and the intentional `Cargo.lock` policy across the root documentation.
- Aligned the release docs with the current publishable-manifest policy for the first release wave.
- Cleaned geometry helpers, tests, and crate README docs so strict `cargo clippy --workspace --all-targets --all-features -- -D warnings` passes.
- Replaced the mirror workflow example with an active guarded workflow that stays dormant until mirror URLs and matching SSH key secrets are configured.
- Aligned legal and package metadata on `RustUse Contributors`, added shared crate authorship, and configured `use-math` docs.rs builds to advertise all features.
- Promoted `Publish Readiness / Release Readiness Checks` from a recommendation to an explicit pre-release branch-protection requirement in the maintainer docs.
- Hardened the manual facade and release-plz publish workflows with crates.io existence checks so first-release misuse fails fast with an explicit readiness guard.
- Aligned support and Discussions wording so the public-facing routing stays accurate both before and after Discussions is enabled.
- Tightened the README landing page and launch posture so the public repository reads correctly before the first crates.io release is live and shows the Git dependency path clearly.
- Tightened commit and PR title guidance so generated release-plz changelog entries stay readable and low-noise.
- Tightened the release-plz parser rules to prefer strict conventional-commit shapes before falling back to generic `Changed` entries.
- Added a shorter maintainer release checklist directly to `RELEASE.md` so the main release policy now carries the high-signal operational flow.

### Added

- Added repository health files for security reporting, support routing, code of conduct, governance, and maintainer authority.
- Added GitHub issue templates and a pull request template for the first public launch.

## 0.0.1 - 2026-05-03

### Added

- Introduced the `use-geometry` crate with 2D points, vectors, lines, segments, circles, triangles, distance helpers, and orientation helpers.
- Introduced the `use-combinatorics` crate with checked factorial, permutation, and combination helpers.
- Introduced the `use-math` facade crate with feature-gated geometry and combinatorics reexports plus a shared prelude.
- Added validated geometry construction through `try_new`-style APIs and non-finite component errors for caller-provided floating-point inputs.
- Added runnable examples and integration tests covering direct focused-crate usage and `use-math` facade usage.

### Tooling

- Added fmt, clippy, docs, dependency policy, and advisory checks to the documented release workflow.
- Documented the first publish sequence for the `use-geometry`, `use-combinatorics`, then `use-math` crates.io release.
