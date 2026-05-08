## Summary

Describe the change and why it is needed.

## Testing

- [ ] `cargo fmt --all -- --check`
- [ ] `cargo check --workspace --all-features`
- [ ] `cargo test --workspace --all-features`
- [ ] `cargo test --workspace --no-default-features`
- [ ] `cargo clippy --workspace --all-targets --all-features -- -D warnings`

## Documentation

- [ ] README or crate docs updated if public behavior changed
- [ ] CHANGELOG updated if this should be part of the next release notes

## Release and review notes

- [ ] Public API change reviewed
- [ ] Feature flags reviewed
- [ ] Mirror provenance preserved when porting external contributions
