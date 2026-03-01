use axum::Json;
use serde::Serialize;

#[derive(Serialize, utoipa::ToSchema)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
}

#[utoipa::path(
    get,
    path = "/api/v1/health",
    responses(
        (status = 200, description = "Service is healthy", body = HealthResponse)
    ),
    tag = "health"
)]
pub async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}
