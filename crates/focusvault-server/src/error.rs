use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use focusvault_core::error::DomainError;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiErrorBody {
    pub error: String,
    pub code: String,
}

pub struct ApiError(pub DomainError);

impl From<DomainError> for ApiError {
    fn from(err: DomainError) -> Self {
        Self(err)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, code) = match &self.0 {
            DomainError::Validation(_) => (StatusCode::BAD_REQUEST, "VALIDATION_ERROR"),
            DomainError::NotFound { .. } => (StatusCode::NOT_FOUND, "NOT_FOUND"),
            DomainError::Duplicate { .. } => (StatusCode::CONFLICT, "DUPLICATE"),
            DomainError::Conflict(_) => (StatusCode::CONFLICT, "CONFLICT"),
            DomainError::Unauthorized(_) => (StatusCode::UNAUTHORIZED, "UNAUTHORIZED"),
            DomainError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, "DATABASE_ERROR"),
            DomainError::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR"),
        };

        let body = ApiErrorBody {
            error: self.0.to_string(),
            code: code.to_string(),
        };

        (status, Json(body)).into_response()
    }
}

/// Convenience type alias for route handlers.
pub type ApiResult<T> = Result<T, ApiError>;
