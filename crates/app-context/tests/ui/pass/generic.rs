use app_context::{Context, FromContext};
use std::sync::Arc;
#[derive(FromContext)]
struct State<T> { value: Arc<T> }
fn main() {
    let context = Context::builder().with(9_u32).unwrap().build();
    assert_eq!(*context.extract::<State<u32>>().unwrap().value, 9);
}
