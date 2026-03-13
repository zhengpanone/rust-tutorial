use crate::handler::user_handler::get_user;
use crate::state::AppState;
use axum::Router;
use axum::routing::get;
use std::sync::Arc;

pub fn user_public_routes() -> Router<Arc<AppState>> {
    Router::new()
        // 公共路由
        .route("/profile/{user_id}", get(get_user))
}

pub fn user_protected_routes() -> Router<Arc<AppState>> {
    Router::new()
        // 需要认证路由
        .route("/me", get(get_user))
}

pub fn user_admin_routes() -> Router<Arc<AppState>> {
    Router::new()
        // 管理路由
        .route("/", get(get_user))
}
