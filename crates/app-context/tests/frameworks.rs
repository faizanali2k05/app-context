#[cfg(feature = "axum")]
mod axum_tests {
    use app_context::{Context, axum::ContextState};
    use axum::{
        Router,
        body::{Body, to_bytes},
        extract::FromRef,
        http::{Request, StatusCode},
        routing::get,
    };
    use std::sync::Arc;
    use tower::ServiceExt;

    #[derive(Clone)]
    struct Outer {
        context: Context,
    }
    impl FromRef<Outer> for Context {
        fn from_ref(state: &Outer) -> Self {
            state.context.clone()
        }
    }
    async fn handler(state: ContextState<Arc<String>>) -> String {
        state.as_str().to_owned()
    }

    #[tokio::test]
    async fn extracts_from_outer_router_state() {
        let context = Context::builder()
            .with(String::from("hello"))
            .unwrap()
            .build();
        let app = Router::new()
            .route("/", get(handler))
            .with_state(Outer { context });
        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            &to_bytes(response.into_body(), 1024).await.unwrap()[..],
            b"hello"
        );
    }

    #[tokio::test]
    async fn missing_dependency_returns_generic_internal_error() {
        let app = Router::new().route("/", get(handler)).with_state(Outer {
            context: Context::new(),
        });
        let response = app.oneshot(Request::new(Body::empty())).await.unwrap();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(
            &to_bytes(response.into_body(), 1024).await.unwrap()[..],
            b"application state unavailable"
        );
    }

    #[test]
    fn wrapper_into_inner() {
        assert_eq!(ContextState(7).into_inner(), 7);
    }
}

#[cfg(feature = "actix")]
mod actix_tests {
    use actix_web::{App, http::StatusCode, test, web};
    use app_context::{Context, actix::ContextState};
    use std::sync::Arc;
    async fn handler(state: ContextState<Arc<String>>) -> String {
        state.as_str().to_owned()
    }

    #[tokio::test]
    async fn extracts_from_app_data() {
        let context = Context::builder()
            .with(String::from("hello"))
            .unwrap()
            .build();
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(context))
                .route("/", web::get().to(handler)),
        )
        .await;
        let response =
            test::call_service(&app, test::TestRequest::get().uri("/").to_request()).await;
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(&test::read_body(response).await[..], b"hello");
    }

    #[tokio::test]
    async fn missing_context_and_missing_dependency_are_generic_errors() {
        for context in [None, Some(Context::new())] {
            let mut app = App::new().route("/", web::get().to(handler));
            if let Some(context) = context {
                app = app.app_data(web::Data::new(context));
            }
            let app = test::init_service(app).await;
            let response =
                test::call_service(&app, test::TestRequest::get().uri("/").to_request()).await;
            assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
            assert_eq!(
                &test::read_body(response).await[..],
                b"application state unavailable"
            );
        }
    }

    #[test]
    fn wrapper_into_inner() {
        assert_eq!(ContextState(7).into_inner(), 7);
    }
}

#[cfg(feature = "tonic")]
mod tonic_tests {
    use app_context::{
        Context,
        tonic::{ContextInterceptor, extract},
    };
    use std::sync::Arc;
    use tonic::{Code, Request, service::Interceptor};

    #[test]
    fn interceptor_preserves_request_metadata_and_other_extensions() {
        let context = Context::builder()
            .with(String::from("hello"))
            .unwrap()
            .build();
        let expected = context.get::<String>().unwrap();
        let mut interceptor = ContextInterceptor::new(context);
        let mut request = Request::new(());
        request
            .metadata_mut()
            .insert("x-test", "yes".parse().unwrap());
        request.extensions_mut().insert(9_u8);
        request.extensions_mut().insert(Context::new());
        let request = interceptor.call(request).unwrap();
        let state: Arc<String> = extract(&request).unwrap();
        assert!(Arc::ptr_eq(&state, &expected));
        assert_eq!(request.metadata().get("x-test").unwrap(), "yes");
        assert_eq!(request.extensions().get::<u8>(), Some(&9));
        let message_request = Request::from_parts(
            request.metadata().clone(),
            request.extensions().clone(),
            123_u32,
        );
        assert_eq!(
            extract::<Arc<String>, _>(&message_request)
                .unwrap()
                .as_str(),
            "hello"
        );
    }

    #[test]
    fn missing_state_never_exposes_type_names() {
        let request = Request::new(());
        let error = extract::<Arc<String>, _>(&request).unwrap_err();
        assert_eq!(error.code(), Code::Internal);
        assert_eq!(error.message(), "application state unavailable");
        let mut interceptor = ContextInterceptor::new(Context::new());
        let request = interceptor.call(request).unwrap();
        assert_eq!(
            extract::<Arc<String>, _>(&request).unwrap_err().message(),
            "application state unavailable"
        );
    }
}
