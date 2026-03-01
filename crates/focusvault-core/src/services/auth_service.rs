use std::sync::Arc;

use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::{AuthToken, CreateUser, LoginRequest, User, UserRow};
use crate::error::{DomainError, DomainResult};
use crate::repository::Repository;

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    pub sub: String,
    pub email: String,
    pub exp: i64,
    pub iat: i64,
}

pub struct AuthService {
    repo: Arc<dyn Repository>,
    jwt_secret: String,
    jwt_expiry_hours: i64,
}

impl AuthService {
    pub fn new(repo: Arc<dyn Repository>, jwt_secret: String) -> Self {
        Self {
            repo,
            jwt_secret,
            jwt_expiry_hours: 24,
        }
    }

    pub async fn register(&self, input: CreateUser) -> DomainResult<AuthToken> {
        let email = input.email.trim().to_lowercase();

        if email.is_empty() || !email.contains('@') {
            return Err(DomainError::validation("Invalid email address"));
        }
        if input.password.len() < 8 {
            return Err(DomainError::validation(
                "Password must be at least 8 characters",
            ));
        }

        if self.repo.get_user_by_email(&email).await?.is_some() {
            return Err(DomainError::duplicate("User", email));
        }

        let password_hash = hash_password(&input.password)?;
        let user_id = Uuid::new_v4();

        let row = self
            .repo
            .create_user(
                user_id,
                &email,
                &password_hash,
                input.display_name.as_deref(),
            )
            .await?;

        let user = row.into_user();
        let token = self.generate_token(&user)?;

        Ok(AuthToken {
            access_token: token,
            token_type: "Bearer".into(),
            expires_in: self.jwt_expiry_hours * 3600,
            user,
        })
    }

    pub async fn login(&self, input: LoginRequest) -> DomainResult<AuthToken> {
        let email = input.email.trim().to_lowercase();

        let row = self
            .repo
            .get_user_by_email(&email)
            .await?
            .ok_or_else(|| DomainError::Unauthorized("Invalid credentials".into()))?;

        verify_password(&input.password, &row.password_hash)?;

        let user = row.into_user();
        let token = self.generate_token(&user)?;

        Ok(AuthToken {
            access_token: token,
            token_type: "Bearer".into(),
            expires_in: self.jwt_expiry_hours * 3600,
            user,
        })
    }

    pub async fn get_user(&self, user_id: Uuid) -> DomainResult<User> {
        self.repo
            .get_user_by_id(user_id)
            .await?
            .map(UserRow::into_user)
            .ok_or_else(|| DomainError::not_found("User", user_id.to_string()))
    }

    pub fn validate_token(&self, token: &str) -> DomainResult<JwtClaims> {
        let key = DecodingKey::from_secret(self.jwt_secret.as_bytes());
        let validation = Validation::default();

        decode::<JwtClaims>(token, &key, &validation)
            .map(|data| data.claims)
            .map_err(|e| DomainError::Unauthorized(format!("Invalid token: {e}")))
    }

    fn generate_token(&self, user: &User) -> DomainResult<String> {
        let now = Utc::now();
        let exp = now + Duration::hours(self.jwt_expiry_hours);

        let claims = JwtClaims {
            sub: user.id.to_string(),
            email: user.email.clone(),
            exp: exp.timestamp(),
            iat: now.timestamp(),
        };

        let key = EncodingKey::from_secret(self.jwt_secret.as_bytes());
        encode(&Header::default(), &claims, &key)
            .map_err(|e| DomainError::Internal(format!("Failed to generate token: {e}")))
    }
}

fn hash_password(password: &str) -> DomainResult<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2
        .hash_password(password.as_bytes(), &salt)
        .map(|h| h.to_string())
        .map_err(|e| DomainError::Internal(format!("Failed to hash password: {e}")))
}

fn verify_password(password: &str, hash: &str) -> DomainResult<()> {
    let parsed = PasswordHash::new(hash)
        .map_err(|e| DomainError::Internal(format!("Invalid password hash: {e}")))?;
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .map_err(|_| DomainError::Unauthorized("Invalid credentials".into()))
}
