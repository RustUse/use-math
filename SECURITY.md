# Security Policy

RustUse accepts private reports for vulnerabilities affecting this workspace,
its published crates, and the canonical release path.

## Supported versions

| Surface | Status |
| ------- | ------ |
| `main` | Supported on a best-effort basis until superseded |
| Latest published `0.0.x` release line | Supported |
| Older published `0.0.x` releases | Not guaranteed |
| Unofficial forks or mirrors | Best effort after canonical triage |

## Reporting a vulnerability

Report vulnerabilities privately to ferris@rustuse.org.

- Do not open a public GitHub issue for a suspected vulnerability.
- Include the affected crate, version, feature flags, and reproduction steps.
- Include impact details, proof of concept, or a minimized test case when
  available.
- If you already have a fix, you may attach a patch or private branch
  reference.

## Response expectations

- Acknowledge receipt within 3 business days.
- Share an initial triage update within 7 business days when enough context is
  available.
- Coordinate disclosure after a fix, mitigation, or public advisory is ready.

## Scope notes

Security fixes and disclosures are coordinated from the canonical GitHub
repository. Mirror infrastructure is not release authority and must not receive
publish credentials.
