# History Reset And Republishing Runbook

This runbook assumes the current private GitHub repository stays in place and
you replace its commit history with one clean public root commit before the
first public launch.

If you create a brand new repository instead of force-pushing the existing one,
use this sequence as the baseline but treat the GitHub settings and secrets
steps as a full reconfiguration rather than a verification pass.

## Preconditions

- The working tree already contains exactly the files you want to publish.
- The repo-owned DX and release files are present, including `.cargo/config.toml`, `.github/workflows/`, `release-plz.toml`, `RELEASE.md`, and `docs/first-public-commit.md`.
- You have admin access to `RustUse/use-math`.
- You have a crates.io account ready for the first manual publish wave.

## Quick command sequence

Use this when you want the shortest day-of-reset operator path and you have
already read the detailed steps below.

Replace `YYYYMMDD` with the actual launch date before running the backup
commands.

```bash
git switch main
git status --short
cargo xcheck
cargo xlint
cargo xtest
cargo xtest-minimal
cargo xexamples
bash scripts/bootstrap-dev-tools.sh --dry-run
pwsh -File scripts/bootstrap-dev-tools.ps1 -DryRun
git branch backup/private-main-YYYYMMDD
git tag private-pre-public-YYYYMMDD
git bundle create ../use-math-private-history-YYYYMMDD.bundle --all
git switch --orphan public-main
git add -A
git status --short
git commit -m "chore: initialize public use-math workspace"
git branch -M main
git push --force-with-lease origin main
```

After the clean public push, verify GitHub settings manually, then run the
release-readiness and first-publish sequence:

```bash
make release-readiness
for crate in use-arithmetic use-number use-integer use-modular use-prime use-polynomial use-equation use-rational use-interval use-real use-complex use-geometry use-combinatorics use-series use-catalan use-geode use-algebra use-vector use-matrix use-linear use-calculus use-probability use-statistics use-trigonometry use-logic use-set; do
	cargo publish -p "$crate"
done
cargo publish --dry-run -p use-math
cargo publish -p use-math
```

## 1. Freeze the public tree locally

Run these commands from the repository root and confirm the tree is the one you
intend to expose publicly.

```bash
git switch main
git status --short
cargo xcheck
cargo xlint
cargo xtest
cargo xtest-minimal
cargo xexamples
bash scripts/bootstrap-dev-tools.sh --dry-run
pwsh -File scripts/bootstrap-dev-tools.ps1 -DryRun
```

If the devcontainer is part of the public onboarding promise, validate it too:

```bash
npx -y @devcontainers/cli up --workspace-folder .
```

## 2. Create a private history backup before rewriting anything

Create both a movable branch backup and an immutable bundle backup.

```bash
git branch backup/private-main-YYYYMMDD
git tag private-pre-public-YYYYMMDD
git bundle create ../use-math-private-history-YYYYMMDD.bundle --all
```

Keep the bundle outside the repository directory and store it somewhere you will
not delete during the public launch cleanup.

## 3. Confirm branch protection will not block the reset

If the current private repository already blocks force-pushes to `main`, either:

- perform the history reset before enabling the final branch protection rules, or
- temporarily relax the rule, perform the reset, then re-enable protection immediately after the new public root commit is on GitHub.

Do not leave `main` permanently force-pushable after the reset is complete.

## 4. Create the clean public root commit

Create a new orphan branch from the validated working tree and commit the public
snapshot as one root commit.

```bash
git switch --orphan public-main
git add -A
git status --short
git commit -m "chore: initialize public use-math workspace"
```

The `git status --short` output should contain only the files you intend to
ship publicly.

## 5. Replace the remote `main` history

Rename the orphan branch back to `main` locally, then force-push it to the
existing GitHub repository.

```bash
git branch -M main
git push --force-with-lease origin main
```

At this point the same GitHub repository should have a clean one-commit public
history, while your local backup branch, tag, and bundle still preserve the
private history if you ever need to recover it.

## 6. Verify GitHub settings on the existing repository

Because this runbook assumes the same GitHub repository survives the reset,
most repository settings should persist. Verify them immediately after the
force-push.

- Confirm the repository visibility is now public when you are ready.
- Enable Discussions if you want the issue chooser to route questions there immediately.
- Confirm Actions, Dependabot, CodeQL, code scanning, secret scanning, and issue forms are still enabled as intended.
- Reapply or tighten branch protection so `Publish Readiness / Release Readiness Checks` is required on `main` before the first public release.
- Reconfirm any repository secrets or variables required for mirrors or future release automation.

## 7. Verify the public repository surfaces

After the public push, check the repository as a user would see it.

- Confirm the README renders correctly.
- Confirm the issue chooser shows the intended forms and contact links.
- Confirm the organization-level support and security defaults route people to
  the right channels.
- Confirm the devcontainer and VS Code recommendations still present a clean onboarding path.
- Confirm the expected required checks appear on pull requests.

## 8. Run the first release-readiness pass from the clean history

Before publishing anything to crates.io, rerun the intended local release path
from the clean public history.

```bash
make release-readiness
```

That should cover:

- workspace validation
- example compilation
- no-default-features coverage
- focused-crate dry-run publish coverage across the full set

The facade crate still needs to wait until the focused crates are visible in the
crates.io index.

## 9. Publish the first crates.io wave manually

For the first release wave, do not use `release-plz release`. Publish in manual
dependency order.

```bash
for crate in use-arithmetic use-number use-integer use-modular use-prime use-polynomial use-equation use-rational use-interval use-real use-complex use-geometry use-combinatorics use-series use-catalan use-geode use-algebra use-vector use-matrix use-linear use-calculus use-probability use-statistics use-trigonometry use-logic use-set; do
	cargo publish -p "$crate"
done
```

Wait for crates.io index propagation, then verify the facade crate resolves
correctly:

```bash
cargo publish --dry-run -p use-math
```

If that dry-run passes, publish the facade crate:

```bash
cargo publish -p use-math
```

You can also trigger `.github/workflows/facade-publish-readiness.yml` as the
post-publication verification gate once the focused crates are live.

## 10. Finish the first public release state

After the focused crates and facade are live:

- Confirm docs.rs builds and crate metadata render correctly.
- Push the release tag if you want the repository history to carry the first public version explicitly.
- Keep using `release-plz` only for subsequent releases, after the initial manual publish wave is complete.
