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
pub async fn list_projects(State(state): State<AppState>) -> ApiResult<Json<Vec<Project>>> {
    let projects = state.projects.list().await.map_err(ApiError::from)?;
    Ok(Json(projects))
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
    State(state): State<AppState>,
    Json(input): Json<CreateProject>,
) -> ApiResult<(StatusCode, Json<Project>)> {
    let project = state.projects.create(input).await.map_err(ApiError::from)?;
    Ok((StatusCode::CREATED, Json(project)))
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
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(update): Json<UpdateProject>,
) -> ApiResult<Json<Project>> {
    let project = state
        .projects
        .update(id, update)
        .await
        .map_err(ApiError::from)?;
    Ok(Json(project))
}
