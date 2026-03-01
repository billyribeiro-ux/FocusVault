use axum::extract::FromRequestParts;
use axum::http::request::Parts;

use crate::error::ApiError;
use crate::middleware::auth::AuthUser;

/// Extractor that requires authentication.
/// Returns 401 if no auth token is present.
pub struct RequireAuth(pub AuthUser);

impl<S: Send + Sync> FromRequestParts<S> for RequireAuth {
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<AuthUser>()
            .cloned()
            .map(RequireAuth)
            .ok_or_else(|| {
                ApiError(focusvault_core::error::DomainError::Unauthorized(
                    "Authentication required".into(),
                ))
            })
    }
}

/// Extractor for optional authentication.
/// Returns None if no auth token is present (local-first mode).
pub struct MaybeAuth(pub Option<AuthUser>);

impl<S: Send + Sync> FromRequestParts<S> for MaybeAuth {
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        Ok(MaybeAuth(parts.extensions.get::<AuthUser>().cloned()))
    }
}
