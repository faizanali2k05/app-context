# Release checklist

## Implementation and local checks

- [x] Every SPEC.md capability implemented, including optional adapters and examples.
- [x] Public API rustdoc, architecture/design/security/performance docs and policies.
- [x] Complete MIT and Apache-2.0 license texts included in each package.
- [x] fmt, Clippy with denied warnings, all-feature behavioral/UI tests and doctests.
- [x] Strict documentation build, example builds and all five runnable examples.
- [x] Default core has no normal dependencies; individual features build.
- [x] Retrieval/initialization benchmarks executed; measured limitations documented.
- [x] Real configured repository verified: https://github.com/faizanali2k05/app-context.
- [x] Package lists audited: no secrets, build artifacts, test fixtures or handoff files.
- [x] Derive archive built with cargo package in an isolated source snapshot.
- [x] MSRV 1.88 all-feature behavioral tests, doctests and examples passed.
- [x] cargo audit and cargo deny passed; only documented duplicate-version warnings.
- [x] Supported Linux/macOS/Windows platform CI passed for the final implementation.

## Publication gates

- [ ] Registry name/ownership rechecked immediately before publication.
- [x] Release metadata reviewed; publication deliberately enabled after source CI passed.
- [x] Derive cargo publish --dry-run verified in isolated candidate (no upload).
- [ ] Derive published as 0.1.0 after all required CI gates pass.
- [ ] Exact derive version visible through the crates.io registry.
- [ ] Core cargo package and cargo publish --dry-run verified with registry derive.
- [ ] Packaged core checked with derive/all features.
- [ ] Core published as 0.1.0; exact registry version and artifact confirmed.

All eight source CI jobs passed:
https://github.com/faizanali2k05/app-context/actions/runs/38060232430
Only publication flags and changelog/release records changed after source checks.
Release packaging will use a clean committed checkout and normal Cargo verification.
Existing Cargo authentication is configured; validity remains to be established by
publication. Neither crate has been uploaded yet.

Local cargo-deny 0.20.2 was obtained from its official GitHub release and verified
against SHA-256 9f12ed4c49936e09b48bf862b595cde2fe64fcbd9d74dfacac6131ca824c8d5f.
No audit warnings or vulnerabilities were ignored. Configured duplicate-version
warnings only concern framework dependencies http and syn.
