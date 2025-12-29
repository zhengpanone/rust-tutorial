use crate::http::user_handler::get_user;
use crate::state::AppState;
use axum::routing::get;
use axum::Router;
use std::sync::Arc;

pub fn routers(state: Arc<AppState>) ->  Router<Arc<AppState>> {
    Router::new()
        .route("/users/{user_id}", get(get_user))
        .route("/api/users/health", get(health_check))
        .with_state(state.clone())
}

async fn health_check() -> &'static str {
    "OK"
}
