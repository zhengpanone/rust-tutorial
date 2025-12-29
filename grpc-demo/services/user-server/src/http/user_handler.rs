// http/user_handler

use crate::schemas::user::UserVO;
use crate::service::user_service::UserService;
use crate::AppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use std::sync::Arc;

pub async fn get_user(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<String>,
) -> Result<Json<UserVO>, axum::http::StatusCode> {
    let user_service = UserService::new(state.clone());
    let resp = user_service
        .get_user_by_id(user_id.as_str())
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?
        .expect("User not found");
    let vo = UserVO::try_from(resp).map_err(|_| StatusCode::NOT_FOUND)?;
    Ok(Json(vo))
}
