use axum::Json;
use focusvault_core::domain::*;
use uuid::Uuid;

use crate::error::ApiResult;

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
pub async fn register(Json(input): Json<CreateUser>) -> ApiResult<Json<AuthToken>> {
    // Phase 7 stub: In a full implementation this would:
    // 1. Hash the password with argon2
    // 2. Insert the user into the database
    // 3. Generate a JWT token
    let user = User {
        id: Uuid::new_v4(),
        email: input.email,
        display_name: input.display_name,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    Ok(Json(AuthToken {
        access_token: user.id.to_string(),
        token_type: "Bearer".into(),
        expires_in: 86400,
        user,
    }))
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
pub async fn login(Json(_input): Json<LoginRequest>) -> ApiResult<Json<AuthToken>> {
    // Phase 7 stub: In a full implementation this would:
    // 1. Look up user by email
    // 2. Verify password hash
    // 3. Generate a JWT token
    let user = User {
        id: Uuid::new_v4(),
        email: "stub@focusvault.local".into(),
        display_name: Some("Local User".into()),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    Ok(Json(AuthToken {
        access_token: user.id.to_string(),
        token_type: "Bearer".into(),
        expires_in: 86400,
        user,
    }))
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
pub async fn me(auth: crate::extractors::RequireAuth) -> ApiResult<Json<User>> {
    let user = User {
        id: auth.0.user_id,
        email: "user@focusvault.local".into(),
        display_name: Some("Authenticated User".into()),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    Ok(Json(user))
}
