use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use uuid::Uuid;

use focusvault_core::domain::*;

use crate::error::{ApiError, ApiResult};
use crate::state::AppState;

#[utoipa::path(
    get,
    path = "/api/v1/courses",
    responses((status = 200, description = "List courses", body = Vec<Course>)),
    tag = "courses"
)]
pub async fn list_courses(State(state): State<AppState>) -> ApiResult<Json<Vec<Course>>> {
    let courses = state.courses.list().await.map_err(ApiError::from)?;
    Ok(Json(courses))
}

#[utoipa::path(
    post,
    path = "/api/v1/courses",
    request_body = CreateCourse,
    responses(
        (status = 201, description = "Course created", body = Course),
        (status = 400, description = "Validation error")
    ),
    tag = "courses"
)]
pub async fn create_course(
    State(state): State<AppState>,
    Json(input): Json<CreateCourse>,
) -> ApiResult<(StatusCode, Json<Course>)> {
    let course = state.courses.create(input).await.map_err(ApiError::from)?;
    Ok((StatusCode::CREATED, Json(course)))
}

#[utoipa::path(
    post,
    path = "/api/v1/courses/{id}/activate",
    params(("id" = Uuid, Path, description = "Course ID")),
    responses(
        (status = 200, description = "Course activated", body = Course),
        (status = 409, description = "Another course is active")
    ),
    tag = "courses"
)]
pub async fn activate_course(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<Course>> {
    let course = state.courses.activate(id).await.map_err(ApiError::from)?;
    Ok(Json(course))
}

#[utoipa::path(
    post,
    path = "/api/v1/courses/{id}/complete",
    params(("id" = Uuid, Path, description = "Course ID")),
    responses((status = 200, description = "Course completed", body = Course)),
    tag = "courses"
)]
pub async fn complete_course(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<Course>> {
    let course = state.courses.complete(id).await.map_err(ApiError::from)?;
    Ok(Json(course))
}
