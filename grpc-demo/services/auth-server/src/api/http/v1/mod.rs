use crate::app::state::AppState;
use axum::Router;
use std::sync::Arc;

mod auth;
mod user;
mod service;

/// 为路由添加前缀的辅助函数
pub fn prefix_routes<B: Clone + Send + Sync + 'static>(
    prefix: &str,
    router: Router<B>,
) -> Router<B> {
    Router::new().nest(prefix, router)
}

pub fn v1_routes() -> Router<Arc<AppState>> {
    // 公开路由
    let public_routes = Router::new()
        .merge(prefix_routes("/auth", auth::auth_public_routes()))
        .merge(prefix_routes("/user", user::user_public_routes()));

    let protected_routes = Router::new()
        .nest("/auth", auth::auth_protected_routes())
        .nest("/user", user::user_protected_routes());

    let admin_routes = Router::new()
        .nest("/admin/auth", auth::auth_admin_routes())
        .nest("/admin/user", user::user_admin_routes());

    // 组合所有路由
    Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .merge(admin_routes)
}
