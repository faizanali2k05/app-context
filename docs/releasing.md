# Release procedure

Publication is authorized by the user after required checks pass. Both manifests
remain publish=false until release readiness is established. Do not bypass package
verification or use --allow-dirty/--no-verify. Obtain a verified canonical repository
URL and review the security reporting policy. Do not invent ownership or URLs.

1. Verify supported platform/MSRV CI, feature tests, rustdoc/examples, UI diagnostics,
   dependency/security checks and package contents.
2. Check crates.io names and owner lists using official registry endpoints. A missing
   name may still be reserved; final registry publication decides availability.
3. Check secure Cargo authentication without printing token contents. Authenticate
   locally if missing; never put credentials into project files or chat.
4. Commit reviewed release files in a clean checkout, deliberately enable publication,
   and run scripts/check-release.py. Use matching 0.1.0 versions on both packages.
5. Run cargo package --list -p app-context-derive, cargo package -p app-context-derive,
   and cargo publish --dry-run -p app-context-derive, then publish the derive crate.
6. Wait until the exact derive version is available through the registry. Then package,
   verify and dry-run app-context (including derive-enabled extraction using the
   packaged registry dependency) before publishing it. All features must resolve.
7. Confirm exact registry versions and downloadable artifacts. Record the release
   in CHANGELOG.md and a versioned release receipt under `docs/`. Never retry a
   publish without first checking whether it succeeded despite a client error.

Core packaging cannot fully verify a not-yet-published derive dependency through
the registry; do not label this gate passed merely because local path builds pass.
No publication occurs when metadata, authentication or essential CI is missing.
