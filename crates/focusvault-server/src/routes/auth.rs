use axum::extract::State;
use axum::Json;
use focusvault_core::domain::*;

use crate::error::ApiResult;
use crate::state::AppState;

/// Register a new user account.
#[utoipa::path(
    post,
    path = "/api/v1/auth/register",
    request_body = CreateUser,
    responses(
        (status = 201, description = "User registered", body = AuthToken),
        (status = 409, description = "Email already taken")
    ),
    tag = "auth"
)]
pub async fn register(
    State(state): State<AppState>,
    Json(input): Json<CreateUser>,
) -> ApiResult<Json<AuthToken>> {
    let token = state.auth.register(input).await?;
    Ok(Json(token))
}

/// Authenticate with email and password.
#[utoipa::path(
    post,
    path = "/api/v1/auth/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = AuthToken),
        (status = 401, description = "Invalid credentials")
    ),
    tag = "auth"
)]
pub async fn login(
    State(state): State<AppState>,
    Json(input): Json<LoginRequest>,
) -> ApiResult<Json<AuthToken>> {
    let token = state.auth.login(input).await?;
    Ok(Json(token))
}

/// Get the current authenticated user.
#[utoipa::path(
    get,
    path = "/api/v1/auth/me",
    responses(
        (status = 200, description = "Current user", body = User),
        (status = 401, description = "Not authenticated")
    ),
    tag = "auth"
)]
pub async fn me(
    State(state): State<AppState>,
    auth: crate::extractors::RequireAuth,
) -> ApiResult<Json<User>> {
    let user = state.auth.get_user(auth.0.user_id).await?;
    Ok(Json(user))
}
