# Security model

All core lookups use safe std Any downcasts. Entry keys are private and created only
by typed insertion. No unsafe code, network access or filesystem access occurs in
the default core. Debug shows sorted type names but never values. Errors include
type names only; adapters deliberately redact them from remote responses.

Contexts are not capability or tenant isolation mechanisms: holders can retrieve
any known registered type. Stored values can contain secrets and interior mutability.
Scope maps are separate, but unchanged values are shared. Type aliases are not
separate registrations. Store per-request/tenant state only in carefully scoped
contexts; replace it explicitly. TypeId/name output must not be persisted as identity.

User FromContext, Clone and Drop implementations can panic or have side effects.
Partial sub-state extraction does not roll back earlier user actions. Allocation
failure follows Rust allocator behavior. No deadlock guarantee applies to user
locks. Builder mutable failures do not alter existing registrations, although the
rejected input is dropped as usual. Consuming fluent methods consume the builder.

Optional frameworks have their own transport/security/dependency behavior, reviewed
through cargo audit/deny CI. Tonic overwrites Context extensions intentionally; do
not combine independent interceptors that disagree about the desired scope.
