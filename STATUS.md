# Status

Implementation complete; all source release checks passed. Registry publication in progress.

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
  explicitly aborted by Cargo as a dry run. Publication was enabled after all required source CI checks passed.

## Release progress

- User authorized uploading the completed source to the existing GitHub repository.
- GitHub source commit f218ee2f2c74e1ab1f34b389122ad546a85feefb exactly matches
  the locally validated source tree 7f9cab64ec5f4e296244065cadd1f7db42d08380.
- All eight GitHub Actions jobs passed: stable/MSRV tests on Linux/macOS/Windows,
  formatting/Clippy/UI/docs/benchmarks and dependency/security checks.
  https://github.com/faizanali2k05/app-context/actions/runs/38060232430
- Only release metadata and changelog dates changed after those source checks.
- Both manifests intentionally enabled for publication after these gates passed.
- Pending: fresh registry name checks, final clean-source derive package/dry-run
  and publication, exact registry confirmation, core package/dry-run and publication.

No crates have been published yet. Cargo authentication validity is not assumed.
