//! Tonic request-extension integration without transport or generated-code dependencies.
//!
//! Use [`ContextInterceptor`] with a generated server's `with_interceptor` method.
//! Extract typed state in the RPC handler with [`extract`]. Existing unrelated
//! extensions are preserved; an existing `Context` extension is replaced.
use crate::{Context, FromContext};
use ::tonic::{Request, Status, service::Interceptor};

/// Attach a shared application context to each incoming Tonic request.
#[derive(Debug, Clone)]
pub struct ContextInterceptor {
    context: Context,
}

impl ContextInterceptor {
    /// Create an interceptor using the supplied immutable application context.
    pub fn new(context: Context) -> Self {
        Self { context }
    }
}

impl Interceptor for ContextInterceptor {
    fn call(&mut self, mut request: Request<()>) -> Result<Request<()>, Status> {
        request.extensions_mut().insert(self.context.clone());
        Ok(request)
    }
}

/// Extract sub-state from a request populated by [`ContextInterceptor`].
///
/// Returns `Status::internal` with a generic message when state is unavailable.
/// This intentionally does not expose Rust type names to the remote caller.
pub fn extract<T: FromContext, B>(request: &Request<B>) -> Result<T, Status> {
    request
        .extensions()
        .get::<Context>()
        .ok_or_else(|| Status::internal("application state unavailable"))?
        .extract()
        .map_err(|_| Status::internal("application state unavailable"))
}
