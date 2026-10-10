use app_context::FromContext;
#[derive(FromContext)]
struct State { #[context()] value: u32 }
fn main() {}
