use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use uuid::Uuid;

use focusvault_core::domain::*;

use crate::error::{ApiError, ApiResult};
use crate::state::AppState;

#[utoipa::path(
    get,
    path = "/api/v1/missions",
    responses((status = 200, description = "List missions", body = Vec<Mission>)),
    tag = "missions"
)]
pub async fn list_missions(State(state): State<AppState>) -> ApiResult<Json<Vec<Mission>>> {
    let missions = state.missions.list().await.map_err(ApiError::from)?;
    Ok(Json(missions))
}

#[utoipa::path(
    post,
    path = "/api/v1/missions",
    request_body = CreateMission,
    responses(
        (status = 201, description = "Mission created", body = Mission),
        (status = 400, description = "Validation error")
    ),
    tag = "missions"
)]
pub async fn create_mission(
    State(state): State<AppState>,
    Json(input): Json<CreateMission>,
) -> ApiResult<(StatusCode, Json<Mission>)> {
    let mission = state.missions.create(input).await.map_err(ApiError::from)?;
    Ok((StatusCode::CREATED, Json(mission)))
}

#[utoipa::path(
    patch,
    path = "/api/v1/missions/{id}",
    params(("id" = Uuid, Path, description = "Mission ID")),
    request_body = UpdateMission,
    responses(
        (status = 200, description = "Mission updated", body = Mission),
        (status = 404, description = "Not found")
    ),
    tag = "missions"
)]
pub async fn update_mission(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(update): Json<UpdateMission>,
) -> ApiResult<Json<Mission>> {
    let mission = state
        .missions
        .update(id, update)
        .await
        .map_err(ApiError::from)?;
    Ok(Json(mission))
}

#[utoipa::path(
    post,
    path = "/api/v1/missions/{id}/activate",
    params(("id" = Uuid, Path, description = "Mission ID")),
    responses(
        (status = 200, description = "Mission activated", body = Mission),
        (status = 409, description = "Another mission is active")
    ),
    tag = "missions"
)]
pub async fn activate_mission(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<Mission>> {
    let mission = state.missions.activate(id).await.map_err(ApiError::from)?;
    Ok(Json(mission))
}
