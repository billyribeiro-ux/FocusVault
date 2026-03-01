use std::sync::Arc;

use focusvault_core::repository::Repository;
use focusvault_core::services::*;

use crate::config::Config;

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub vault: Arc<VaultService>,
    pub missions: Arc<MissionService>,
    pub daily_logs: Arc<DailyLogService>,
    pub courses: Arc<CourseService>,
    pub languages: Arc<LanguageService>,
    pub projects: Arc<ProjectService>,
}

impl AppState {
    pub fn new(config: Config, repo: Arc<dyn Repository>) -> Self {
        Self {
            config,
            vault: Arc::new(VaultService::new(repo.clone())),
            missions: Arc::new(MissionService::new(repo.clone())),
            daily_logs: Arc::new(DailyLogService::new(repo.clone())),
            courses: Arc::new(CourseService::new(repo.clone())),
            languages: Arc::new(LanguageService::new(repo.clone())),
            projects: Arc::new(ProjectService::new(repo)),
        }
    }
}
