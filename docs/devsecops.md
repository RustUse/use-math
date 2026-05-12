# DevSecOps Foundation

This repository uses a small set of GitHub Actions workflows to keep Rust CI,
dependency hygiene, static analysis, secret detection, and SBOM generation in a
reviewable state for the first public release wave.

The baseline is intentionally pragmatic:

- required pull request checks stay focused on build, lint, tests, dependency risk, secret scanning, and repository scans
- scheduled jobs keep dependency and static analysis data fresh without changing local development requirements
- local development still works with plain Cargo commands; the `Makefile` is a convenience layer, not a new requirement

## Workflows

### `CI`

File: `.github/workflows/ci.yml`

- Triggers on pull requests to `main` and pushes to `main`
- Uses the repository toolchain from `rust-toolchain.toml`, which is currently stable Rust `1.95.0`
- Runs `cargo fmt --all -- --check`
- Runs `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- Runs `cargo test --workspace --all-features`
- Runs `cargo build --workspace --all-features`

### `Pull Request Quality Gate`

File: `.github/workflows/pull-request.yml`

- Triggers on every pull request
- Cancels outdated pull request runs with workflow concurrency
- Re-runs the formatting, clippy, test, and build checks as an explicit PR gate
- Keeps one readable required status check for merge policy

### `Cargo Audit`

File: `.github/workflows/cargo-audit.yml`

- Triggers on pull requests, pushes to `main`, and weekly schedule
- Installs `cargo-audit`
- Runs `cargo audit`
- Fails on known vulnerable dependencies reported through the RustSec advisory database

### `Cargo Deny`

File: `.github/workflows/cargo-deny.yml`

- Triggers on pull requests, pushes to `main`, and weekly schedule
- Installs `cargo-deny`
- Runs `cargo deny check`
- Enforces advisories, allowed licenses, crate source policy, wildcard bans, and duplicate-version visibility

### `CodeQL`

File: `.github/workflows/codeql.yml`

- Triggers on pull requests, pushes to `main`, and weekly schedule
- Runs GitHub CodeQL for Rust
- Builds the workspace with all features enabled before analysis
- Uploads results to GitHub code scanning
- The job is currently guarded to run only for public repositories, because GitHub documents Rust CodeQL support but private repositories still require code scanning availability in the repository environment

### `Gitleaks`

File: `.github/workflows/secrets.yml`

- Triggers on pull requests and pushes to `main`
- Checks the full Git history fetched by `actions/checkout@v4` with `fetch-depth: 0`
- Uses `gitleaks/gitleaks-action@v2`
- Keeps permissions minimal and disables PR comments to reduce noise

### `Trivy`

File: `.github/workflows/trivy.yml`

- Triggers on pull requests and pushes to `main`
- Scans the repository filesystem in `fs` mode
- Includes vulnerability and misconfiguration scanners
- Produces a SARIF report for `HIGH` and `CRITICAL` findings when GitHub code scanning upload is available
- Fails the workflow only on `CRITICAL` findings for `v0.1.0`

### `SBOM`

File: `.github/workflows/sbom.yml`

- Triggers on pushes to `main` and on manual `workflow_dispatch`
- Installs `cargo-cyclonedx`
- Generates a CycloneDX JSON SBOM from `crates/use-math/Cargo.toml` with all features enabled
- Uploads the resulting artifact as `sbom.cyclonedx.json`
- Does not commit generated SBOM output back into the repository

### `Advisory Rust Quality`

File: `.github/workflows/advisory-rust-quality.yml`

- Triggers on pull requests to `main`
- Runs `cargo-machete` in advisory mode to flag potentially unused dependencies
- Runs `cargo-semver-checks` in advisory mode against the pull request base revision for each publishable crate
- Uses `continue-on-error` so these checks stay informative during the `v0.1.0` hardening phase instead of blocking merges

### `Publish Readiness`

File: `.github/workflows/publish-readiness.yml`

- Triggers on pull requests to `main`, pushes to `main`, and manual dispatch
- Compiles workspace examples with all features enabled
- Tests the workspace without default features so minimal consumer paths stay healthy
- Runs `cargo publish --dry-run --allow-dirty` for every focused crate in the workspace
- Intentionally does not dry-run `use-math` yet, because the facade crate cannot complete publish validation until matching focused-crate versions are available in the crates.io index

### `Facade Publish Readiness`

File: `.github/workflows/facade-publish-readiness.yml`

- Triggers on manual dispatch only
- Exists for the post-publication window where the focused crates are already visible on crates.io
- Verifies that every focused crate already resolves from crates.io before continuing
- Lists the packaged `use-math` files and then runs `cargo publish --dry-run --allow-dirty -p use-math`
- Should stay manual because it is expected to fail before registry propagation completes

### `Release PR Automation`

File: `.github/workflows/release-plz-pr.yml`

- Triggers on pushes to `main` and on manual dispatch
- Runs `release-plz release-pr` to prepare lockstep version bumps and changelog updates
- Uses `release-plz.toml` to keep every publishable crate in one version group
- Keeps release publishing separate from version/changelog preparation

### `Release Publish Automation`

File: `.github/workflows/release-plz-release.yml`

- Triggers automatically on pushes to `main` after `CRATES_IO_AUTOPUBLISH_ENABLED=true`, and still supports manual dispatch for reruns
- Runs `release-plz release` for post-initial-release publishing
- Uses GitHub OIDC trusted publishing, so no long-lived `CARGO_REGISTRY_TOKEN` secret is needed in the workflow
- Verifies that all published crates already resolve from crates.io before attempting the automated release step
- Should still be treated as post-initial-release automation; crates.io requires the first publish of new crates to happen outside the trusted-publishing path

## Which Checks Block Pull Requests

Required checks before the first public release are:

- `CI / Rust Workspace CI`
- `Pull Request Quality Gate / PR Quality Checks`
- `Publish Readiness / Release Readiness Checks`
- `Cargo Audit / Dependency Vulnerability Audit`
- `Cargo Deny / Dependency Policy Check`
- `Gitleaks / Secret Scan`
- `Trivy / Filesystem Vulnerability and Misconfiguration Scan`
- `CodeQL / CodeQL Analysis` only when the CodeQL job is available and running in the repository environment

`SBOM / Generate CycloneDX SBOM` is not a required pull request check. It is a `main` branch and manual artifact workflow.

`Advisory Rust Quality` is also intentionally non-blocking for now.

The repository cannot enforce GitHub branch protection from source files alone. Maintainers still need to apply the required-check rule in the GitHub UI or ruleset configuration.

## Local Commands

The fastest local path is still plain Cargo:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo check --workspace --all-features --examples
cargo test --workspace --all-features
cargo test --workspace --no-default-features
cargo build --workspace --all-features
```

