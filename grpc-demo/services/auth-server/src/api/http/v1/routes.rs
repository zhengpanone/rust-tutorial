use crate::app::state::AppState;
use crate::application::handlers::auth_handler;
use crate::application::handlers::health_handler::health_check;
use axum::Router;
use axum::routing::{delete, get, post, put};
use std::sync::Arc;
use tracing::info;

pub fn configure_routes() -> Router<Arc<AppState>> {
    info!("🚀 Configuring API routes...");
    let router = Router::new()
        // 公共路由(无需认证)
        .merge(config_public_routes());
    todo!()
}

/// 配置公共路由(无需认证)
fn config_public_routes() -> Router<Arc<AppState>> {
    info!("🚀 Configuring public API routes...");
    Router::new()
        .route("/health", get(health_check))
        .route("/auth/login", post(auth_handler::login))
        .route("/auth/register", post(auth_handler::register))
        .route("/auth/refresh", post(auth_handler::refresh_token))
        .route("/auth/forgot-password", post(auth_handler::forgot_password))
        .route("/auth/reset-password", post(auth_handler::reset_password))
        .route("/auth/verify-email", post(auth_handler::verify_email))
        .route(
            "/auth/resend-verification",
            post(auth_handler::resend_verification),
        )
        // 公开的服务信息
        .route("/services/public", get(list_public_services))
        .route("/services/{id}/public", get(get_public_service))
        // API文档
        .route("/docs", get(serve_api_docs))
        .route("/openapi.json", get(serve_openapi_spec))
}

/// 配置认证路由
fn configure_auth_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/auth/logout", post(auth_handler::logout))
        .route("/auth/me", get(auth_handler::get_current_user))
        .route("/auth/me", put(auth_handler::update_current_user))
        .route("/auth/change-password", post(auth_handler::change_password))
        .route("/auth/sessions", get(auth_handler::get_user_sessions))
        .route(
            "/auth/sessions/{session_id}",
            delete(auth_handler::revoke_user_session),
        )
    // .layer(middleware::from_fn(jwt_auth_middleware))
}

/// 占位处理函数
async fn list_public_services() -> &'static str {
    "Public services"
}
async fn get_public_service() -> &'static str {
    "Public service"
}
async fn serve_api_docs() -> &'static str {
    "API docs"
}
async fn serve_openapi_spec() -> &'static str {
    "OpenAPI spec"
}
async fn update_service_status() -> &'static str {
    "Update status"
}
async fn publish_api() -> &'static str {
    "Publish API"
}
async fn deprecate_api() -> &'static str {
    "Deprecate API"
}
async fn get_user_roles() -> &'static str {
    "User roles"
}
async fn update_user_roles() -> &'static str {
    "Update user roles"
}
async fn get_user_permissions() -> &'static str {
    "User permissions"
}
async fn create_backup() -> &'static str {
    "Create backup"
}
async fn restore_backup() -> &'static str {
    "Restore backup"
}
async fn get_system_logs() -> &'static str {
    "System logs"
}
