use axum::Json;
use focusvault_core::domain::*;

use crate::error::ApiResult;
use crate::extractors::RequireAuth;

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
    _auth: RequireAuth,
    Json(request): Json<SyncPushRequest>,
) -> ApiResult<Json<SyncPushResponse>> {
    // Phase 7 stub: In a full implementation this would:
    // 1. Validate each event against the server's current state
    // 2. Apply non-conflicting changes
    // 3. Return conflicts for last-write-wins resolution

    let accepted = request.events.len() as i32;

    Ok(Json(SyncPushResponse {
        accepted,
        conflicts: vec![],
        server_time: chrono::Utc::now(),
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
    _auth: RequireAuth,
    Json(_request): Json<SyncPullRequest>,
) -> ApiResult<Json<SyncPullResponse>> {
    // Phase 7 stub: In a full implementation this would:
    // 1. Query all sync events after last_synced_at for this user
    // 2. Exclude events from the requesting device
    // 3. Return paginated results

    Ok(Json(SyncPullResponse {
        events: vec![],
        server_time: chrono::Utc::now(),
        has_more: false,
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
pub async fn sync_status(_auth: RequireAuth) -> ApiResult<Json<SyncStatus>> {
    Ok(Json(SyncStatus {
        device_id: "server".into(),
        last_synced_at: Some(chrono::Utc::now()),
        pending_changes: 0,
        is_online: true,
    }))
}
