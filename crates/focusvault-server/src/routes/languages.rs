use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use uuid::Uuid;

use focusvault_core::domain::*;

use crate::error::{ApiError, ApiResult};
use crate::state::AppState;

#[utoipa::path(
    get,
    path = "/api/v1/languages",
    responses((status = 200, description = "List language tracks", body = Vec<LanguageTrack>)),
    tag = "languages"
)]
pub async fn list_language_tracks(
    State(state): State<AppState>,
) -> ApiResult<Json<Vec<LanguageTrack>>> {
    let tracks = state.languages.list().await.map_err(ApiError::from)?;
    Ok(Json(tracks))
}

#[utoipa::path(
    post,
    path = "/api/v1/languages",
    request_body = CreateLanguageTrack,
    responses(
        (status = 201, description = "Language track created", body = LanguageTrack),
        (status = 400, description = "Validation error")
    ),
    tag = "languages"
)]
pub async fn create_language_track(
    State(state): State<AppState>,
    Json(input): Json<CreateLanguageTrack>,
) -> ApiResult<(StatusCode, Json<LanguageTrack>)> {
    let track = state
        .languages
        .create(input)
        .await
        .map_err(ApiError::from)?;
    Ok((StatusCode::CREATED, Json(track)))
}

#[utoipa::path(
    post,
    path = "/api/v1/languages/{id}/activate",
    params(("id" = Uuid, Path, description = "Language track ID")),
    responses(
        (status = 200, description = "Language track activated", body = LanguageTrack),
        (status = 409, description = "Another language track is active")
    ),
    tag = "languages"
)]
pub async fn activate_language_track(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<LanguageTrack>> {
    let track = state
        .languages
        .activate(id)
        .await
        .map_err(ApiError::from)?;
    Ok(Json(track))
}

#[utoipa::path(
    post,
    path = "/api/v1/languages/{id}/complete",
    params(("id" = Uuid, Path, description = "Language track ID")),
    responses((status = 200, description = "Language track completed", body = LanguageTrack)),
    tag = "languages"
)]
pub async fn complete_language_track(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<LanguageTrack>> {
    let track = state
        .languages
        .complete(id)
        .await
        .map_err(ApiError::from)?;
    Ok(Json(track))
}
