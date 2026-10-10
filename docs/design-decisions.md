# Design decisions

The core depends only on std. Context is an Arc-owned immutable HashMap keyed by
TypeId, with Arc<dyn Any + Send + Sync> entries. Values are Send + Sync + 'static.
No unsafe code is permitted. Type names are labels, never lookup identifiers.

API: Context::{new,builder,get,get_ref,contains,len,is_empty,extract,scope};
ContextBuilder::{new,insert,insert_arc,replace,replace_arc,with,with_arc,build}.
Insertion checks before mutation; duplicates preserve the original value.
Replacement requires an existing type. Fluent methods consume the builder and
return Result. build is infallible. Context provides no mutation API.

Scopes copy map entries and share each value's Arc. Explicit replacement shadows
an inherited type without changing the parent. Lookup is independent of scope
depth, and drop does not traverse recursive parent chains. Context clones share
one map. Interior mutability inside stored values remains shared intentionally.

FromContext extracts owned sub-state fallibly. Arc<T> retrieves shared T, and
Context clones itself. No blanket implementation clones arbitrary T, because it
would conflict with user sub-state implementations. Derive recursively extracts
fields; #[context(clone)] clones a concrete stored value, #[context(default)]
constructs a field without lookup. Bounds are added to field types. The macro
resolves renamed dependencies using proc-macro-crate.

Missing and duplicate registrations are runtime errors because registrations can
be selected dynamically; field extraction and thread safety are compile-time
constraints. Aliases share a slot; use newtypes for distinct same-representation
values. Unsized trait objects must be wrapped in a sized Arc<dyn Trait> before
insertion, making retrieval return Arc<Arc<dyn Trait>>.

Axum obtains Context using FromRef<S>; Actix uses web::Data<Context>; Tonic attaches
Context to request extensions using an interceptor. Public adapter errors are
generic to avoid exposing type names. Local core errors retain actionable names.

Failure modes: missing or duplicate type, missing framework state, invalid derive
input, extraction failure and allocation failure (Rust allocator behavior).
User Clone/FromContext/Drop may panic; we do not catch them. Earlier fields can
be cloned before a later field lookup fails.

Testing covers errors, aliases/newtypes, Arc identity/drop, immutable snapshots,
threads, generic derives and compile failures plus actual framework request
paths. This is not a parser or security boundary; parser fuzzing is inapplicable.

MSRV is uniformly 1.88 for both packages and every optional integration. Although
the std-only core may compile on earlier Rust, this avoids promising a lower
version that optional frameworks and current macro dependencies cannot support.
The workspace lockfile and CI preserve the checked resolution.
