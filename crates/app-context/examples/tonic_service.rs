use app_context::{
    Context, FromContext,
    tonic::{ContextInterceptor, extract},
};
use std::sync::Arc;
use tonic::{Request, Response, Status, service::Interceptor};

struct Greeting(String);
#[derive(FromContext)]
struct ServiceState {
    greeting: Arc<Greeting>,
}

// This is the body of a generated Tonic service method. No protoc/build.rs needed.
fn greet(request: Request<()>) -> Result<Response<String>, Status> {
    let state: ServiceState = extract(&request)?;
    Ok(Response::new(state.greeting.0.clone()))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let context = Context::builder().with(Greeting("hello".into()))?.build();
    let mut interceptor = ContextInterceptor::new(context);
    let request = interceptor.call(Request::new(()))?;
    let response = greet(request)?;
    assert_eq!(response.into_inner(), "hello");
    println!("Tonic service state extraction succeeded");
    Ok(())
}
