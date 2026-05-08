# Changelog

## Unreleased

### Added

- Added a manual `Facade Publish Readiness` workflow and matching local facade dry-run target so `use-math` can be revalidated once focused crates are live on crates.io.
- Added `release-plz` configuration and maintainer workflows for lockstep version PR automation and future publish automation across `use-geometry`, `use-combinatorics`, and `use-math`.
- Added cross-platform Cargo aliases, VS Code task definitions, and extension recommendations so contributors can run the main validation flows without depending on `make`.
- Added issue forms, a stricter pull request template, optional Cargo tool bootstrap scripts, and a checked-in devcontainer for open-source contributor onboarding.

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
