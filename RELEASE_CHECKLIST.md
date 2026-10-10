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

- [x] Registry name/ownership rechecked immediately before publication.
- [x] Release metadata reviewed; publication deliberately enabled after source CI passed.
- [x] Derive cargo publish --dry-run verified in isolated candidate (no upload).
- [x] Derive published as 0.1.0 after all required CI gates pass.
- [x] Exact derive version visible through the crates.io registry.
- [x] Core cargo package and cargo publish --dry-run verified with registry derive.
- [x] Packaged core checked with derive/all features.
- [x] Core published as 0.1.0; exact registry version and artifact confirmed.

All eight release CI jobs passed for the exact published source commit:
https://github.com/faizanali2k05/app-context/actions/runs/38060656073
Commit: 3ebdeb312a6bb87dc080187ed373059ab033a9ac.

Both final archives were reviewed from a clean checkout; Cargo package and publish
verification remained enabled. Core all-feature packaging, 2 packaged unit tests,
6 packaged doctests, example builds and publish dry run passed using registry derive.
The integration/UI fixtures and benchmark sources stay in the repository; Cargo
reports their intentional exclusion from the published archive during packaging.

Published app-context-derive 0.1.0 first, waited for registry availability, then
published app-context 0.1.0. Both exact versions, downloadable artifacts, source
contents, clean VCS metadata and SHA-256 checksums independently verified. Machine
readable evidence is in docs/release-0.1.0.json. Existing Cargo authentication worked;
no credentials were printed, added to files or requested in chat.

Local cargo-deny 0.20.2 was obtained from its official GitHub release and verified
against SHA-256 9f12ed4c49936e09b48bf862b595cde2fe64fcbd9d74dfacac6131ca824c8d5f.
No audit warnings or vulnerabilities were ignored. Configured duplicate-version
warnings only concern framework dependencies http and syn.
