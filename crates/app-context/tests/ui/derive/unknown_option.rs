use app_context::FromContext;
#[derive(FromContext)]
struct State { #[context(secret)] value: u32 }
fn main() {}
