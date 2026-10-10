use app_context::{Context, ContextError};
use std::{
    sync::atomic::{AtomicUsize, Ordering},
    thread,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let context = Context::builder().with(AtomicUsize::new(0))?.build();
    let worker_context = context.clone();
    let worker = thread::spawn(move || -> Result<(), ContextError> {
        worker_context
            .get::<AtomicUsize>()?
            .fetch_add(1, Ordering::Relaxed);
        Ok(())
    });
    worker
        .join()
        .map_err(|_| std::io::Error::other("worker panicked"))??;
    println!(
        "completed: {}",
        context.get_ref::<AtomicUsize>()?.load(Ordering::Relaxed)
    );
    Ok(())
}
