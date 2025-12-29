use crate::handler::health_handler::health_check;
use crate::state::AppState;
use axum::Router;
use axum::http::StatusCode;
use axum::routing::get;
use std::sync::Arc;

mod user;

pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .nest("/users", user::routers(state.clone()))
        .route("/health", get(health_check))
        .fallback(not_found)
        .with_state(state.clone())
}

async fn not_found() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "NOT Found")
}
