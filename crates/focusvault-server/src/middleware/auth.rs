use axum::extract::Request;
use axum::http::header::AUTHORIZATION;
use axum::middleware::Next;
use axum::response::Response;
use uuid::Uuid;

/// Authenticated user identity extracted from the request.
#[derive(Debug, Clone)]
pub struct AuthUser {
    pub user_id: Uuid,
    pub source: AuthSource,
}

#[derive(Debug, Clone)]
pub enum AuthSource {
    Bearer,
    ApiKey,
}

/// Middleware that extracts an optional auth token.
/// In local-first mode (no auth configured), all requests pass through.
/// When auth is enabled, the token is validated and the user identity is
/// attached as a request extension.
pub async fn optional_auth(mut req: Request, next: Next) -> Response {
    if let Some(auth_header) = req.headers().get(AUTHORIZATION) {
        if let Ok(header_str) = auth_header.to_str() {
            if let Some(token) = header_str.strip_prefix("Bearer ") {
                // In a full implementation, this would verify a JWT or session token.
                // For now, treat any valid UUID in the bearer token as the user ID.
                if let Ok(user_id) = Uuid::parse_str(token) {
                    req.extensions_mut().insert(AuthUser {
                        user_id,
                        source: AuthSource::Bearer,
                    });
                }
            } else if let Some(key) = header_str.strip_prefix("ApiKey ") {
                // In a full implementation, this would look up the API key in the database.
                // For now, treat any valid UUID as the user ID.
                if let Ok(user_id) = Uuid::parse_str(key) {
                    req.extensions_mut().insert(AuthUser {
                        user_id,
                        source: AuthSource::ApiKey,
                    });
                }
            }
        }
    }

    next.run(req).await
}
