# app-context

Immutable, typed application state for services, command-line applications and workers.
Register each dependency once, share it through `Arc`, and extract explicit sub-state
without a dependency injection framework or global state.

## Installation and quick start

```toml
[dependencies]
app-context = "0.1"
```

```rust
use app_context::{Context, ContextError};
struct Database { name: String }
let context = Context::builder()
    .with(Database { name: "orders".into() })?
    .with(8080_u16)?
    .build();
let database = context.get::<Database>()?; // Arc<Database>
assert_eq!(database.name, "orders");
assert_eq!(*context.get_ref::<u16>()?, 8080); // borrowed, no Arc clone
# Ok::<(), ContextError>(())
```

Use this when several parts of an application need the same registered dependencies
and explicit extraction errors are useful. Prefer an ordinary struct and references
when your state is small and statically known. This crate does not construct services,
resolve dependency graphs, run lifecycle hooks or prove registration at compile time.

## Shared values and scopes

```rust
use app_context::{Context, ContextError};
use std::sync::Arc;
struct Endpoint(&'static str);
let endpoint = Arc::new(Endpoint("production"));
let parent = Context::builder().with_arc(endpoint.clone())?.build();
assert!(Arc::ptr_eq(&endpoint, &parent.get::<Endpoint>()?));
let mut builder = parent.scope();
builder.replace(Endpoint("test"))?;
let child = builder.build();
assert_eq!(child.get_ref::<Endpoint>()?.0, "test");
assert_eq!(parent.get_ref::<Endpoint>()?.0, "production");
# Ok::<(), ContextError>(())
```

`insert` rejects duplicate types without overwriting them; `replace` requires an
existing registration. Mutable builder errors preserve the map. Fluent `with` and
`with_arc` consume the builder even when they return an error. Context is immutable,
while stored values may use their own interior mutability. Context cloning shares
the whole map; a scope copies entries while sharing the dependencies' Arcs.

Keys are exact Rust types. Aliases refer to the same slot; use newtypes for multiple
strings, database pools or configuration objects. `with_arc(Arc<T>)` registers `T`;
`with(Arc<T>)` registers `Arc<T>` and retrieval returns `Arc<Arc<T>>`. For trait-object
services, wrap them in a newtype or store the sized `Arc<dyn Trait + Send + Sync>`.
Values must be `Send + Sync + 'static`. No borrowed local state or `Rc` is accepted.

## Explicit sub-state extraction

```toml
app-context = { version = "0.1", features = ["derive"] }
```

```rust
# #[cfg(feature = "derive")]
# fn example() -> Result<(), app_context::ContextError> {
use app_context::{Context, FromContext};
use std::sync::Arc;
struct Database;
#[derive(FromContext)]
struct Worker {
    database: Arc<Database>,
    #[context(clone)]
    name: String,
    #[context(default)]
    retries: u32,
}
let context = Context::builder().with(Database)?.with(String::from("worker"))?.build();
let worker = context.extract::<Worker>()?;
assert_eq!(worker.name, "worker");
assert_eq!(worker.retries, 0);
assert!(Arc::ptr_eq(&worker.database, &context.get::<Database>()?));
# Ok(())
# }
# #[cfg(feature = "derive")]
# example().unwrap();
```

Implement `FromContext` manually to add custom extraction behavior. The derive
supports named, tuple and unit structs, generics, lifetimes and const parameters.
Fields recursively call `FromContext` in declaration order. `Arc<T>` shares stored
`T`, and `Context` clones the context. `#[context(clone)]` clones a stored concrete
value; `#[context(default)]` never performs a lookup. Only one field option is allowed.
Enums, unions and container-level options are rejected. Missing fields return
`ContextError::Missing` with the requested Rust type name; duplicates return
`ContextError::Duplicate`. Earlier field clones are not rolled back if extraction fails.
Renamed Cargo dependencies are supported automatically. No blanket `FromContext`
implementation clones arbitrary values, leaving room for user-defined sub-state.

## Feature flags and framework adapters

| Feature | Behavior |
| --- | --- |
| default (empty) | std-only context/builder/errors/extraction |
| derive | re-export `FromContext` procedural macro |
| axum | `app_context::axum::ContextState<T>` extractor for Axum 0.8 |
| actix | `app_context::actix::ContextState<T>` extractor for Actix Web 4 |
| tonic | `ContextInterceptor` and `extract` for Tonic 0.14 |

All features are additive and can be enabled together. No default framework,
network listener or asynchronous runtime is installed by the core.

Axum: use `Context` as router state (or implement `FromRef<OuterState>` for it), and
extract `ContextState<YourSubState>` in a handler. Actix: register
`web::Data::new(context)` via `App::app_data`, then use `ContextState<YourSubState>`.
Tonic: attach `ContextInterceptor::new(context)` to a generated server with
`with_interceptor`, then call `app_context::tonic::extract(&request)` in RPC methods.
The interceptor preserves unrelated extensions and replaces an existing `Context`.
Adapters return a generic internal error for missing state so remote responses do
not expose Rust type names. Use core extraction at startup to validate registrations.

## Complete runnable examples

From the repository root:

```sh
cargo run -p app-context --example cli
cargo run -p app-context --example background_worker
cargo run -p app-context --example test_substitution
cargo run -p app-context --example axum_app --features axum,derive
cargo run -p app-context --example tonic_service --features tonic,derive
```

The Axum example sends an actual in-process router request. The Tonic example
contains a transport-independent RPC method and passes a request through the real
interceptor; integrate that method body with your generated service. Neither needs
an open network port, database, or `protoc`. Integration tests cover success and
missing-state paths for all three frameworks.

## Compatibility and stability

Rust 2024 edition; MSRV Rust 1.88. CI checks Linux, macOS and Windows on stable and
1.88. Other std platforms are expected to work but are not in the support matrix.
TypeId and type-name output is diagnostic only, not a stable serialized identifier.
Version 0.1 is an initial API: breaking changes use a minor version until 1.0;
fixes that preserve documented behavior use patch versions. No on-disk format exists.

## Security and performance

No unsafe Rust, hidden globals or value printing in Debug/errors. Type names may
reveal application structure; do not send core errors to untrusted clients.
This is not an access-control or secret-storage boundary. Do not store per-user
state in a global shared context unless sharing is intended. User Clone/Drop and
FromContext implementations can execute arbitrary code or panic. Allocation failure
follows the standard allocator policy. Interior mutability remains the caller's
responsibility. See repository docs/security.md and SECURITY.md.

Borrowing performs a hash lookup/downcast; shared retrieval additionally clones an
Arc. Scope creation copies n map entries and their Arc handles. There is no universal
speed or overhead claim. `cargo bench -p app-context --bench context` measures
retrieval, cloning, scope creation and one/four-type initialization on your machine.
See docs/performance.md for the methodology and recorded local results.

## Alternatives

An ordinary state struct provides compile-time field presence and no type map.
Axum State/FromRef is suitable for an Axum-only application. DI frameworks such as
shaku provide construction patterns this crate intentionally leaves to the caller.
app-context is useful when the same immutable registration model should work in
multiple application environments. It is not a replacement for every state struct.

## License

Dual-licensed under MIT OR Apache-2.0. See LICENSE-MIT and LICENSE-APACHE.