The repository also provides a `Makefile` with these convenience commands:

- `make fmt`
- `make lint`
- `make test`
- `make test-minimal`
- `make build`
- `make examples`
- `make audit`
- `make deny`
- `make sbom`
- `make publish-dry-run-focused`
- `make publish-dry-run-facade`
- `make release-readiness`
- `make facade-post-publish-validation`
- `make verify`

`make verify` intentionally runs only the core Rust validation path:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo build --workspace --all-features
```

`make release-readiness` extends that path with example compilation, no-default-features coverage, and focused-crate publish dry-runs:

```bash
cargo check --workspace --all-features --examples
cargo test --workspace --no-default-features
make publish-dry-run-focused
```

The post-publication facade check is intentionally separate:

```bash
cargo publish --dry-run --allow-dirty -p use-math
```

Run it only after matching focused-crate versions are visible on crates.io.

`make audit`, `make deny`, and `make sbom` assume the corresponding tools are installed locally.

Suggested local installs:

```bash
cargo install cargo-audit cargo-deny cargo-cyclonedx --locked
```

Optional advisory installs:

```bash
cargo install cargo-machete cargo-semver-checks --locked
```

Optional release automation install:

```bash
cargo install release-plz --locked
```

Optional local equivalents for the CI-first scanners:

```bash
gitleaks dir .
trivy fs --scanners vuln,misconfig --severity HIGH,CRITICAL .
```

Optional advisory commands:

```bash
cargo machete --with-metadata
cargo semver-checks --manifest-path crates/use-geometry/Cargo.toml --baseline-rev <BASE_REV> --all-features
cargo semver-checks --manifest-path crates/use-combinatorics/Cargo.toml --baseline-rev <BASE_REV> --all-features
cargo semver-checks --manifest-path crates/use-math/Cargo.toml --baseline-rev <BASE_REV> --all-features
```

Optional release automation preview commands:

```bash
release-plz release-pr --config release-plz.toml
release-plz release --config release-plz.toml
```

## How `cargo-audit` Works

`cargo-audit` compares the resolved dependency graph from `Cargo.lock` against
the RustSec advisory database.

- It is best at catching known vulnerable crates and versions.
- It does not replace code review, API hardening, or threat modeling.
- A failure usually means you need to upgrade, patch, replace, or temporarily remove a dependency.

If a result is not actionable yet, document the risk and planned remediation in the pull request before merging any temporary exception.

## How `cargo-deny` Works

`cargo-deny` enforces broader supply-chain policy than `cargo-audit` alone.

This repository currently uses it to:

- deny known vulnerable crates
- skip unmaintained advisories and warn on yanked crates
- allow a pragmatic set of permissive licenses
- deny unknown licenses unless clarified
- deny unknown registries and git sources
- warn about duplicate crate versions instead of failing the build on day one

Unmaintained advisories are intentionally non-blocking in CI because the
upstream RustSec maintenance flag can be noisy during ecosystem transitions.
Maintainers should still review unmaintained signals during dependency upgrades
and release-readiness checks.

Policy lives in `deny.toml`. Any exception should be narrow, documented, and reviewed.

## How SBOM Generation Works

The current SBOM workflow is intentionally scoped to the publishable `use-math`
facade crate with all features enabled.

Workflow command:

```bash
cargo cyclonedx \
  --manifest-path crates/use-math/Cargo.toml \
  --all-features \
  --format json \
  --spec-version 1.5 \
  --override-filename sbom.cyclonedx
