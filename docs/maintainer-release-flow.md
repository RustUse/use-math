# Maintainer Release Flow

This document describes how maintainers should run releases with the current
`release-plz` setup.

It covers two different paths:

- the initial public crates.io wave, which is still manual
- normal follow-up releases, where version bumps and changelog generation are automated and publishing stays maintainer-triggered

## Current model

- `Release PR Automation` opens or updates a release PR from `main`.
- `release-plz` keeps `use-geometry`, `use-combinatorics`, and `use-math` in one lockstep version group.
- The shared root `CHANGELOG.md` is generated through the `use-math` package entry and includes focused-crate commits.
- `Release Publish Automation` stays manual and should be used only after the initial manual publish wave is complete.

## How changelog generation works

The current parser rules map strict conventional-commit style subjects into
these changelog groups:

- `feat:` -> `Added`
- `fix:` -> `Fixed`
- `security:` -> `Security`
- `refactor:`, `perf:`, `change:` -> `Changed`
- `docs:` -> `Documentation`
- `build:`, `ci:`, `chore:`, `deps:`, `test:` -> `Tooling`
- `changelog: ignore` footer -> skipped from release notes

The intended subject shapes are:

- `type: summary`
- `type(scope): summary`
- `type!: summary`
- `type(scope)!: summary`

Breaking changes should use `!` in the subject or a `BREAKING CHANGE:` footer.

Any commit that does not match one of the explicit parser groups still lands in
`Changed`. That is intentional because it prevents real work from disappearing
from release notes, but it also means vague subjects create vague release notes.

## What is strict enough for clean release notes?

The current rules are good enough if maintainers enforce them consistently.

They are stricter than before, but they are still not fully self-enforcing
because the fallback `Changed` parser remains enabled. In practice, that means:

- release notes stay complete even when a commit subject is imperfect
- release note quality still depends on PR titles and squash-merge commit subjects
- maintainers should reject vague titles such as `updates`, `fix stuff`, or `misc cleanup`

The simplest operating rule is: treat the PR title as the future release note
line if the PR will be squash-merged.

## Preferred commit and PR title examples

- `feat: add triangle centroid helper`
- `fix: reject non-finite slope inputs`
- `docs: clarify facade feature flags`
- `refactor: simplify aabb validation path`
- `build: pin cargo-deny for local parity`
- `security: harden publish workflow gating`

## Normal post-initial-release flow

Use this flow after the first public crates.io wave already exists.

1. Merge ordinary PRs into `main` with clean conventional commit style in the final commit subject or squash-merge title.
2. Let `Release PR Automation` open or update the release PR.
3. Review the release PR for three things:
    - the lockstep version bump across all publishable crates
    - the generated root `CHANGELOG.md`
    - any low-signal fallback entries in `Changed`
4. If the generated changelog needs cleanup, edit the changelog directly in the release PR branch before merging.
5. Merge the release PR into `main`.
6. Confirm the push-triggered release-readiness and security checks are green on the merged release commit.
7. Manually dispatch `Release Publish Automation` with `post-initial-release = true`.
8. Verify the published crates, docs.rs pages, and repository tag or release artifacts after the workflow completes.

## Initial public release exception

Do not use `Release Publish Automation` for the first public crates.io wave.

Use the manual dependency-ordered publish path instead:

1. Confirm `use-geometry`, `use-combinatorics`, and `use-math` are still the only intended first-wave publishable crates.
2. Run the full publish-readiness checks.
3. Publish `use-geometry`.
4. Publish `use-combinatorics`.
5. Wait for crates.io index propagation.
6. Run `cargo publish --dry-run -p use-math` or the manual `Facade Publish Readiness` workflow.
7. Publish `use-math`.

After that first wave is complete, the manual `Release Publish Automation`
workflow becomes the path for subsequent releases.

## Maintainer review checklist for every release PR

- The version bump is still lockstep across the three publishable crates.
- The root changelog reads cleanly without vague fallback entries.
- Any intentionally skipped commits actually carry `changelog: ignore` for a good reason.
- The release still matches the current publish surface and feature model.
- The publish workflow is being used in the correct phase: manual first wave versus post-initial-release automation.
