//! core verification for app-context.
use app_context::{Context, ContextBuilder, ContextError, FromContext};
use std::{
    any::type_name,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    thread,
};

#[test]
fn empty_context_and_builder_defaults() {
    for context in [
        Context::new(),
        Context::default(),
        ContextBuilder::new().build(),
        ContextBuilder::default().build(),
    ] {
        assert!(context.is_empty());
        assert_eq!(context.len(), 0);
        assert!(!context.contains::<u32>());
        assert_eq!(
            context.get::<u32>(),
            Err(ContextError::Missing { type_name: "u32" })
        );
        assert_eq!(
            context.get_ref::<u32>(),
            Err(ContextError::Missing { type_name: "u32" })
        );
    }
}

#[test]
fn duplicate_and_failed_replace_preserve_registrations() {
    let mut builder = Context::builder();
    builder.insert(10_u32).unwrap();
    assert_eq!(
        builder.insert(20_u32).unwrap_err(),
        ContextError::Duplicate { type_name: "u32" }
    );
    assert_eq!(
        builder.replace(20_u64).unwrap_err(),
        ContextError::Missing { type_name: "u64" }
    );
    let context = builder.build();
    assert_eq!(context.len(), 1);
    assert_eq!(*context.get_ref::<u32>().unwrap(), 10);
    assert!(!context.contains::<u64>());
}

#[test]
fn fluent_builder_shares_existing_arcs_under_inner_type() {
    let value = Arc::new(String::from("database"));
    let context = Context::builder()
        .with_arc(Arc::clone(&value))
        .unwrap()
        .with(5_u16)
        .unwrap()
        .build();
    assert_eq!(context.len(), 2);
    let retrieved = context.get::<String>().unwrap();
    assert!(Arc::ptr_eq(&value, &retrieved));
    assert!(context.contains::<String>());
    assert!(!context.contains::<Arc<String>>());
    assert!(Arc::ptr_eq(&value, &context.clone().get().unwrap()));
}

#[test]
fn explicit_replacement_preserves_new_arc_identity() {
    let value = Arc::new(5_u8);
    let mut builder = Context::builder().with(1_u8).unwrap();
    builder.replace_arc(value.clone()).unwrap();
    assert!(Arc::ptr_eq(&value, &builder.build().get().unwrap()));
}

#[test]
fn scopes_are_isolated_and_share_unchanged_values() {
    let parent = Context::builder()
        .with(1_u32)
        .unwrap()
        .with(String::from("shared"))
        .unwrap()
        .build();
    let mut child_builder = parent.scope();
    assert!(matches!(
        child_builder.insert(2_u32),
        Err(ContextError::Duplicate { .. })
    ));
    child_builder.replace(2_u32).unwrap().insert(true).unwrap();
    let child = child_builder.build();
    let mut grandchild = child.scope();
    grandchild.replace(3_u32).unwrap();
    assert_eq!(*grandchild.build().get_ref::<u32>().unwrap(), 3);
    assert_eq!(*child.get_ref::<u32>().unwrap(), 2);
    assert_eq!(*parent.get_ref::<u32>().unwrap(), 1);
    assert!(!parent.contains::<bool>());
    assert!(Arc::ptr_eq(
        &parent.get::<String>().unwrap(),
        &child.get().unwrap()
    ));
}

#[test]
fn aliases_share_slots_and_newtypes_distinguish_values() {
    type Alias = u32;
    struct Port(u32);
    let mut builder = Context::builder().with(1_u32).unwrap();
    assert!(matches!(
        builder.insert(2 as Alias),
        Err(ContextError::Duplicate { .. })
    ));
    builder.insert(Port(3)).unwrap();
    let context = builder.build();
    assert_eq!(*context.get_ref::<Alias>().unwrap(), 1);
    assert_eq!(context.get_ref::<Port>().unwrap().0, 3);
}

#[test]
fn errors_are_actionable_and_debug_does_not_reveal_values() {
    let mut builder = Context::builder().with("TOP_SECRET".to_string()).unwrap();
    let duplicate = builder.insert(String::new()).unwrap_err();
    assert!(duplicate.to_string().contains("explicitly replace"));
    assert!(!format!("{builder:?}").contains("TOP_SECRET"));
    let context = builder.build();
    assert!(!format!("{context:?}").contains("TOP_SECRET"));
    assert!(
        context
            .get::<u64>()
            .unwrap_err()
            .to_string()
            .contains("insert it before")
    );
    let error = ContextError::Missing {
        type_name: type_name::<u64>(),
    };
    assert_eq!(error, error.clone());
    let _: &dyn std::error::Error = &error;
}

#[test]
fn threads_share_interior_mutability() {
    let context = Context::builder()
        .with(AtomicUsize::new(0))
        .unwrap()
        .build();
    let handles: Vec<_> = (0..16)
        .map(|_| {
            let context = context.clone();
            thread::spawn(move || {
                for _ in 0..100 {
                    context
                        .get_ref::<AtomicUsize>()
                        .unwrap()
                        .fetch_add(1, Ordering::Relaxed);
                }
            })
        })
        .collect();
    for handle in handles {
        handle.join().unwrap();
    }
    assert_eq!(
        context
            .get_ref::<AtomicUsize>()
            .unwrap()
            .load(Ordering::Relaxed),
        1600
    );
}

#[test]
fn dependency_drops_only_after_last_owner() {
    struct Tracked(Arc<AtomicUsize>);
    impl Drop for Tracked {
        fn drop(&mut self) {
            self.0.fetch_add(1, Ordering::Relaxed);
        }
    }
    let drops = Arc::new(AtomicUsize::new(0));
    let context = Context::builder()
        .with(Tracked(drops.clone()))
        .unwrap()
        .build();
    let child = context.scope().build();
    let value = context.get::<Tracked>().unwrap();
    drop(context);
    drop(child);
    assert_eq!(drops.load(Ordering::Relaxed), 0);
    drop(value);
    assert_eq!(drops.load(Ordering::Relaxed), 1);
}

#[test]
fn extraction_is_explicit_and_missing_dependencies_fail() {
    struct State(Arc<u32>);
    impl FromContext for State {
        fn from_context(context: &Context) -> Result<Self, ContextError> {
            Ok(Self(context.get()?))
        }
    }
    let context = Context::builder().with(7_u32).unwrap().build();
    assert_eq!(*context.extract::<State>().unwrap().0, 7);
    assert!(Arc::ptr_eq(
        &context.get::<u32>().unwrap(),
        &context.extract::<Arc<u32>>().unwrap()
    ));
    assert_eq!(context.extract::<Context>().unwrap().len(), 1);
    assert!(matches!(
        Context::new().extract::<State>(),
        Err(ContextError::Missing { .. })
    ));
}

#[test]
fn trait_objects_are_supported_through_sized_wrappers() {
    trait Service: Send + Sync {
        fn value(&self) -> u8;
    }
    struct Real;
    impl Service for Real {
        fn value(&self) -> u8 {
            9
        }
    }
    let service: Arc<dyn Service> = Arc::new(Real);
    let context = Context::builder().with(service).unwrap().build();
    assert_eq!(context.get_ref::<Arc<dyn Service>>().unwrap().value(), 9);
}

#[test]
fn deep_scopes_do_not_recurse_on_lookup_or_drop() {
    let mut context = Context::builder().with(0_u32).unwrap().build();
    for i in 1..10_000 {
        let mut builder = context.scope();
        builder.replace(i as u32).unwrap();
        context = builder.build();
    }
    assert_eq!(*context.get_ref::<u32>().unwrap(), 9_999);
}
