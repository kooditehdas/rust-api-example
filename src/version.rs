use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct VersionBody {
    version: &'static str,
}

pub(crate) async fn version() -> Json<VersionBody> {
    Json(VersionBody {
        version: env!("CARGO_PKG_VERSION"),
    })
}
