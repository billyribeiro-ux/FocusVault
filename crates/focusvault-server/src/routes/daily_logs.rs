use axum::extract::{Path, Query, State};
use axum::Json;
use chrono::NaiveDate;

use focusvault_core::domain::*;

use crate::error::{ApiError, ApiResult};
use crate::state::AppState;

#[utoipa::path(
    get,
    path = "/api/v1/logs",
    params(
        ("from" = Option<String>, Query, description = "Start date (YYYY-MM-DD)"),
        ("to" = Option<String>, Query, description = "End date (YYYY-MM-DD)"),
        ("limit" = Option<i64>, Query, description = "Max items"),
        ("offset" = Option<i64>, Query, description = "Offset"),
    ),
    responses((status = 200, description = "List daily logs", body = Vec<DailyLog>)),
    tag = "daily_logs"
)]
pub async fn list_daily_logs(
    State(state): State<AppState>,
    Query(filters): Query<DailyLogFilters>,
) -> ApiResult<Json<Vec<DailyLog>>> {
    let logs = state
        .daily_logs
        .list(filters)
        .await
        .map_err(ApiError::from)?;
    Ok(Json(logs))
}

#[utoipa::path(
    post,
    path = "/api/v1/logs",
    request_body = UpsertDailyLog,
    responses(
        (status = 200, description = "Daily log upserted", body = DailyLog),
        (status = 400, description = "Validation error")
    ),
    tag = "daily_logs"
)]
pub async fn upsert_daily_log(
    State(state): State<AppState>,
    Json(input): Json<UpsertDailyLog>,
) -> ApiResult<Json<DailyLog>> {
    let log = state
        .daily_logs
        .upsert(input)
        .await
        .map_err(ApiError::from)?;
    Ok(Json(log))
}

#[utoipa::path(
    patch,
    path = "/api/v1/logs/{date}",
    params(("date" = String, Path, description = "Date (YYYY-MM-DD)")),
    request_body = UpdateDailyLog,
    responses(
        (status = 200, description = "Daily log updated", body = DailyLog),
        (status = 404, description = "Not found")
    ),
    tag = "daily_logs"
)]
pub async fn update_daily_log(
    State(state): State<AppState>,
    Path(date): Path<NaiveDate>,
    Json(update): Json<UpdateDailyLog>,
) -> ApiResult<Json<DailyLog>> {
    let log = state
        .daily_logs
        .update(date, update)
        .await
        .map_err(ApiError::from)?;
    Ok(Json(log))
}
