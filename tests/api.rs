//! Integration tests: requests go through the real router with `oneshot`, no port or network.

use axum::Router;
use axum::body::Body;
use axum::http::{Request, StatusCode, header};
use axum::routing::get;
use http_body_util::BodyExt;
use serde_json::{Value, json};
use tower::ServiceExt;

/// Sends one request to `router` and returns the status and the parsed JSON body.
async fn send(router: Router, method: &str, path: &str) -> (StatusCode, Value) {
    let request = Request::builder()
        .method(method)
        .uri(path)
        .body(Body::empty())
        .expect("valid request");
    let response = router.oneshot(request).await.expect("infallible service");

    let status = response.status();
    let content_type = response
        .headers()
        .get(header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or_default()
        .to_owned();
    assert_eq!(
        content_type, "application/json",
        "content type for {method} {path}"
    );

    let bytes = response
        .into_body()
        .collect()
        .await
        .expect("readable body")
        .to_bytes();
    let body = serde_json::from_slice(&bytes).expect("JSON body");
    (status, body)
}

#[tokio::test]
async fn health_returns_ok() {
    let (status, body) = send(rust_api_example::app(), "GET", "/health").await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, json!({ "status": "ok" }));
}

#[tokio::test]
async fn unknown_routes_return_json_404() {
    for (method, path) in [("GET", "/does-not-exist"), ("POST", "/health")] {
        let (status, body) = send(rust_api_example::app(), method, path).await;

        assert_eq!(status, StatusCode::NOT_FOUND, "{method} {path}");
        assert_eq!(body, json!({ "error": "Not found" }), "{method} {path}");
    }
}

#[tokio::test]
async fn panics_return_json_500() {
    async fn boom() -> &'static str {
        panic!("boom")
    }
    let router = rust_api_example::with_error_handling(Router::new().route("/panic", get(boom)));

    let (status, body) = send(router, "GET", "/panic").await;

    assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(body, json!({ "error": "Internal server error" }));
}
