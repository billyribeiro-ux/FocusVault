use axum::http::header;
use axum::middleware;
use axum::routing::{get, patch, post};
use axum::Router;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::middleware::optional_auth;
use crate::routes::*;
use crate::state::AppState;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "FocusVault API",
        version = "0.1.0",
        description = "Local-first execution OS for high-output builders"
    ),
    paths(
        health::health_check,
        vault::list_vault_items,
        vault::create_vault_item,
        vault::update_vault_item,
        vault::delete_vault_item,
        missions::list_missions,
        missions::create_mission,
        missions::update_mission,
        missions::activate_mission,
        daily_logs::get_today,
        daily_logs::list_daily_logs,
        daily_logs::upsert_daily_log,
        daily_logs::update_daily_log,
        courses::list_courses,
        courses::create_course,
        courses::activate_course,
        courses::complete_course,
        languages::list_language_tracks,
        languages::create_language_track,
        languages::activate_language_track,
        languages::complete_language_track,
        projects::list_projects,
        projects::create_project,
        projects::update_project,
        auth::register,
        auth::login,
        auth::me,
        sync::sync_push,
        sync::sync_pull,
        sync::sync_status,
    ),
    components(schemas(
        health::HealthResponse,
        focusvault_core::domain::VaultItem,
        focusvault_core::domain::CreateVaultItem,
        focusvault_core::domain::UpdateVaultItem,
        focusvault_core::domain::VaultFilters,
        focusvault_core::domain::VaultItemType,
        focusvault_core::domain::VaultItemStatus,
        focusvault_core::domain::Priority,
        focusvault_core::domain::CaptureSource,
        focusvault_core::domain::Mission,
        focusvault_core::domain::CreateMission,
        focusvault_core::domain::UpdateMission,
        focusvault_core::domain::MissionStatus,
        focusvault_core::domain::WeeklyTargets,
        focusvault_core::domain::MissionKpis,
        focusvault_core::domain::DailyLog,
        focusvault_core::domain::UpsertDailyLog,
        focusvault_core::domain::UpdateDailyLog,
        focusvault_core::domain::DailyLogFilters,
        focusvault_core::domain::Mood,
        focusvault_core::domain::Course,
        focusvault_core::domain::CreateCourse,
        focusvault_core::domain::UpdateCourse,
        focusvault_core::domain::CourseStatus,
        focusvault_core::domain::LanguageTrack,
        focusvault_core::domain::CreateLanguageTrack,
        focusvault_core::domain::UpdateLanguageTrack,
        focusvault_core::domain::LanguageTrackStatus,
        focusvault_core::domain::Project,
        focusvault_core::domain::CreateProject,
        focusvault_core::domain::UpdateProject,
        focusvault_core::domain::User,
        focusvault_core::domain::CreateUser,
        focusvault_core::domain::LoginRequest,
        focusvault_core::domain::AuthToken,
        focusvault_core::domain::ApiKey,
        focusvault_core::domain::CreateApiKey,
        focusvault_core::domain::ApiKeyCreated,
        focusvault_core::domain::SyncPushRequest,
        focusvault_core::domain::SyncPushEvent,
        focusvault_core::domain::SyncPushResponse,
        focusvault_core::domain::SyncConflict,
        focusvault_core::domain::SyncPullRequest,
        focusvault_core::domain::SyncPullResponse,
        focusvault_core::domain::SyncEvent,
        focusvault_core::domain::SyncEntityType,
        focusvault_core::domain::SyncAction,
        focusvault_core::domain::SyncStatus,
    )),
    tags(
        (name = "health", description = "Health check"),
        (name = "vault", description = "Vault item management"),
        (name = "missions", description = "Mission management"),
        (name = "daily_logs", description = "Daily log tracking"),
        (name = "courses", description = "Course tracking"),
        (name = "languages", description = "Language track management"),
        (name = "projects", description = "Project management"),
        (name = "auth", description = "Authentication"),
        (name = "sync", description = "Offline-first sync"),
    )
)]
pub struct ApiDoc;

pub fn create_router(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(vec![
            header::CONTENT_TYPE,
            header::AUTHORIZATION,
            header::ACCEPT,
        ]);

    let api_routes = Router::new()
        // Health
        .route("/health", get(health::health_check))
        // Vault
        .route(
            "/vault",
            get(vault::list_vault_items).post(vault::create_vault_item),
        )
        .route(
            "/vault/{id}",
            patch(vault::update_vault_item).delete(vault::delete_vault_item),
        )
        // Missions
        .route(
            "/missions",
            get(missions::list_missions).post(missions::create_mission),
        )
        .route("/missions/{id}", patch(missions::update_mission))
        .route("/missions/{id}/activate", post(missions::activate_mission))
        // Daily Logs
        .route("/logs/today", get(daily_logs::get_today))
        .route(
            "/logs",
            get(daily_logs::list_daily_logs).post(daily_logs::upsert_daily_log),
        )
        .route("/logs/{date}", patch(daily_logs::update_daily_log))
        // Courses
        .route(
            "/courses",
            get(courses::list_courses).post(courses::create_course),
        )
        .route("/courses/{id}/activate", post(courses::activate_course))
        .route("/courses/{id}/complete", post(courses::complete_course))
        // Languages
        .route(
            "/languages",
            get(languages::list_language_tracks).post(languages::create_language_track),
        )
        .route(
            "/languages/{id}/activate",
            post(languages::activate_language_track),
        )
        .route(
            "/languages/{id}/complete",
            post(languages::complete_language_track),
        )
        // Projects
        .route(
            "/projects",
            get(projects::list_projects).post(projects::create_project),
        )
        .route("/projects/{id}", patch(projects::update_project))
        // Auth
        .route("/auth/register", post(auth::register))
        .route("/auth/login", post(auth::login))
        .route("/auth/me", get(auth::me))
        // Sync
        .route("/sync/push", post(sync::sync_push))
        .route("/sync/pull", post(sync::sync_pull))
        .route("/sync/status", get(sync::sync_status))
        // Apply optional auth middleware to all API routes
        .layer(middleware::from_fn(optional_auth));

    Router::new()
        .merge(SwaggerUi::new("/api/docs").url("/api/docs/openapi.json", ApiDoc::openapi()))
        .nest("/api/v1", api_routes)
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
