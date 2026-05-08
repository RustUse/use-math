.PHONY: fmt lint test test-minimal build examples audit deny sbom publish-dry-run-focused publish-dry-run-facade release-readiness facade-post-publish-validation verify

fmt:
	cargo fmt --all -- --check

lint:
	cargo clippy --workspace --all-targets --all-features -- -D warnings

test:
	cargo test --workspace --all-features

test-minimal:
	cargo test --workspace --no-default-features

build:
	cargo build --workspace --all-features

examples:
	cargo check --workspace --all-features --examples

audit:
	cargo audit

deny:
	cargo deny check

sbom:
	cargo cyclonedx --manifest-path crates/use-math/Cargo.toml --all-features --format json --spec-version 1.5 --override-filename sbom.cyclonedx

publish-dry-run-focused:
	cargo publish --dry-run --allow-dirty -p use-geometry
	cargo publish --dry-run --allow-dirty -p use-combinatorics

publish-dry-run-facade:
	cargo publish --dry-run --allow-dirty -p use-math

release-readiness: verify examples test-minimal publish-dry-run-focused

facade-post-publish-validation: publish-dry-run-facade

verify: fmt lint test build
