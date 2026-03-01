use axum::body::Body;
use axum::http::{Method, Request, StatusCode};
use focusvault_server::config::{Config, DbBackend};
use http_body_util::BodyExt;
use serde_json::{json, Value};
use tower::ServiceExt;

async fn build_test_app() -> axum::Router {
    let config = Config {
        host: "127.0.0.1".into(),
        port: 0,
        database_url: "sqlite::memory:".into(),
        frontend_url: "http://localhost:5173".into(),
        db_backend: DbBackend::Sqlite,
        jwt_secret: "test-secret-key-for-integration-tests-only".into(),
    };
    focusvault_server::build_router(config).await
}

async fn body_to_json(body: Body) -> Value {
    let bytes = body.collect().await.unwrap().to_bytes();
    serde_json::from_slice(&bytes).unwrap()
}

fn json_request(method: Method, uri: &str, body: Option<Value>) -> Request<Body> {
    let mut builder = Request::builder().method(method).uri(uri);
    if body.is_some() {
        builder = builder.header("content-type", "application/json");
    }
    let body_str = body.map(|b| b.to_string()).unwrap_or_default();
    builder.body(Body::from(body_str)).unwrap()
}

fn auth_request(method: Method, uri: &str, token: &str, body: Option<Value>) -> Request<Body> {
    let mut builder = Request::builder()
        .method(method)
        .uri(uri)
        .header("authorization", format!("Bearer {token}"));
    if body.is_some() {
        builder = builder.header("content-type", "application/json");
    }
    let body_str = body.map(|b| b.to_string()).unwrap_or_default();
    builder.body(Body::from(body_str)).unwrap()
}

// ── Health ──

#[tokio::test]
async fn health_check_returns_ok() {
    let app = build_test_app().await;
    let req = json_request(Method::GET, "/api/v1/health", None);
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let body = body_to_json(resp.into_body()).await;
    assert_eq!(body["status"], "ok");
    assert!(body["version"].as_str().is_some());
}

// ── Auth ──

#[tokio::test]
async fn register_creates_user_and_returns_token() {
    let app = build_test_app().await;
    let req = json_request(
        Method::POST,
        "/api/v1/auth/register",
        Some(json!({
            "email": "alice@example.com",
            "password": "strongpassword123",
            "display_name": "Alice"
        })),
    );

    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let body = body_to_json(resp.into_body()).await;
    assert_eq!(body["token_type"], "Bearer");
    assert!(body["access_token"].as_str().unwrap().len() > 10);
    assert_eq!(body["user"]["email"], "alice@example.com");
    assert_eq!(body["user"]["display_name"], "Alice");
}

