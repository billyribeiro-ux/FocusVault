use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use uuid::Uuid;

use focusvault_core::domain::*;

use crate::error::{ApiError, ApiResult};
use crate::state::AppState;

#[utoipa::path(
    get,
    path = "/api/v1/projects",
    responses((status = 200, description = "List projects", body = Vec<Project>)),
    tag = "projects"
)]
pub async fn list_projects(
    State(state): State<AppState>,
) -> ApiResult<Json<Vec<Project>>> {
    let projects = state
        .vault
        .stale_items(0)
        .await
        .map_err(ApiError::from)?; // placeholder
    // TODO: Wire up project service in Phase 2
    let _ = projects;
    Ok(Json(vec![]))
}

#[utoipa::path(
    post,
    path = "/api/v1/projects",
    request_body = CreateProject,
    responses(
        (status = 201, description = "Project created", body = Project),
        (status = 400, description = "Validation error")
    ),
    tag = "projects"
)]
pub async fn create_project(
    State(_state): State<AppState>,
    Json(_input): Json<CreateProject>,
) -> ApiResult<(StatusCode, Json<Project>)> {
    // TODO: Wire up in Phase 2
    Err(ApiError(focusvault_core::error::DomainError::Internal(
        "Projects not yet implemented".into(),
    )))
}

#[utoipa::path(
    patch,
    path = "/api/v1/projects/{id}",
    params(("id" = Uuid, Path, description = "Project ID")),
    request_body = UpdateProject,
    responses(
        (status = 200, description = "Project updated", body = Project),
        (status = 404, description = "Not found")
    ),
    tag = "projects"
)]
pub async fn update_project(
    State(_state): State<AppState>,
    Path(_id): Path<Uuid>,
    Json(_update): Json<UpdateProject>,
) -> ApiResult<Json<Project>> {
    // TODO: Wire up in Phase 2
    Err(ApiError(focusvault_core::error::DomainError::Internal(
        "Projects not yet implemented".into(),
    )))
}
