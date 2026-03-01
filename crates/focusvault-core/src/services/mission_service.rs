use std::sync::Arc;
use uuid::Uuid;

use crate::domain::*;
use crate::error::{DomainError, DomainResult};
use crate::repository::Repository;

pub struct MissionService {
    repo: Arc<dyn Repository>,
}

impl MissionService {
    pub fn new(repo: Arc<dyn Repository>) -> Self {
        Self { repo }
    }

    pub async fn list(&self) -> DomainResult<Vec<Mission>> {
        self.repo.list_missions().await
    }

    pub async fn get(&self, id: Uuid) -> DomainResult<Mission> {
        self.repo.get_mission(id).await
    }

    pub async fn create(&self, input: CreateMission) -> DomainResult<Mission> {
        if input.name.trim().is_empty() {
            return Err(DomainError::validation("Mission name is required"));
        }

        self.repo.create_mission(input).await
    }

    pub async fn update(&self, id: Uuid, update: UpdateMission) -> DomainResult<Mission> {
        // If trying to set status to active directly via update, enforce single-active
        if let Some(MissionStatus::Active) = &update.status {
            return Err(DomainError::validation(
                "Use the activate endpoint to set a mission to active",
            ));
        }

        self.repo.update_mission(id, update).await
    }

    /// Activates a mission. Enforces single-active: if another mission is active,
    /// the caller must pause or complete it first.
    pub async fn activate(&self, id: Uuid) -> DomainResult<Mission> {
        let current_active = self.repo.get_active_mission().await?;

        if let Some(active) = current_active {
            if active.id == id {
                // Already active, no-op
                return Ok(active);
            }
            return Err(DomainError::conflict(format!(
                "Mission '{}' is already active. Pause or complete it first.",
                active.name
            )));
        }

        self.repo.activate_mission(id).await
    }
}
