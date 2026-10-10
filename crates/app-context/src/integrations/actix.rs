//! Actix Web extraction from `web::Data<Context>` application data.
//!
//! Register exactly `web::Data::new(context)` using `App::app_data`.
use crate::{Context, FromContext};
use actix_web::{
    Error, FromRequest, HttpRequest, dev::Payload, error::ErrorInternalServerError, web::Data,
};
use std::{
    future::{Ready, ready},
    ops::Deref,
};

/// Owned typed sub-state extracted from Actix application data.
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

impl<T: FromContext> FromRequest for ContextState<T> {
    type Error = Error;
    type Future = Ready<Result<Self, Error>>;

    fn from_request(request: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let result = request
            .app_data::<Data<Context>>()
            .ok_or_else(|| ErrorInternalServerError("application state unavailable"))
            .and_then(|context| {
                context
                    .extract()
                    .map(Self)
                    .map_err(|_| ErrorInternalServerError("application state unavailable"))
            });
        ready(result)
    }
}
