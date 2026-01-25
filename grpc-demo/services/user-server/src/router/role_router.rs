use std::sync::Arc;

use axum::{
    Router,
    routing::{delete, get, post, put},
};

use crate::{
    handler::role_handler::{create_role, get_role_detail},
    state::AppState,
};

pub fn role_router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/detail/{role_id}", get(get_role_detail))
        .route("/create", post(create_role))
        .route("/update", put(create_role))
        .route("/delete", delete(create_role))
        .with_state(state.clone())
}