```

The generated file is copied to the repository root in CI and uploaded as the
artifact `sbom.cyclonedx.json`.

Local generation via `make sbom` writes the file under `crates/use-math/`.

If the repository later needs per-crate SBOMs for all publishable workspace
members, add a dedicated follow-up workflow rather than overloading this one.

## Handling False Positives

### `cargo-audit`

- Confirm the advisory actually affects the compiled dependency path in this workspace.
- Check whether the vulnerable feature set is enabled.
- Prefer upgrading or removing the dependency before adding any exception.

### `cargo-deny`

- If a license or source is flagged incorrectly, confirm the package metadata first.
- If an exception is justified, add the narrowest possible change in `deny.toml` and explain it in the pull request.

### `Gitleaks`

- Confirm whether the match is a real credential, a test fixture, or a public example token.
- If it is a recurring benign pattern, extend `.gitleaks.toml` with the narrowest rule-scoped allowlist you can justify instead of disabling the scanner broadly.
- Prefer `gitleaks:allow` only for explicit inline test fixtures that are clearly meant to remain in the repository.

### `Trivy`

- Confirm whether the finding is in an active dependency, generated cache, or an example artifact.
- Fix genuine issues first; add reviewed finding IDs to `.trivyignore` only when the risk is understood and documented.

### `CodeQL`

- Review the alert path and data flow, not just the headline.
- If the result is not actionable, capture the reasoning in the pull request or security discussion and keep the suppression narrow.

## If Gitleaks Detects a Leaked Secret

Treat any real secret leak as an incident.

1. Revoke or rotate the secret at the provider immediately.
2. Replace the secret in GitHub Actions secrets, local environment stores, or deployment systems.
3. Audit recent use of the leaked credential.
4. Remove the leaked value from the repository history when appropriate.
5. Re-run the secret scan and document the remediation.

Removing the string from the latest commit is not enough on its own. Rotation matters because Git history may still contain the leaked value.

## What Must Pass Before Merging to `main`

At minimum:

- required pull request checks are green
- review comments are resolved
- the branch is up to date with `main`
- any dependency or security exception is documented in the pull request
- public API or workflow changes include documentation updates when needed

## Branch Protection for `main`

Recommended GitHub branch protection settings:

- Require a pull request before merging
- Require status checks to pass before merging
- Require branches to be up to date before merging
- Require conversation resolution before merging
- Dismiss stale approvals when new commits are pushed
- Restrict direct pushes to `main`

Recommended required status checks:

- `CI / Rust Workspace CI`
- `Pull Request Quality Gate / PR Quality Checks`
- `Publish Readiness / Release Readiness Checks`
- `Cargo Audit / Dependency Vulnerability Audit`
- `Cargo Deny / Dependency Policy Check`
- `Gitleaks / Secret Scan`
- `Trivy / Filesystem Vulnerability and Misconfiguration Scan`
- `CodeQL / CodeQL Analysis` only if the CodeQL job is available and working for this repository

Optional advisory checks that can stay unrequired until the repository wants stricter automation:

- `Advisory Rust Quality / Cargo Machete Advisory`
- `Advisory Rust Quality / Cargo SemVer Checks Advisory`

Manual release-maintainer workflows that should not be required pull request checks:

- `Facade Publish Readiness / Facade Publish Dry Run`
- `Release PR Automation / Release-plz PR`
- `Release Publish Automation / Release-plz Release`

## Notes and Assumptions

- The repository toolchain is pinned in `rust-toolchain.toml`, and it is currently stable Rust `1.95.0`.
- CodeQL supports Rust on GitHub, but private repositories still need code scanning availability in their GitHub environment before this should become a required check.
- `gitleaks/gitleaks-action@v2` may require a Gitleaks license for organization-owned repositories. If this repository is moved under an organization and the job starts failing for licensing reasons, add the required repository or organization secret and keep the workflow name unchanged.
- `.gitleaks.toml` extends the default Gitleaks ruleset without suppressing any findings yet; it exists so future reviewed false positives can be handled narrowly.
- `.trivyignore` is intentionally empty and repository-owned; add only reviewed finding IDs and keep each suppression documented in the pull request that introduces it.
- `Publish Readiness` intentionally validates the focused crates first; the final `use-math` facade publish dry-run stays a post-publication release step until matching focused-crate versions resolve from crates.io.
- `release-plz.toml` is configured for one workspace version group and a single root changelog owned by the `use-math` entry, with focused-crate commits included in the shared release notes.
