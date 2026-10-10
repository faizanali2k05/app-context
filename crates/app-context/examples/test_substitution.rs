use app_context::{Context, ContextError};

#[derive(Debug, PartialEq)]
struct Endpoint(&'static str);

fn main() -> Result<(), ContextError> {
    let production = Context::builder().with(Endpoint("production"))?.build();
    let mut test_scope = production.scope();
    test_scope.replace(Endpoint("local-test"))?;
    let testing = test_scope.build();
    assert_eq!(production.get_ref::<Endpoint>()?, &Endpoint("production"));
    assert_eq!(testing.get_ref::<Endpoint>()?, &Endpoint("local-test"));
    Ok(())
}
