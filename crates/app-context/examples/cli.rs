use app_context::{Context, ContextError};

struct Settings {
    greeting: String,
}

fn main() -> Result<(), ContextError> {
    let context = Context::builder()
        .with(Settings {
            greeting: "Hello from app-context".into(),
        })?
        .build();
    println!("{}", context.get_ref::<Settings>()?.greeting);
    Ok(())
}
