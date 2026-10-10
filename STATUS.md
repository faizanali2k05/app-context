# Status

Implementation complete; release checks in progress. Publication remains disabled.

## Implemented

- Dependency-free immutable typed Context/ContextBuilder with structured errors.
- Arc identity-preserving insertion/retrieval, borrowed reads, extraction and scopes.
- Derive crate: named/tuple/unit structs, generics/nested state, clone/default fields,
  dependency renaming, targeted compile diagnostics.
- Optional Axum/Actix extractors and Tonic interceptor/extraction.
- Five required examples, complete rustdoc, dual license texts and project policies.
- Linux/macOS/Windows stable/MSRV CI, feature builds and security workflows.
- Retrieval, cloning, scoping and initialization Criterion benchmarks.

## Verified locally (Linux, stable Rust 1.99)

- cargo clippy --workspace --all-targets --all-features --offline -- -D warnings.
- cargo test --workspace --all-features --offline: 2 unit, 12 core, 3 derive,
  8 framework tests, 8 compile-fail cases plus generic compile-pass and 6 doctests.
- cargo fmt --all --check.
- Strict documentation build and individual feature builds passed.
- Default core normal dependency tree contains only app-context itself.
- README code examples are doctests. Renamed dependency consumer actually runs.
- Repository URL verified: https://github.com/faizanali2k05/app-context.

- Rust 1.88 MSRV: all-feature unit/core/derive/framework tests, doctests and example builds passed.
- All five examples ran successfully; 13 additional fmt/docs/feature/example checks passed.
- Criterion measurements completed; see docs/BENCHMARK_RESULTS.md.
- Derive archive built and package contents reviewed in an isolated source snapshot.

- cargo-audit 0.22.2: no known vulnerabilities or warnings in 171 lockfile packages,
  using official RustSec revision 7eebec69c352c7191b1f13eb95dd510eeca5d1de (2026-10-09).
- cargo-deny 0.20.2: advisories, licenses, bans and sources passed with all features.
  Expected duplicate-version warnings remain for framework dependencies http and syn.
- Derive cargo publish --dry-run passed in the isolated candidate directory; upload
  explicitly aborted by Cargo as a dry run. The working manifests remain disabled.

No crates have been published.
Windows/macOS CI has not run; the repository's existing workflow run list is empty.
Core registry packaging must follow derive publication. A local verification commit is prepared; it is not yet a published release. Publication remains disabled in both manifests.
