use crate::app::state::AppState;
use crate::application::handlers::auth_handler::{force_logout, login, logout, register};
use axum::Router;
use axum::routing::post;
use std::sync::Arc;

pub fn auth_public_routes() -> Router<Arc<AppState>> {
    Router::new()
        // 公开路由
        .route("/login", post(login))
        .route("/register", post(register))
}
pub fn auth_protected_routes() -> Router<Arc<AppState>> {
    Router::new()
        // 需要认证的路由
        .route("/logout", post(logout))
}

pub fn auth_admin_routes() -> Router<Arc<AppState>> {
    Router::new()
        // 管理路由
        .route("/force-logout/{user_id}", post(force_logout))
}
