use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct VersionBody {
    name: &'static str,
    version: &'static str,
}

pub(crate) async fn version() -> Json<VersionBody> {
    Json(VersionBody {
        name: env!("CARGO_PKG_NAME"),
        version: env!("CARGO_PKG_VERSION"),
    })
}
