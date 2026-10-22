use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct HealthBody {
    status: &'static str,
}

pub(crate) async fn health() -> Json<HealthBody> {
    Json(HealthBody { status: "OK" })
}
