use app_context::{Context, FromContext, axum::ContextState};
use axum::{
    Router,
    body::Body,
    http::{Request, StatusCode},
    routing::get,
};
use std::sync::Arc;
use tower::ServiceExt;

struct Greeting(String);
#[derive(FromContext)]
struct HandlerState {
    greeting: Arc<Greeting>,
}

async fn greet(ContextState(state): ContextState<HandlerState>) -> String {
    state.greeting.0.clone()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let context = Context::builder().with(Greeting("hello".into()))?.build();
    let router = Router::new().route("/", get(greet)).with_state(context);
    // Exercise a real router request without opening a network port.
    let response = router
        .oneshot(Request::builder().uri("/").body(Body::empty())?)
        .await?;
    assert_eq!(response.status(), StatusCode::OK);
    println!("Axum context extraction succeeded");
    Ok(())
}
