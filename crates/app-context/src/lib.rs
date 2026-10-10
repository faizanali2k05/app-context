//! Framework-independent, immutable, typed application state.
//!
//! The default core has no dependencies. Values are shared using [`Arc`]; registration
//! is explicit and frozen when a [`ContextBuilder`] is built. There are no globals
//! or automatic dependency factories. Registrations use exact Rust types.
//!
//! ```
//! use app_context::{Context, ContextError};
//! struct Database { name: String }
//! let context = Context::builder()
//!     .with(Database { name: "orders".into() })?
//!     .with(8080_u16)?
//!     .build();
//! let database = context.get::<Database>()?;
//! assert_eq!(database.name, "orders");
//! let mut scoped = context.scope();
//! scoped.replace(9090_u16)?;
//! assert_eq!(*scoped.build().get_ref::<u16>()?, 9090);
//! assert_eq!(*context.get_ref::<u16>()?, 8080);
//! # Ok::<(), ContextError>(())
//! ```
//!
//! Enable `derive` for [`FromContext`](trait@FromContext) derivation, or `axum`,
//! `actix`, and `tonic` for optional adapters. See the README for feature examples.
//! Stored types must be `Send + Sync + 'static`. Use newtypes for distinct values
//! of the same representation. Context immutability does not freeze interior
//! mutability inside a dependency.
//!
//! ```compile_fail
//! use app_context::Context;
//! use std::rc::Rc;
//! // Rc is not Send + Sync and cannot be shared in an application context.
//! let _ = Context::builder().with(Rc::new(1));
//! ```

#![forbid(unsafe_code)]

use std::{
    any::{Any, TypeId, type_name},
    collections::HashMap,
    fmt,
    sync::Arc,
};
mod error;
pub use error::ContextError;

#[cfg(feature = "derive")]
pub use app_context_derive::FromContext;

#[cfg(any(feature = "axum", feature = "actix", feature = "tonic"))]
mod integrations;
#[cfg(feature = "actix")]
pub use integrations::actix;
#[cfg(feature = "axum")]
pub use integrations::axum;
#[cfg(feature = "tonic")]
pub use integrations::tonic;

#[derive(Clone)]
struct Entry {
    value: Arc<dyn Any + Send + Sync>,
    name: &'static str,
}

/// An immutable collection of typed, shared application dependencies.
///
/// Clones share the entire map. Retrieval never invokes dependency factories.
/// `get::<T>()` returns `Arc<T>` while `get_ref::<T>()` borrows `T`.
#[derive(Clone, Default)]
pub struct Context {
    entries: Arc<HashMap<TypeId, Entry>>,
}

impl Context {
    /// Construct an empty immutable context. Use [`Self::builder`] to add values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Start constructing a context.
    pub fn builder() -> ContextBuilder {
        ContextBuilder::new()
    }

    /// Retrieve shared ownership of a registered value of exactly type `T`.
    ///
    /// Returns [`ContextError::Missing`] when `T` is not registered.
    pub fn get<T: Send + Sync + 'static>(&self) -> Result<Arc<T>, ContextError> {
        self.entries
            .get(&TypeId::of::<T>())
            .and_then(|entry| Arc::clone(&entry.value).downcast::<T>().ok())
            .ok_or(ContextError::Missing {
                type_name: type_name::<T>(),
            })
    }

    /// Borrow a registered value, without cloning its `Arc`.
    ///
    /// Returns [`ContextError::Missing`] when `T` is not registered.
    pub fn get_ref<T: Send + Sync + 'static>(&self) -> Result<&T, ContextError> {
        self.entries
            .get(&TypeId::of::<T>())
            .and_then(|entry| entry.value.downcast_ref::<T>())
            .ok_or(ContextError::Missing {
                type_name: type_name::<T>(),
            })
    }

    /// Whether a value of exactly type `T` is registered.
    pub fn contains<T: Send + Sync + 'static>(&self) -> bool {
        self.entries.contains_key(&TypeId::of::<T>())
    }

    /// Number of registered types.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether no types are registered.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Extract owned sub-state through its explicit [`FromContext`] implementation.
    pub fn extract<T: FromContext>(&self) -> Result<T, ContextError> {
        T::from_context(self)
    }

    /// Create a builder initially containing all current registrations.
    ///
    /// Values keep the same `Arc` identity. Use [`ContextBuilder::replace`] to
    /// override a type locally. Building a scope never changes this context.
    /// This copies the registration map, so creating a scope costs O(n) entries.
    pub fn scope(&self) -> ContextBuilder {
        ContextBuilder {
            entries: self.entries.as_ref().clone(),
        }
    }
}

