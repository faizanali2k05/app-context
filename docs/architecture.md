# Architecture

app-context/src/lib.rs implements the core. error.rs defines structured errors.
integrations/ contains feature-gated adapters. app-context-derive is a separate
procedural macro package.

Flow: insert T/Arc<T> -> freeze builder -> clone Context across workers -> borrow
T or clone its Arc -> extract explicit sub-state. Scopes copy entries into a new
builder and freeze a new snapshot. There is no global state, factory execution,
lazy service resolution or lock on the map.

Entry invariant: TypeId::of::<T>() is paired with erased Arc<T> and type_name::<T>().
Only typed insertion creates entries. Retrieval downcasts to the same type;
invariant mismatch is treated as missing rather than panicking.

The core has no runtime dependencies by default. Derive and framework features
are optional and additive, including when enabled together.
