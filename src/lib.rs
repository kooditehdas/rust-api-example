//! HTTP JSON API. [`app`] builds the router without binding a port, so tests can drive it
//! directly with `tower::ServiceExt::oneshot`.

mod error;
mod health;

use axum::Router;
use axum::routing::get;
use tower_http::catch_panic::CatchPanicLayer;

pub use error::ApiError;

/// Builds the API router with every route registered.
pub fn app() -> Router {
    with_error_handling(Router::new().route("/health", get(health::health)))
}

/// Adds the JSON error behavior to `router`: unknown paths and unsupported methods return the
/// JSON 404, and panics become a JSON 500.
///
/// Call it last, after every route is registered: axum layers only wrap routes added before them.
pub fn with_error_handling(router: Router) -> Router {
    router
        .fallback(error::not_found)
        .method_not_allowed_fallback(error::not_found)
        .layer(CatchPanicLayer::custom(error::panic_response))
}
