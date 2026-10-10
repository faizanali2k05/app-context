use app_context::{Context, FromContext};
#[derive(FromContext)]
struct State { value: u32 }
fn main() { let _ = Context::new().extract::<State>(); }
