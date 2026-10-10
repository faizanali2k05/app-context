# app-context: implementation requirements

Create a lightweight Rust crate named `app-context`.

Goal:
Provide framework-independent, typed application state/context with ergonomics inspired by Axum State/FromRef, without depending on Axum.

This is NOT a full enterprise DI container.

Core API:
Context
ContextBuilder
FromContext
ContextError

Features:
- insert typed values
- retrieve typed values
- Arc-backed shared dependencies
- derive FromContext for sub-state
- nested contexts/scopes if they remain simple
- immutable context after construction
- compile-time-friendly extraction APIs
- helpful duplicate/missing-state errors

Avoid:
- string service names
- runtime reflection beyond TypeId where needed
- excessive macros
- hidden global state
- service locator magic

Optional integration modules:
axum
actix
tonic

Core crate must be framework independent.

Include examples:
- CLI application
- background worker
- Axum app
- tonic service
- test dependency substitution

Benchmark retrieval and initialization overhead.
Keep the core tiny.