#[tokio::test]
async fn register_rejects_short_password() {
    let app = build_test_app().await;
    let req = json_request(
        Method::POST,
        "/api/v1/auth/register",
        Some(json!({
            "email": "bob@example.com",
            "password": "short"
        })),
    );

    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn register_rejects_invalid_email() {
    let app = build_test_app().await;
    let req = json_request(
        Method::POST,
        "/api/v1/auth/register",
        Some(json!({
            "email": "not-an-email",
            "password": "strongpassword123"
        })),
    );

    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn register_duplicate_email_returns_conflict() {
    let app = build_test_app().await;

    // Register first user
    let req = json_request(
        Method::POST,
        "/api/v1/auth/register",
        Some(json!({
            "email": "dup@example.com",
            "password": "strongpassword123"
        })),
    );
    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    // Register duplicate
    let req = json_request(
        Method::POST,
        "/api/v1/auth/register",
        Some(json!({
            "email": "dup@example.com",
            "password": "differentpassword123"
        })),
    );
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::CONFLICT);
}

#[tokio::test]
async fn login_returns_token_for_valid_credentials() {
    let app = build_test_app().await;

    // Register
    let req = json_request(
        Method::POST,
        "/api/v1/auth/register",
        Some(json!({
            "email": "login@example.com",
            "password": "strongpassword123"
        })),
    );
    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    // Login
    let req = json_request(
        Method::POST,
        "/api/v1/auth/login",
        Some(json!({
            "email": "login@example.com",
            "password": "strongpassword123"
        })),
    );
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let body = body_to_json(resp.into_body()).await;
    assert_eq!(body["token_type"], "Bearer");
    assert_eq!(body["user"]["email"], "login@example.com");
}

#[tokio::test]
async fn login_rejects_wrong_password() {
    let app = build_test_app().await;

    // Register
    let req = json_request(
        Method::POST,
        "/api/v1/auth/register",
        Some(json!({
            "email": "wrong@example.com",
            "password": "strongpassword123"
        })),
    );
    app.clone().oneshot(req).await.unwrap();

    // Login with wrong password
    let req = json_request(
        Method::POST,
        "/api/v1/auth/login",
        Some(json!({
            "email": "wrong@example.com",
            "password": "wrongpassword"
        })),
    );
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn me_returns_user_with_valid_token() {
    let app = build_test_app().await;

    // Register to get a token
    let req = json_request(
        Method::POST,
        "/api/v1/auth/register",
        Some(json!({
            "email": "me@example.com",
            "password": "strongpassword123",
            "display_name": "Me User"
        })),
    );
    let resp = app.clone().oneshot(req).await.unwrap();
    let body = body_to_json(resp.into_body()).await;
    let token = body["access_token"].as_str().unwrap();

    // Get /auth/me
    let req = auth_request(Method::GET, "/api/v1/auth/me", token, None);
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let body = body_to_json(resp.into_body()).await;
    assert_eq!(body["email"], "me@example.com");
}

#[tokio::test]
async fn me_returns_401_without_token() {
    let app = build_test_app().await;
    let req = json_request(Method::GET, "/api/v1/auth/me", None);
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

// ── Vault Items ──

#[tokio::test]
async fn vault_crud_lifecycle() {
    let app = build_test_app().await;

    // Create
    let req = json_request(
        Method::POST,
        "/api/v1/vault",
        Some(json!({
            "type": "link",
            "url": "https://rust-lang.org",
            "title": "Rust Language",
            "why": "Essential language for systems programming projects"
        })),
    );
    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::CREATED);

    let body = body_to_json(resp.into_body()).await;
    assert_eq!(body["type"], "link");
    assert_eq!(body["status"], "inbox");
    assert_eq!(body["hostname"], "rust-lang.org");
    let id = body["id"].as_str().unwrap().to_string();

    // List
    let req = json_request(Method::GET, "/api/v1/vault", None);
    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let body = body_to_json(resp.into_body()).await;
    assert_eq!(body.as_array().unwrap().len(), 1);

    // Update
    let req = json_request(
        Method::PATCH,
        &format!("/api/v1/vault/{id}"),
        Some(json!({ "status": "saved", "pinned": true })),
    );
    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let body = body_to_json(resp.into_body()).await;
    assert_eq!(body["status"], "saved");
    assert_eq!(body["pinned"], true);

    // Delete
    let req = json_request(Method::DELETE, &format!("/api/v1/vault/{id}"), None);
    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    // Verify deleted
    let req = json_request(Method::GET, "/api/v1/vault", None);
    let resp = app.oneshot(req).await.unwrap();
    let body = body_to_json(resp.into_body()).await;
    assert_eq!(body.as_array().unwrap().len(), 0);
}

#[tokio::test]
async fn vault_rejects_invalid_why() {
    let app = build_test_app().await;

    let req = json_request(
        Method::POST,
        "/api/v1/vault",
        Some(json!({
            "type": "note",
            "why": "short"
        })),
    );
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

// ── Missions ──

#[tokio::test]
async fn mission_lifecycle_with_single_active_rule() {
    let app = build_test_app().await;

    // Create mission A
    let req = json_request(
        Method::POST,
        "/api/v1/missions",
        Some(json!({ "name": "Mission A", "tab_limit": 5 })),
    );
    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::CREATED);
    let body = body_to_json(resp.into_body()).await;
    let id_a = body["id"].as_str().unwrap().to_string();
    assert_eq!(body["status"], "paused");

    // Activate A
    let req = json_request(
        Method::POST,
        &format!("/api/v1/missions/{id_a}/activate"),
        None,
    );
    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = body_to_json(resp.into_body()).await;
    assert_eq!(body["status"], "active");

    // Create and activate B — should deactivate A
    let req = json_request(
        Method::POST,
        "/api/v1/missions",
        Some(json!({ "name": "Mission B" })),
    );
    let resp = app.clone().oneshot(req).await.unwrap();
    let body = body_to_json(resp.into_body()).await;
    let id_b = body["id"].as_str().unwrap().to_string();

    let req = json_request(
        Method::POST,
        &format!("/api/v1/missions/{id_b}/activate"),
        None,
    );
    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    // List should show A paused, B active
    let req = json_request(Method::GET, "/api/v1/missions", None);
    let resp = app.oneshot(req).await.unwrap();
    let body = body_to_json(resp.into_body()).await;
    let missions = body.as_array().unwrap();
    assert_eq!(missions.len(), 2);
    let active_count = missions.iter().filter(|m| m["status"] == "active").count();
    assert_eq!(active_count, 1);
}

// ── Daily Logs ──

#[tokio::test]
async fn daily_log_today_and_update() {
    let app = build_test_app().await;

    // Get today (auto-creates)
    let req = json_request(Method::GET, "/api/v1/logs/today", None);
    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = body_to_json(resp.into_body()).await;
    assert_eq!(body["cycles_completed"], 0);
    let date = body["date"].as_str().unwrap().to_string();

    // Update today
    let req = json_request(
        Method::PATCH,
        &format!("/api/v1/logs/{date}"),
        Some(json!({
            "cycles_completed": 3,
            "watch_done": true,
            "build_done": true
        })),
    );
    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = body_to_json(resp.into_body()).await;
    assert_eq!(body["cycles_completed"], 3);
    assert_eq!(body["watch_done"], true);
    assert_eq!(body["build_done"], true);
    assert_eq!(body["prove_done"], false);
}

// ── Courses ──

#[tokio::test]
async fn course_create_activate_complete() {
    let app = build_test_app().await;

    // Create
    let req = json_request(
        Method::POST,
        "/api/v1/courses",
        Some(json!({
            "name": "Rust Advanced",
            "provider": "Udemy"
        })),
    );
    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::CREATED);
    let body = body_to_json(resp.into_body()).await;
    let id = body["id"].as_str().unwrap().to_string();
    assert_eq!(body["status"], "paused");

    // Activate
    let req = json_request(
        Method::POST,
        &format!("/api/v1/courses/{id}/activate"),
        None,
    );
    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    // Complete
    let req = json_request(
        Method::POST,
        &format!("/api/v1/courses/{id}/complete"),
        None,
    );
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = body_to_json(resp.into_body()).await;
    assert_eq!(body["status"], "completed");
}

// ── Language Tracks ──

#[tokio::test]
async fn language_track_create_activate_complete() {
    let app = build_test_app().await;

    let req = json_request(
        Method::POST,
        "/api/v1/languages",
        Some(json!({
            "name": "Portuguese",
            "weekly_goal_minutes": 300
        })),
    );
    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::CREATED);
    let body = body_to_json(resp.into_body()).await;
    let id = body["id"].as_str().unwrap().to_string();

    let req = json_request(
        Method::POST,
        &format!("/api/v1/languages/{id}/activate"),
        None,
    );
    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let req = json_request(
        Method::POST,
        &format!("/api/v1/languages/{id}/complete"),
        None,
    );
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = body_to_json(resp.into_body()).await;
    assert_eq!(body["status"], "completed");
}

