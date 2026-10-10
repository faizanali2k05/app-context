# Implementation plan

1. Design immutable type map, Arc ownership, errors, scope snapshots, derive and adapters.
2. Implement dependency-free core with identity/isolation/errors/concurrency tests.
3. Implement derive for named/tuple/unit and generic structs, nested sub-state,
   cloned values, renamed dependency support and compile-fail diagnostics.
4. Implement optional Axum/Actix extractors and Tonic interceptor; in-process tests.
5. Complete rustdoc, examples, policies, licenses, feature/platform/MSRV CI and benchmarks.
6. Audit packages and verify names, repository, ownership and release prerequisites;
   publish dependency first only when every required gate passes.

Acceptance mapping: core invariants -> core integration tests; extraction types ->
doctests and trybuild; derives -> derive integration and UI tests; adapters ->
in-process request tests; isolation -> no-default-feature checks; examples ->
build/run; performance -> Criterion measurements. Track actual results in STATUS.md.

Primary references:
- https://doc.rust-lang.org/std/any/struct.TypeId.html
- https://doc.rust-lang.org/std/sync/struct.Arc.html
- https://docs.rs/proc-macro-crate/latest/proc_macro_crate/
- https://docs.rs/axum/latest/axum/extract/trait.FromRequestParts.html
- https://docs.rs/actix-web/latest/actix_web/trait.FromRequest.html
- https://docs.rs/tonic/latest/tonic/service/trait.Interceptor.html

Progress: design, core, derive, adapters, docs, licenses, examples, benchmarks,
local stable/MSRV checks and dependency audits completed. Derive package verification
and dry-run passed. Remaining: remote platform CI, enable publication, publish derive,
verify core against registry dependency and publish core. See STATUS.md and
RELEASE_CHECKLIST.md. Publication authorization exists; remote Git push authorization
is separately required by CODEX_PROMPT.md.
