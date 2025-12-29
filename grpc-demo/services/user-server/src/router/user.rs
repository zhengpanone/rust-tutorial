use crate::handler::user_handler::{get_user, register_user};
use crate::state::AppState;
use axum::routing::{get, post};
use axum::Router;
use std::sync::Arc;

pub fn routers(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/{user_id}", get(get_user))
        .route("/register", post(register_user))
        .with_state(state.clone())
}
