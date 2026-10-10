use app_state::{Context, FromContext};
use std::sync::Arc;
#[derive(FromContext)]
struct State { value: Arc<u32> }
fn main() {
    let context = Context::builder().with(8_u32).unwrap().build();
    assert_eq!(*context.extract::<State>().unwrap().value, 8);
}
