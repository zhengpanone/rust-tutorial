// handler/user_handler

use crate::AppState;
use crate::schemas::user::{RegisterDTO, UserVO};
use crate::service::user_service::UserService;
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use std::sync::Arc;

pub async fn get_user(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<String>,
) -> Result<Json<UserVO>, StatusCode> {
    let user_service = UserService::new(state.clone());
    let resp = user_service
        .get_user_by_id(user_id.as_str())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .expect("User not found");
    let vo = UserVO::try_from(resp).map_err(|_| StatusCode::NOT_FOUND)?;
    Ok(Json(vo))
}

pub async fn register_user(
    State(state): State<Arc<AppState>>,
    Json(register_request): Json<RegisterDTO>,
) -> Result<Json<UserVO>, StatusCode> {
    let user_service = UserService::new(state.clone());
    let resp = user_service
        .create_user(&register_request)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let vo = UserVO::try_from(resp).map_err(|_| StatusCode::NOT_FOUND)?;
    Ok(Json(vo))
}