impl fmt::Debug for Context {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut types: Vec<_> = self.entries.values().map(|e| e.name).collect();
        types.sort_unstable();
        f.debug_struct("Context")
            .field("types", &types)
            .finish_non_exhaustive()
    }
}

/// Mutable registration phase for an immutable [`Context`].
///
/// Insertion rejects duplicates. Replacement must be deliberate and requires an
/// existing registration. The mutable methods preserve existing entries on error.
#[derive(Default)]
pub struct ContextBuilder {
    entries: HashMap<TypeId, Entry>,
}

impl ContextBuilder {
    /// Start an empty registration map.
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a value, storing it in an `Arc`. Reject duplicate types.
    pub fn insert<T: Send + Sync + 'static>(
        &mut self,
        value: T,
    ) -> Result<&mut Self, ContextError> {
        self.insert_arc(Arc::new(value))
    }

    /// Register an existing `Arc<T>` under type `T`, preserving its identity.
    pub fn insert_arc<T: Send + Sync + 'static>(
        &mut self,
        value: Arc<T>,
    ) -> Result<&mut Self, ContextError> {
        use std::collections::hash_map::Entry as Slot;
        match self.entries.entry(TypeId::of::<T>()) {
            Slot::Occupied(_) => Err(ContextError::Duplicate {
                type_name: type_name::<T>(),
            }),
            Slot::Vacant(slot) => {
                slot.insert(Entry {
                    value,
                    name: type_name::<T>(),
                });
                Ok(self)
            }
        }
    }

    /// Explicitly replace an existing value of type `T`.
    ///
    /// Returns [`ContextError::Missing`] instead of creating a new registration.
    pub fn replace<T: Send + Sync + 'static>(
        &mut self,
        value: T,
    ) -> Result<&mut Self, ContextError> {
        self.replace_arc(Arc::new(value))
    }

    /// Replace a registered `T` with an existing `Arc<T>`, preserving its identity.
    pub fn replace_arc<T: Send + Sync + 'static>(
        &mut self,
        value: Arc<T>,
    ) -> Result<&mut Self, ContextError> {
        let entry = self
            .entries
            .get_mut(&TypeId::of::<T>())
            .ok_or(ContextError::Missing {
                type_name: type_name::<T>(),
            })?;
        entry.value = value;
        Ok(self)
    }

    /// Consuming convenience method for [`Self::insert`].
    ///
    /// On error this builder is consumed. Use `insert` if you need to recover it.
    pub fn with<T: Send + Sync + 'static>(mut self, value: T) -> Result<Self, ContextError> {
        self.insert(value)?;
        Ok(self)
    }

    /// Consuming convenience method for [`Self::insert_arc`].
    pub fn with_arc<T: Send + Sync + 'static>(
        mut self,
        value: Arc<T>,
    ) -> Result<Self, ContextError> {
        self.insert_arc(value)?;
        Ok(self)
    }

    /// Freeze all registrations. The resulting context has no mutation API.
    pub fn build(self) -> Context {
        Context {
            entries: Arc::new(self.entries),
        }
    }
}

impl fmt::Debug for ContextBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut types: Vec<_> = self.entries.values().map(|e| e.name).collect();
        types.sort_unstable();
        f.debug_struct("ContextBuilder")
            .field("types", &types)
            .finish_non_exhaustive()
    }
}

/// Explicit, fallible extraction of an owned dependency or sub-state.
///
/// Implemented for `Arc<T>` and `Context`. The optional derive supports nested
/// named, tuple and unit structs. Fields normally use their own `FromContext`
/// implementation; `#[context(clone)]` clones a stored concrete value, while
/// `#[context(default)]` initializes a field using `Default` without lookup.
///
/// ```
/// use app_context::{Context, ContextError, FromContext};
/// use std::sync::Arc;
/// struct Worker { count: Arc<u64> }
/// impl FromContext for Worker {
///     fn from_context(context: &Context) -> Result<Self, ContextError> {
///         Ok(Self { count: context.get()? })
///     }
/// }
/// let context = Context::builder().with(42_u64)?.build();
/// assert_eq!(*context.extract::<Worker>()?.count, 42);
/// # Ok::<(), ContextError>(())
/// ```
pub trait FromContext: Sized {
    /// Extract from registrations, reporting missing types as structured errors.
    fn from_context(context: &Context) -> Result<Self, ContextError>;
}

impl<T: Send + Sync + 'static> FromContext for Arc<T> {
    fn from_context(context: &Context) -> Result<Self, ContextError> {
        context.get()
    }
}

impl FromContext for Context {
    fn from_context(context: &Context) -> Result<Self, ContextError> {
        Ok(context.clone())
    }
}
