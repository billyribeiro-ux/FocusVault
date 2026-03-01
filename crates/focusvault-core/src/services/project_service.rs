use std::sync::Arc;
use uuid::Uuid;

use crate::domain::*;
use crate::error::{DomainError, DomainResult};
use crate::repository::Repository;

pub struct ProjectService {
    repo: Arc<dyn Repository>,
}

impl ProjectService {
    pub fn new(repo: Arc<dyn Repository>) -> Self {
        Self { repo }
    }

    pub async fn list(&self) -> DomainResult<Vec<Project>> {
        self.repo.list_projects().await
    }

    pub async fn create(&self, input: CreateProject) -> DomainResult<Project> {
        if input.name.trim().is_empty() {
            return Err(DomainError::validation("Project name is required"));
        }
        self.repo.create_project(input).await
    }

    pub async fn update(&self, id: Uuid, update: UpdateProject) -> DomainResult<Project> {
        self.repo.update_project(id, update).await
    }
}