// ── Projects ──

#[tokio::test]
async fn project_crud() {
    let app = build_test_app().await;

    // Create
    let req = json_request(
        Method::POST,
        "/api/v1/projects",
        Some(json!({ "name": "FocusVault", "color": "#3b82f6" })),
    );
    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::CREATED);
    let body = body_to_json(resp.into_body()).await;
    let id = body["id"].as_str().unwrap().to_string();
    assert_eq!(body["name"], "FocusVault");

    // Update
    let req = json_request(
        Method::PATCH,
        &format!("/api/v1/projects/{id}"),
        Some(json!({ "archived": true })),
    );
    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = body_to_json(resp.into_body()).await;
    assert_eq!(body["archived"], true);

    // List
    let req = json_request(Method::GET, "/api/v1/projects", None);
    let resp = app.oneshot(req).await.unwrap();
    let body = body_to_json(resp.into_body()).await;
    assert_eq!(body.as_array().unwrap().len(), 1);
}

// ── Vault URL normalization ──

#[tokio::test]
async fn vault_normalizes_urls() {
    let app = build_test_app().await;

    let req = json_request(
        Method::POST,
        "/api/v1/vault",
        Some(json!({
            "type": "link",
            "url": "https://example.com/page?utm_source=twitter&q=test",
            "why": "Testing URL normalization with tracking params"
        })),
    );
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::CREATED);
    let body = body_to_json(resp.into_body()).await;

    let url = body["url"].as_str().unwrap();
    assert!(!url.contains("utm_source"));
    assert!(url.contains("q=test"));
}

