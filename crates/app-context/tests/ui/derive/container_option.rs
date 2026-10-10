use app_context::FromContext;
#[derive(FromContext)]
#[context(clone)]
struct State;
fn main() {}
