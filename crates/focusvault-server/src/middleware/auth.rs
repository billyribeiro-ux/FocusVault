use axum::extract::Request;
use axum::http::header::AUTHORIZATION;
use axum::middleware::Next;
use axum::response::Response;
use jsonwebtoken::{decode, DecodingKey, Validation};
use uuid::Uuid;

use focusvault_core::services::auth_service::JwtClaims;

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
/// When auth is enabled, the JWT is validated and the user identity is
/// attached as a request extension.
pub async fn optional_auth(mut req: Request, next: Next) -> Response {
    if let Some(auth_header) = req.headers().get(AUTHORIZATION) {
        if let Ok(header_str) = auth_header.to_str() {
            if let Some(token) = header_str.strip_prefix("Bearer ") {
                // Try JWT validation first
                let jwt_secret = extract_jwt_secret(&req);
                if let Some(secret) = jwt_secret {
                    let key = DecodingKey::from_secret(secret.as_bytes());
                    if let Ok(token_data) = decode::<JwtClaims>(token, &key, &Validation::default()) {
                        if let Ok(user_id) = Uuid::parse_str(&token_data.claims.sub) {
                            req.extensions_mut().insert(AuthUser {
                                user_id,
                                source: AuthSource::Bearer,
                            });
                        }
                    }
                }
            } else if let Some(key) = header_str.strip_prefix("ApiKey ") {
                // API key auth — validated against DB in the future
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

fn extract_jwt_secret(req: &Request) -> Option<String> {
    // The JWT secret is injected via the axum::Extension<AppState> layer
    req.extensions()
        .get::<crate::state::AppState>()
        .map(|state| state.config.jwt_secret.clone())
}
