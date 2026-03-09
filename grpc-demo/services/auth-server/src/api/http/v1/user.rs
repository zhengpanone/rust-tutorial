use crate::app::state::AppState;
use crate::application::handlers::auth_handler::update_current_user;
use crate::application::handlers::user_handler::{
    create_user, delete_user, get_current_user, get_user_profile_public,
};
use axum::Router;
use axum::routing::{get, post};
use std::sync::Arc;

pub fn user_public_routes() -> Router<Arc<AppState>> {
    Router::new()
        // 公共路由
        .route("/profile/{user_id}", get(get_user_profile_public))
}

pub fn user_protected_routes() -> Router<Arc<AppState>> {
    Router::new()
        // 需要认证路由
        .route("/me", get(get_current_user).put(update_current_user))
}

pub fn user_admin_routes() -> Router<Arc<AppState>> {
    Router::new()
        // 管理路由
        .route("/", post(create_user).delete(delete_user))
}
