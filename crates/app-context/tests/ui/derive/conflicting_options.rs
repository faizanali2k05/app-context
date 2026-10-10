use app_context::FromContext;
#[derive(FromContext)]
struct State { #[context(clone, default)] value: u32 }
fn main() {}
