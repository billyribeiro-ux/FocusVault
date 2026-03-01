use std::sync::Arc;
use uuid::Uuid;

use crate::domain::*;
use crate::error::{DomainError, DomainResult};
use crate::repository::Repository;

pub struct CourseService {
    repo: Arc<dyn Repository>,
}

impl CourseService {
    pub fn new(repo: Arc<dyn Repository>) -> Self {
        Self { repo }
    }

    pub async fn list(&self) -> DomainResult<Vec<Course>> {
        self.repo.list_courses().await
    }

    pub async fn create(&self, input: CreateCourse) -> DomainResult<Course> {
        if input.name.trim().is_empty() {
            return Err(DomainError::validation("Course name is required"));
        }
        self.repo.create_course(input).await
    }

    /// Activate a course. Enforces single-active: if another is active, caller must
    /// pause or complete it first.
    pub async fn activate(&self, id: Uuid) -> DomainResult<Course> {
        let current_active = self.repo.get_active_course().await?;

        if let Some(active) = current_active {
            if active.id == id {
                return Ok(active);
            }
            return Err(DomainError::conflict(format!(
                "Course '{}' is already active. Pause or complete it first.",
                active.name
            )));
        }

        self.repo.activate_course(id).await
    }

    pub async fn complete(&self, id: Uuid) -> DomainResult<Course> {
        self.repo.complete_course(id).await
    }
}
