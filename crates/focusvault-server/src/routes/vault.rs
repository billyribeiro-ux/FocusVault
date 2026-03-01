use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use uuid::Uuid;

use focusvault_core::domain::*;

use crate::error::{ApiError, ApiResult};
use crate::state::AppState;

#[utoipa::path(
    get,
    path = "/api/v1/vault",
    params(
        ("query" = Option<String>, Query, description = "Search query"),
        ("status" = Option<String>, Query, description = "Filter by status"),
        ("limit" = Option<i64>, Query, description = "Max items to return"),
        ("offset" = Option<i64>, Query, description = "Offset for pagination"),
    ),
    responses(
        (status = 200, description = "List of vault items", body = Vec<VaultItem>)
    ),
    tag = "vault"
)]
pub async fn list_vault_items(
    State(state): State<AppState>,
    Query(filters): Query<VaultFilters>,
) -> ApiResult<Json<Vec<VaultItem>>> {
    let items = state.vault.list(filters).await.map_err(ApiError::from)?;
    Ok(Json(items))
}

#[utoipa::path(
    post,
    path = "/api/v1/vault",
    request_body = CreateVaultItem,
    responses(
        (status = 201, description = "Vault item created", body = VaultItem),
        (status = 400, description = "Validation error")
    ),
    tag = "vault"
)]
pub async fn create_vault_item(
    State(state): State<AppState>,
    Json(input): Json<CreateVaultItem>,
) -> ApiResult<(StatusCode, Json<VaultItem>)> {
    let item = state.vault.create(input).await.map_err(ApiError::from)?;
    Ok((StatusCode::CREATED, Json(item)))
}

#[utoipa::path(
    patch,
    path = "/api/v1/vault/{id}",
    params(("id" = Uuid, Path, description = "Vault item ID")),
    request_body = UpdateVaultItem,
    responses(
        (status = 200, description = "Vault item updated", body = VaultItem),
        (status = 404, description = "Not found")
    ),
    tag = "vault"
)]
pub async fn update_vault_item(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(update): Json<UpdateVaultItem>,
) -> ApiResult<Json<VaultItem>> {
    let item = state
        .vault
        .update(id, update)
        .await
        .map_err(ApiError::from)?;
    Ok(Json(item))
}

#[utoipa::path(
    delete,
    path = "/api/v1/vault/{id}",
    params(("id" = Uuid, Path, description = "Vault item ID")),
    responses(
        (status = 204, description = "Vault item deleted"),
        (status = 404, description = "Not found")
    ),
    tag = "vault"
)]
pub async fn delete_vault_item(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<StatusCode> {
    state.vault.delete(id).await.map_err(ApiError::from)?;
    Ok(StatusCode::NO_CONTENT)
}
