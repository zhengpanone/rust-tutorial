use crate::state::AppState;
use axum::Router;
use axum::http::StatusCode;
use std::sync::Arc;

mod user;

pub fn crate_router(state: Arc<AppState>) -> Router {
    Router::new()
        .nest("/admin", admin_routes(state.clone()))
        .fallback(not_found)
        .with_state(state)
}

fn admin_routes(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new().nest("/users", user::routers(state))
}

async fn not_found() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "NOT Found")
}
