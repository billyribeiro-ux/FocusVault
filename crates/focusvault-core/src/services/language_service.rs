use std::sync::Arc;
use uuid::Uuid;

use crate::domain::*;
use crate::error::{DomainError, DomainResult};
use crate::repository::Repository;

pub struct LanguageService {
    repo: Arc<dyn Repository>,
}

impl LanguageService {
    pub fn new(repo: Arc<dyn Repository>) -> Self {
        Self { repo }
    }

    pub async fn list(&self) -> DomainResult<Vec<LanguageTrack>> {
        self.repo.list_language_tracks().await
    }

    pub async fn create(&self, input: CreateLanguageTrack) -> DomainResult<LanguageTrack> {
        if input.name.trim().is_empty() {
            return Err(DomainError::validation(
                "Language track name is required",
            ));
        }
        self.repo.create_language_track(input).await
    }

    /// Activate a language track. Enforces single-active.
    pub async fn activate(&self, id: Uuid) -> DomainResult<LanguageTrack> {
        let current_active = self.repo.get_active_language_track().await?;

        if let Some(active) = current_active {
            if active.id == id {
                return Ok(active);
            }
            return Err(DomainError::conflict(format!(
                "Language track '{}' is already active. Pause or complete it first.",
                active.name
            )));
        }

        self.repo.activate_language_track(id).await
    }

    pub async fn complete(&self, id: Uuid) -> DomainResult<LanguageTrack> {
        self.repo.complete_language_track(id).await
    }
}
