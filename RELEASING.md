# Releasing

This repository uses a specialized first-wave release flow rather than the
single-crate manual publish pattern used by some other RustUse repos.

## Current release state

`use-math` publishes a first wave of focused crates before the `use-math`
facade crate.

## Canonical release guide

Use [RELEASE.md](RELEASE.md) as the authoritative release policy for:

- first-wave publish scope
- focused-crate publish ordering
- publish readiness checks
- trusted publishing setup after the first public wave
- maintainer release checklist

If the repository history is being reset before the first public push, follow
the additional documents referenced from `RELEASE.md`.

## Current automation

The repository already includes the specialized workflows that match this
release shape:

- `publish-readiness.yml`
- `facade-publish-readiness.yml`
- `release-plz-pr.yml`
- `release-plz-release.yml`

This file exists to keep the top-level release entrypoint consistent with the
other RustUse repositories while preserving the more detailed custom guidance
in `RELEASE.md`.
