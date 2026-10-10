use app_context::Context;
use std::rc::Rc;
fn main() { let _ = Context::builder().with(Rc::new(1)); }
