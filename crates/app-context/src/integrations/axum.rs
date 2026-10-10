//! Axum extraction from router state containing a [`crate::Context`].
//!
//! Set the router state to `Context`, or implement `axum::extract::FromRef<S>`
//! for `Context`. Handler parameters can then use `ContextState<T>`.
use crate::{Context, FromContext};
use ::axum::{
    extract::{FromRef, FromRequestParts},
    http::{StatusCode, request::Parts},
};
use std::ops::Deref;

/// Owned typed sub-state extracted from an Axum router's application context.
#[derive(Debug, Clone)]
pub struct ContextState<T>(pub T);

impl<T> ContextState<T> {
    /// Consume the wrapper, returning the extracted value.
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> Deref for ContextState<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

impl<S, T> FromRequestParts<S> for ContextState<T>
where
    S: Send + Sync,
    Context: FromRef<S>,
    T: FromContext + Send,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        Context::from_ref(state).extract().map(Self).map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "application state unavailable",
            )
        })
    }
}
