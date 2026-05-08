# Release Policy

RustUse/use-math is not published yet. The root workspace metadata keeps
`publish = false` as the default, while the current first-wave crate manifests
already opt in with `publish = true` on this branch. The release task is to
verify that only the intended crates remain publishable.

Because `use-geometry`, `use-combinatorics`, and `use-math` are already marked
`publish = true`, `cargo package` and `cargo publish --dry-run` for `use-math`
will not resolve fully until the matching focused-crate releases are available
in the crates.io index.

## First Publish Wave

The intended first publish candidates are:

- `use-geometry`
- `use-combinatorics`
- `use-math`

Publish in dependency order: release `use-geometry` first, then
`use-combinatorics`, wait for crates.io index propagation, then release
`use-math`.

## Publish Surface

Before the first publish wave, confirm that the release surface:

- keeps the workspace-level default at `publish = false`
- keeps `crates/use-geometry/Cargo.toml` at `publish = true`
- keeps `crates/use-combinatorics/Cargo.toml` at `publish = true`
- keeps `crates/use-math/Cargo.toml` at `publish = true`
- leaves any future non-release crates opted out until they are intentionally reviewed

## Versioning

- The workspace currently uses lockstep `0.x.y` versioning.
- Before `1.0`, breaking changes should bump the minor version.
- Before `1.0`, additive compatible changes should bump the patch version.
- The facade crate should only advertise actively supported crates and features.

## Automated Release Validation

The repository now includes a dedicated release-validation path:

- `.github/workflows/publish-readiness.yml` runs on pull requests, pushes to `main`, and manual dispatch.
- `make release-readiness` runs the same high-value local checks for examples, no-default-features coverage, and focused-crate publish dry-runs.
- The workflow intentionally dry-runs `use-geometry` and `use-combinatorics` only. The final `use-math` dry-run still depends on those crate versions being visible in the crates.io index.
- `.github/workflows/facade-publish-readiness.yml` is a manual post-publication check that dry-runs `use-math` only after the focused crates are live on crates.io.

## Branch Protection Gate

Before the first public release, the canonical GitHub repository should require `Publish Readiness / Release Readiness Checks` on `main`.

This repository can document the required check name, but it cannot enforce branch protection from version-controlled files alone. Set the rule in the GitHub branch protection or ruleset UI before the first crates.io publish.

## Version and Changelog Automation

The repository now includes `release-plz` configuration in `release-plz.toml` and maintainer workflows under `.github/workflows/release-plz-*.yml`.

- `Release PR Automation` opens or updates a release PR with lockstep version changes for `use-geometry`, `use-combinatorics`, and `use-math`.
- The workspace is configured with one `version_group` so the three published crates keep the same version.
- The root `CHANGELOG.md` remains the shared changelog and is updated through the `use-math` package entry, including focused-crate commits.
- `Release Publish Automation` is manual for now and is meant for the post-initial-release stage, after the repository is ready to rely on trusted publishing or another finalized credential flow.

## Publish Readiness Checklist

1. Confirm `cargo fmt` is clean.
2. Confirm `cargo check --workspace --all-features` passes.
3. Confirm `cargo check --workspace --all-features --examples` passes.
4. Confirm `cargo test --workspace --all-features` passes.
5. Confirm `cargo test --workspace --no-default-features` passes.
6. Confirm `cargo clippy --workspace --all-targets --all-features` passes.
7. Confirm `cargo deny check` and `cargo audit` pass.
8. Review README examples, crate metadata, repository health files, `Cargo.lock`, and changelog entries.
9. Confirm `SECURITY.md`, `SUPPORT.md`, `CODE_OF_CONDUCT.md`, and governance docs reflect the current public launch posture.
10. Confirm the first-wave crate manifests are the only intentionally publishable crates.
11. Confirm `cargo publish --dry-run --allow-dirty -p use-geometry` passes.
12. Confirm `cargo publish --dry-run --allow-dirty -p use-combinatorics` passes.
13. Publish `use-geometry` and `use-combinatorics`, then wait for crates.io
    index resolution.
14. Confirm branch protection on `main` requires `Publish Readiness / Release Readiness Checks` before the first public release.
15. Confirm `cargo publish --dry-run --allow-dirty -p use-math` passes, or run `.github/workflows/facade-publish-readiness.yml`, once the matching focused-crate versions are available on crates.io.
16. Publish the first wave manually in dependency order, because crates.io trusted publishing cannot create new crates for the first release.
17. Reconfirm that the first publish wave crate manifests remain intentionally
    publishable and that any future non-release crates stay opted out.
