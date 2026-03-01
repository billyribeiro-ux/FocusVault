use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("Validation error: {0}")]
    Validation(String),

    #[error("{entity} not found: {id}")]
    NotFound { entity: &'static str, id: String },

    #[error("Duplicate {entity}: {detail}")]
    Duplicate { entity: &'static str, detail: String },

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Database error: {0}")]
    Database(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

impl DomainError {
    pub fn validation(msg: impl Into<String>) -> Self {
        Self::Validation(msg.into())
    }

    pub fn not_found(entity: &'static str, id: impl Into<String>) -> Self {
        Self::NotFound {
            entity,
            id: id.into(),
        }
    }

    pub fn duplicate(entity: &'static str, detail: impl Into<String>) -> Self {
        Self::Duplicate {
            entity,
            detail: detail.into(),
        }
    }

    pub fn conflict(msg: impl Into<String>) -> Self {
        Self::Conflict(msg.into())
    }
}

pub type DomainResult<T> = Result<T, DomainError>;
