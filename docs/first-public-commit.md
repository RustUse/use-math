# First Public Commit Checklist

This checklist exists for the clean-history open-source launch path.

Use it if the repository history is reset before the first public push so the
new root commit still preserves the important release, tooling, and governance
decisions already made in this workspace.

## Keep in the first public revision

- Keep the repository health files that stay local: `CONTRIBUTING.md`,
  `GOVERNANCE.md`, and `MAINTAINERS.md`, and rely on the RustUse
  organization-level defaults for code of conduct, security, and support.
- Keep the Rust workspace defaults in `Cargo.toml`, including `publish = false` at the workspace level and the intentional first-wave crate publish settings in the crate manifests.
- Keep `release-plz.toml`, `CHANGELOG.md`, and `RELEASE.md` so lockstep versioning and the shared changelog policy survive the reset.
- Keep the repo-owned DX files: `.cargo/config.toml`, `.vscode/tasks.json`, `.vscode/extensions.json`, `.devcontainer/devcontainer.json`, `.devcontainer/post-create.sh`, and the bootstrap scripts under `scripts/`.
- Keep the CI and security workflows under `.github/workflows/`.
- Keep mirror documentation and dormant mirror automation only if GitHub remains canonical after the reset.

See `docs/history-reset-and-republish.md` for the exact operator sequence when
you are ready to rewrite `main` and open the repository.

## Verify in GitHub after the reset

- Confirm the canonical repository is still `RustUse/use-math` and `main` remains the default branch.
- If you kept the same GitHub repository, verify the existing settings survived the force-push and re-enable anything you intentionally relaxed for the reset.
- If you recreated the repository instead of force-pushing it, reapply branch protection, Actions settings, code scanning, issue forms, Discussions, variables, and secrets.
- Enable GitHub Discussions if you want the issue chooser to route questions there immediately.
- Confirm `Publish Readiness / Release Readiness Checks` is required on `main` before the first public release.
- Reconfirm any repository variables or secrets that are intentionally not stored in git, such as mirror URLs, mirror SSH keys, and future publish credentials.

## Validate the clean-slate repo locally

- Run `cargo xcheck`.
- Run `cargo xlint`.
- Run `cargo xtest`.
- Run `cargo xtest-minimal`.
- Run `cargo xexamples`.
- Run `cargo xdoc` when public docs changed.
- Run `bash scripts/bootstrap-dev-tools.sh --dry-run` and `pwsh -File scripts/bootstrap-dev-tools.ps1 -DryRun` if the bootstrap scripts changed.
- Run `npx -y @devcontainers/cli up --workspace-folder .` if the devcontainer changed or if you want to validate the public onboarding path before launch.

## Preserve the first-release constraints

- Keep every focused crate under `crates/` plus `use-math` as the first-wave publishable set.
- Publish all focused crates first, wait for crates.io index propagation, then publish `use-math`.
- Treat `.github/workflows/facade-publish-readiness.yml` as the post-publication facade gate only after the focused crates resolve from crates.io.
- Treat `.github/workflows/release-plz-release.yml` as post-initial-release automation only. The first publish wave still needs the manual dependency-ordered path.
- Keep the shared root changelog and lockstep version group unless you intentionally change the release model.

## Public-facing review before the first push

- Confirm repository URLs, contact links, and support/security addresses are correct.
- Confirm issue forms, PR template, and support routing still match whether Discussions is enabled.
- Confirm README examples and crate metadata describe only the surfaces you intend to publish.
- Confirm no local paths, personal machine details, or temporary launch notes remain in tracked files.
- Confirm mirrors are either intentionally dormant or fully configured.

## After the first public push

- Verify the README renders correctly on GitHub.
- Verify the issue chooser shows the intended forms and contact links.
- Verify the devcontainer and VS Code recommendations still present a clean onboarding path.
- Verify required status checks appear with the expected names on pull requests.
- Verify the release-readiness workflow still passes before attempting the first crates.io publish.
