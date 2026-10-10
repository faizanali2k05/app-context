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
- [ ] Supported Linux/macOS/Windows platform CI has run for the final source revision.

## Publication gates

- [ ] Registry name/ownership rechecked immediately before publication.
- [ ] Final release source committed and clean; publication deliberately enabled.
- [x] Derive cargo publish --dry-run verified in isolated candidate (no upload).
- [ ] Derive published as 0.1.0 after all required CI gates pass.
- [ ] Exact derive version visible through the crates.io registry.
- [ ] Core cargo package and cargo publish --dry-run verified with registry derive.
- [ ] Packaged core checked with derive/all features.
- [ ] Core published as 0.1.0; exact registry version and artifact confirmed.

Both packages remain publish=false. Existing Cargo token configuration was detected
without printing its contents; authentication validity has not been assumed.
The official sparse index currently contains neither intended name, but this is
not a reservation or proof that the names are publishable.

Package artifact review used /tmp/app-context-package-review copied from source,
without --allow-dirty or --no-verify. This is independent package evidence, not an
assertion that the working Git checkout is clean. Core package resolution remains
blocked until the derive crate is registered. No package has been uploaded.

The dry-run-only candidate temporarily enabled publication for app-context-derive
in /tmp; neither working project manifest was enabled. The exact macro source,
README and licenses match the working project. Audit tool cargo-deny was obtained
from its official GitHub release and verified against SHA-256
9f12ed4c49936e09b48bf862b595cde2fe64fcbd9d74dfacac6131ca824c8d5f.
No audit warnings or vulnerability entries were ignored. Duplicate dependency
versions are reported at warning level as configured from the outset.

Next action: push the finished source to the configured repository, run the prepared
CI matrix, inspect/fix every failure, then enable release publication, publish derive,
verify core registry packaging and publish core. CODEX_PROMPT.md requires separate
user authorization before pushing the remote repository, so the push is pending.
