use std::any::Any;

use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

/// An error response: the status code plus a JSON body `{ "error": "<message>" }`.
#[derive(Debug)]
pub struct ApiError {
    status: StatusCode,
    message: String,
}

impl ApiError {
    /// Creates an error response with the given status and message.
    pub fn new(status: StatusCode, message: impl Into<String>) -> Self {
        Self {
            status,
            message: message.into(),
        }
    }
}

#[derive(Serialize)]
struct ErrorBody<'a> {
    error: &'a str,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (
            self.status,
            Json(ErrorBody {
                error: &self.message,
            }),
        )
            .into_response()
    }
}

pub(crate) async fn not_found() -> ApiError {
    ApiError::new(StatusCode::NOT_FOUND, "Not found")
}

/// Turns a panic in a handler into a JSON 500 instead of a dropped connection.
#[allow(clippy::needless_pass_by_value)] // signature required by CatchPanicLayer::custom
pub(crate) fn panic_response(_panic: Box<dyn Any + Send + 'static>) -> Response {
    eprintln!("handler panicked");
    ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
}
