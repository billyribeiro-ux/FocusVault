use axum::extract::State;
use axum::Json;
use chrono::Utc;
use focusvault_core::domain::*;
use uuid::Uuid;

use crate::error::ApiResult;
use crate::extractors::RequireAuth;
use crate::state::AppState;

const SYNC_PAGE_SIZE: i64 = 100;

/// Push local changes to the server.
#[utoipa::path(
    post,
    path = "/api/v1/sync/push",
    request_body = SyncPushRequest,
    responses(
        (status = 200, description = "Changes pushed", body = SyncPushResponse),
        (status = 401, description = "Not authenticated")
    ),
    tag = "sync"
)]
pub async fn sync_push(
    State(state): State<AppState>,
    auth: RequireAuth,
    Json(request): Json<SyncPushRequest>,
) -> ApiResult<Json<SyncPushResponse>> {
    let mut accepted = 0i32;
    let conflicts: Vec<SyncConflict> = vec![];

    for event in &request.events {
        let sync_event = SyncEvent {
            id: Uuid::new_v4(),
            user_id: auth.0.user_id,
            entity_type: event.entity_type.clone(),
            entity_id: event.entity_id,
            action: event.action.clone(),
            payload: event.payload.clone(),
            device_id: request.device_id.clone(),
            timestamp: event.timestamp,
        };

        // Last-write-wins: store the event. The pull side delivers events
        // chronologically so newer writes naturally overwrite older ones.
        state.repo.insert_sync_event(&sync_event).await?;
        accepted += 1;
    }

    Ok(Json(SyncPushResponse {
        accepted,
        conflicts,
        server_time: Utc::now(),
    }))
}

/// Pull changes from the server since last sync.
#[utoipa::path(
    post,
    path = "/api/v1/sync/pull",
    request_body = SyncPullRequest,
    responses(
        (status = 200, description = "Changes pulled", body = SyncPullResponse),
        (status = 401, description = "Not authenticated")
    ),
    tag = "sync"
)]
pub async fn sync_pull(
    State(state): State<AppState>,
    auth: RequireAuth,
    Json(request): Json<SyncPullRequest>,
) -> ApiResult<Json<SyncPullResponse>> {
    let events = state
        .repo
        .list_sync_events_since(
            auth.0.user_id,
            request.last_synced_at,
            &request.device_id,
            SYNC_PAGE_SIZE + 1,
        )
        .await?;

    let has_more = events.len() as i64 > SYNC_PAGE_SIZE;
    let events: Vec<SyncEvent> = events.into_iter().take(SYNC_PAGE_SIZE as usize).collect();

    Ok(Json(SyncPullResponse {
        events,
        server_time: Utc::now(),
        has_more,
    }))
}

/// Get sync status for the current device.
#[utoipa::path(
    get,
    path = "/api/v1/sync/status",
    responses(
        (status = 200, description = "Sync status", body = SyncStatus),
        (status = 401, description = "Not authenticated")
    ),
    tag = "sync"
)]
pub async fn sync_status(
    State(state): State<AppState>,
    auth: RequireAuth,
) -> ApiResult<Json<SyncStatus>> {
    let pending = state
        .repo
        .count_pending_sync_events(auth.0.user_id, "server")
        .await?;

    Ok(Json(SyncStatus {
        device_id: "server".into(),
        last_synced_at: Some(Utc::now()),
        pending_changes: pending as i32,
        is_online: true,
    }))
}
