# Release Policy

RustUse/use-math is not published yet. The root workspace metadata keeps
`publish = false` as the default, while the current first-wave crate manifests
already opt in with `publish = true` on this branch. The release task is to
verify that only the intended crates remain publishable.

If the repository history is reset before the first public push, keep
`docs/first-public-commit.md` alongside this release policy so the clean-slate
repository preserves the current launch and release decisions.

For the exact same-repository reset and first publish sequence, use
`docs/history-reset-and-republish.md`.

Because every focused crate under `crates/` and the `use-math` facade are now
marked `publish = true`, `cargo package` and `cargo publish --dry-run` for
`use-math` will not resolve fully until the matching focused-crate releases are
available in the crates.io index.

## First Publish Wave

The intended first publish candidates are every focused crate under `crates/`
plus `use-math`.

Publish all focused crates first. The order among focused crates is
maintainer-chosen because they do not depend on one another. Wait for crates.io
index propagation, then release `use-math`.

## Publish Surface

Before the first publish wave, confirm that the release surface:

- keeps the workspace-level default at `publish = false`
- keeps every focused crate under `crates/` at `publish = true`
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
- The workflow intentionally dry-runs every focused crate first. The final `use-math` dry-run still depends on those crate versions being visible in the crates.io index.
- `.github/workflows/facade-publish-readiness.yml` is a manual post-publication check that dry-runs `use-math` only after the focused crates are live on crates.io.
- The facade workflow now fails fast unless every focused crate already resolves from crates.io, so the manual gate is explicit instead of relying on a downstream Cargo error.

## Branch Protection Gate

Before the first public release, the canonical GitHub repository should require `Publish Readiness / Release Readiness Checks` on `main`.

This repository can document the required check name, but it cannot enforce branch protection from version-controlled files alone. Set the rule in the GitHub branch protection or ruleset UI before the first crates.io publish.

## Version and Changelog Automation

The repository now includes `release-plz` configuration in `release-plz.toml` and maintainer workflows under `.github/workflows/release-plz-*.yml`.

For the maintainer-facing merge, review, and dispatch sequence, use
`docs/maintainer-release-flow.md`.

- `Release PR Automation` opens or updates a release PR with lockstep version changes for every publishable crate in the workspace.
- The workspace is configured with one `version_group` so all published crates keep the same version.
- The root `CHANGELOG.md` remains the shared changelog and is updated through the `use-math` package entry, including focused-crate commits.
- `Release Publish Automation` can publish automatically on pushes to `main` after the initial manual wave is complete, crates.io trusted publishing is configured for every published crate, and the `CRATES_IO_AUTOPUBLISH_ENABLED` repository variable is set to `true`.
- The publish workflow uses GitHub OIDC, keeps manual dispatch as a fallback, and still checks that every focused crate plus `use-math` already exist on crates.io before it attempts automated publishing.

One-time post-initial-release setup:

- Configure crates.io Trusted Publishing for each published crate with repository owner `RustUse`, repository name `use-math`, and workflow filename `release-plz-release.yml`.
- Leave the crates.io environment field empty unless you intentionally add a matching GitHub Actions environment to the workflow later.
- Set the repository variable `CRATES_IO_AUTOPUBLISH_ENABLED` to `true` only after the initial manual crates.io wave is complete.
- Do not set `CARGO_REGISTRY_TOKEN` for this workflow when using trusted publishing.

## Maintainer Release Checklist

Use this shorter checklist when you want the operational release path without
reading the longer maintainer guide end to end.

For normal post-initial-release releases:

1. Merge ordinary PRs with clean final commit subjects or squash titles that match `type: summary` or `type(scope)!: summary`.
2. Let `Release PR Automation` open or update the release PR.
3. Review the release PR for the lockstep version bump, the generated root `CHANGELOG.md`, and any low-signal fallback entries under `Changed`.
4. Clean up the changelog directly in the release PR branch when the generated wording is accurate but not maintainer-quality.
5. Merge the release PR after the required checks pass.
6. Let the push-triggered `Release Publish Automation` run on the merged release commit, or manually dispatch it with `post-initial-release = true` if you need a controlled rerun.
7. Verify the published crates, docs.rs pages, and any release tags or artifacts after the workflow completes.

For the initial public crates.io wave:

1. Do not use `Release Publish Automation` yet.
2. Run the full release-readiness path and publish every focused crate before `use-math`.
3. Treat `.github/workflows/facade-publish-readiness.yml` as the final facade check once the focused crates resolve from crates.io.

## Publish Readiness Checklist

1. Confirm `cargo fmt` is clean.
2. Confirm `cargo check --workspace --all-features` passes.
3. Confirm `cargo check --workspace --all-features --examples` passes.
4. Confirm `cargo test --workspace --all-features` passes.
5. Confirm `cargo test --workspace --no-default-features` passes.
6. Confirm `cargo clippy --workspace --all-targets --all-features` passes.
7. Confirm `cargo deny check` and `cargo audit` pass.
8. Review README examples, crate metadata, repository health files, `Cargo.lock`, and changelog entries.
9. Confirm the organization-level security, support, and code of conduct
   defaults, plus the local governance docs, reflect the current public launch
   posture.
10. Confirm the focused crates under `crates/` plus `use-math` are the only intentionally publishable crates.
11. Confirm the focused-crate dry-run path passes across the full set, for example via `make publish-dry-run-focused`.
12. Publish all focused crates, then wait for crates.io index resolution.
13. Confirm branch protection on `main` requires `Publish Readiness / Release Readiness Checks` before the first public release.
14. Confirm `cargo publish --dry-run --allow-dirty -p use-math` passes, or run `.github/workflows/facade-publish-readiness.yml`, once the matching focused-crate versions are available on crates.io.
15. Publish the first wave manually in dependency order, because crates.io trusted publishing cannot create new crates for the first release.
16. Reconfirm that the first publish wave crate manifests remain intentionally
    publishable and that any future non-release crates stay opted out.
