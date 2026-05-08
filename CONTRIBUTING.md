# Contributing

RustUse/use-math is intentionally early and small. Contributions should favor correctness, clear naming, and minimal surface area over broad feature count.

For routing and project policy, use `SUPPORT.md`, `SECURITY.md`,
`CODE_OF_CONDUCT.md`, `GOVERNANCE.md`, and `MAINTAINERS.md` alongside this
guide.

## Development Flow

1. Make the smallest useful change that improves the current crates.
2. Add or update unit tests for every public function you introduce or change.
3. Keep dependencies lightweight unless there is a strong justification.
4. Preserve the utility-first, beginner-friendly API direction.

## Local Validation

```sh
cargo fmt --all --check
cargo check --workspace --all-features
cargo check --workspace --all-features --examples
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo deny check
cargo audit
cargo test --workspace --all-features
cargo test --workspace --no-default-features
cargo doc --workspace --all-features --no-deps
```

## Documentation

- Update the root README when the crate list or facade story changes.
- Keep crate README examples small and runnable.
- Keep docs aligned with the currently supported `use-math`, `use-geometry`, and `use-combinatorics` crates.
- Follow [CRATE_TEMPLATE.md](CRATE_TEMPLATE.md) when introducing a new focused crate or expanding the facade surface.

## Cross-forge contributions

GitHub is currently the canonical repository and final merge target for RustUse.
Public mirrors may also exist on GitLab, Codeberg or Forgejo, SourceHut, or
other public Git forges.

The expected rollout is GitLab first, then Codeberg or Forgejo if a second
public mirror is useful, with SourceHut kept as a later optional mirror.

Issues, merge requests, and patches from mirrors are welcome. If a change is
accepted from a mirror, a maintainer may port it into a GitHub branch or pull
request before the final merge.

Contributors do not need to switch platforms just because GitHub is canonical.
When a mirrored issue, merge request, or patch is carried into GitHub, include a
reference to the original source so the review trail stays clear and authorship
is preserved.

Final release tags and any future crates.io publishing are coordinated only from
the canonical GitHub repository. Mirror CI can validate changes, but it is not
release authority.

The checked-in `.github/workflows/mirror.yml` workflow should remain dormant
until the canonical repository has the documented mirror URL variables and
matching SSH key secrets.

## Release Policy

- The workspace-level default keeps `publish = false`, while the current first-wave crate manifests already opt in with `publish = true`.
- Before a release, confirm that only the intended first-wave crates remain publishable.
- The initial supported crates are `use-combinatorics`, `use-geometry`, and `use-math`.
- Versions move in lockstep at `0.x.y` for now.
- Until `1.0`, breaking API changes should bump the minor version and compatible additive changes should bump the patch version.
- `Cargo.lock` is committed intentionally for reproducible CI, security checks, and release dry runs in this library workspace.
- Before any newly reviewed crate is made publishable, it should have stable naming, README coverage, unit tests, and changelog notes.

`release-plz` now drives release PRs and changelog generation for the publishable crates. Keep commit messages predictable:

- Prefer conventional prefixes such as `feat:`, `fix:`, `docs:`, `refactor:`, `build:`, `ci:`, `test:`, and `chore:`.
- Use `!` or a `BREAKING CHANGE:` footer for public API breaks that should bump the `0.x` minor version.
- Add a `changelog: ignore` footer only when a commit truly should stay out of release notes.

## Release Checklist

1. Run `cargo fmt`.
2. Run `cargo check --workspace --all-features`.
3. Run `cargo check --workspace --all-features --examples`.
4. Run `cargo clippy --workspace --all-targets --all-features -- -D warnings`.
5. Run `cargo deny check` and `cargo audit`.
6. Run `cargo test --workspace --all-features`.
7. Run `cargo test --workspace --no-default-features`.
8. Run `cargo doc --workspace --all-features --no-deps`.
9. Update `CHANGELOG.md`, `Cargo.lock`, and any affected README examples.
10. Confirm branch protection on `main` requires `Publish Readiness / Release Readiness Checks` before the first public release.
11. Review whether the crate should remain unpublished or become part of the first public release wave.