// ── Sync ──

#[tokio::test]
async fn sync_requires_auth() {
    let app = build_test_app().await;

    let req = json_request(
        Method::POST,
        "/api/v1/sync/push",
        Some(json!({
            "device_id": "test",
            "events": [],
            "last_synced_at": null
        })),
    );
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn sync_push_and_pull_roundtrip() {
    let app = build_test_app().await;

    // Register to get token
    let req = json_request(
        Method::POST,
        "/api/v1/auth/register",
        Some(json!({
            "email": "sync@example.com",
            "password": "strongpassword123"
        })),
    );
    let resp = app.clone().oneshot(req).await.unwrap();
    let body = body_to_json(resp.into_body()).await;
    let token = body["access_token"].as_str().unwrap().to_string();

    // Push an event
    let req = auth_request(
        Method::POST,
        "/api/v1/sync/push",
        &token,
        Some(json!({
            "device_id": "device-a",
            "events": [{
                "entity_type": "vault_item",
                "entity_id": "00000000-0000-0000-0000-000000000001",
                "action": "create",
                "payload": {"title": "test"},
                "timestamp": "2025-01-01T00:00:00Z"
            }],
            "last_synced_at": null
        })),
    );
    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = body_to_json(resp.into_body()).await;
    assert_eq!(body["accepted"], 1);
    assert!(body["conflicts"].as_array().unwrap().is_empty());

    // Pull from a different device should get the event
    let req = auth_request(
        Method::POST,
        "/api/v1/sync/pull",
        &token,
        Some(json!({
            "device_id": "device-b",
            "last_synced_at": null
        })),
    );
    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = body_to_json(resp.into_body()).await;
    assert_eq!(body["events"].as_array().unwrap().len(), 1);
    assert_eq!(body["has_more"], false);

    // Pull from the same device should get nothing
    let req = auth_request(
        Method::POST,
        "/api/v1/sync/pull",
        &token,
        Some(json!({
            "device_id": "device-a",
            "last_synced_at": null
        })),
    );
    let resp = app.clone().oneshot(req).await.unwrap();
    let body = body_to_json(resp.into_body()).await;
    assert_eq!(body["events"].as_array().unwrap().len(), 0);

    // Sync status
    let req = auth_request(Method::GET, "/api/v1/sync/status", &token, None);
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = body_to_json(resp.into_body()).await;
    assert_eq!(body["is_online"], true);
}
