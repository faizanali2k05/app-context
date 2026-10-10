# app-context-derive

Procedural derive for app-context typed sub-state extraction. Normally enable the
`derive` feature on app-context and import app_context::FromContext.

Supports named, tuple and unit structs, generics, lifetimes, const generics and
nested sub-state. By default each field uses its FromContext implementation.
Arc<T> shares registered T; #[context(clone)] clones a registered concrete field;
#[context(default)] initializes a field with Default and performs no lookup.
Unknown, empty, conflicting or container options, enums and unions are rejected.
The macro locates renamed app-context dependencies with proc-macro-crate.

Generated code extracts in field order with ?, and adds the necessary bounds to
field types. Earlier field clones/extraction effects are not rolled back when a
later field fails. No unsafe code, factories, caching or globals are generated.
The macro must have an app-context dependency in the caller's Cargo manifest.

Rust 2024 edition, MSRV 1.88. Dual license MIT OR Apache-2.0. This is initially a
0.1 API; breaking changes use a minor version before 1.0. Compile-fail and generic
success tests live in the app-context package to avoid a cyclic dev dependency.
