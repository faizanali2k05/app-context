GLOBAL RUST CRATE QUALITY STANDARD

You are building a production-quality open-source Rust crate intended for publication on crates.io.

Do not produce a demo, prototype, tutorial implementation, placeholder architecture, or TODO-heavy repository.

ENGINEERING

1. Use stable Rust.
2. Use Rust 2024 edition unless a documented compatibility reason prevents it.
3. Choose a realistic MSRV and declare `rust-version` in Cargo.toml.
4. Test MSRV in CI.
5. Minimize dependencies and default features.
6. Feature-gate optional integrations.
7. Avoid unsafe Rust. If unsafe is required:
   - isolate it
   - document the safety invariant
   - test it
   - run Miri where applicable.
8. No unwrap()/expect() in reusable library paths unless logically impossible and justified.
9. Provide structured error types using thiserror or equivalent where appropriate.
10. APIs must be idiomatic Rust and designed before implementation.

QUALITY

The repository must pass:

cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
cargo test --doc
cargo doc --all-features --no-deps

Add where relevant:
- cargo-deny
- cargo-audit
- cargo-semver-checks
- Miri
- fuzzing
- property tests
- Criterion benchmarks

DOCUMENTATION

All public API must have rustdoc documentation.

The repository must contain:

README.md
CHANGELOG.md
LICENSE-MIT
LICENSE-APACHE
CONTRIBUTING.md
SECURITY.md
CODE_OF_CONDUCT.md
docs/architecture.md
docs/design-decisions.md
docs/security.md
docs/performance.md

README must include:
- what problem this solves
- when to use it
- when NOT to use it
- installation
- 30-second quick start
- complete example
- feature flags
- MSRV
- platform support
- security notes
- performance notes
- alternatives/comparison
- stability policy

Examples shown in README and rustdoc must compile.

TESTING

Include:
- unit tests
- integration tests
- edge cases
- error paths
- regression tests

Where parsers/protocols/security are involved:
- property testing
- fuzz targets

CI

GitHub Actions matrix covering relevant:
- Linux
- Windows
- macOS

Run:
format
clippy
tests
doctests
MSRV tests
documentation build
security/dependency checks

RELEASE

Cargo.toml must contain:
description
repository
documentation
homepage if appropriate
keywords
categories
license
readme
rust-version

Configure docs.rs metadata correctly.

Use semantic versioning.

Do not publish generated junk, test fixtures unnecessarily, large assets or development-only files in the crate package.

Use `cargo package --list` and `cargo package` as release checks.

PERFORMANCE

Do not state that something is “fast”, “zero-cost”, “lock-free”, “constant-time”, etc. unless the implementation and benchmarks justify the statement.

SECURITY

Do not make security guarantees stronger than the implementation provides.

Document threat models and known limitations.

DESIGN PROCESS

Before coding:
1. Write docs/design-decisions.md.
2. Define public API.
3. Identify invariants.
4. Identify failure modes.
5. Design test strategy.

Then implement.

After implementation:
1. run every quality command
2. repair all warnings/errors
3. review public API for unnecessary exposure
4. review dependencies
5. review docs against actual behavior
6. ensure examples compile
7. remove TODOs/placeholders
8. produce RELEASE_CHECKLIST.md

The final repository should be credible enough that an experienced Rust developer could inspect it, understand it, test it and consider depending on it.

