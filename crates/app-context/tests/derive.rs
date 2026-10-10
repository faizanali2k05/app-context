#![cfg(feature = "derive")]
use app_context::{Context, ContextError, FromContext};
use std::{marker::PhantomData, sync::Arc};

#[derive(FromContext)]
struct Inner {
    count: Arc<u32>,
}
#[derive(FromContext)]
struct Outer {
    inner: Inner,
    #[context(clone)]
    label: String,
    #[context(default)]
    enabled: bool,
    context: Context,
}
#[derive(FromContext)]
struct Tuple(Arc<u32>, #[context(clone)] String);
#[derive(FromContext)]
struct Unit;
#[derive(FromContext)]
struct Empty {}
#[derive(FromContext)]
struct Generic<T> {
    value: Arc<T>,
}
#[derive(FromContext)]
struct BorrowedMarker<'a, T, const N: usize>
where
    T: Clone,
{
    #[context(default)]
    marker: PhantomData<&'a [T; N]>,
}

#[test]
fn nested_cloned_default_and_context_fields() {
    let context = Context::builder()
        .with(42_u32)
        .unwrap()
        .with(String::from("label"))
        .unwrap()
        .build();
    let outer = context.extract::<Outer>().unwrap();
    assert_eq!(*outer.inner.count, 42);
    assert_eq!(outer.label, "label");
    assert!(!outer.enabled);
    assert!(Arc::ptr_eq(&outer.inner.count, &context.get().unwrap()));
    assert_eq!(outer.context.len(), 2);
    let tuple = context.extract::<Tuple>().unwrap();
    assert_eq!(*tuple.0, 42);
    assert_eq!(tuple.1, "label");
    let generic = context.extract::<Generic<u32>>().unwrap();
    assert_eq!(*generic.value, 42);
    let _ = context.extract::<Unit>().unwrap();
    let _ = context.extract::<Empty>().unwrap();
    let _ = context.extract::<BorrowedMarker<'_, String, 3>>().unwrap();
}

#[test]
fn derives_propagate_missing_errors() {
    let context = Context::builder().with(42_u32).unwrap().build();
    assert!(
        matches!(context.extract::<Outer>(), Err(ContextError::Missing { type_name }) if type_name.contains("String"))
    );
    assert!(matches!(
        Context::new().extract::<Tuple>(),
        Err(ContextError::Missing { type_name: "u32" })
    ));
}
