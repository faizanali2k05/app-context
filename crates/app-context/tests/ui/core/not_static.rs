use app_context::Context;
fn main() {
    let text = String::from("local");
    let _ = Context::builder().with(text.as_str());
}
