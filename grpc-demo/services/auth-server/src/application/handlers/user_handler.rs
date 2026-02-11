// handler/user_handler

use crate::app::middleware::http::auth::extractor::AuthUser;
use crate::app::state::AppState;
use crate::application::handlers::SecurityAddon;
use crate::infrastructure::web::dto::user::request::{CreateUserRequest, UpdateUserRequest};
use crate::infrastructure::web::dto::user::response::UserResponse;
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use common::error::AppResult;
use common::web::response::ApiResponse;
use std::sync::Arc;
use tracing::{debug, instrument};
use utoipa::OpenApi;

const TAG_NAME: &str = "User API";

// pub async fn get_user(
//     State(state): State<Arc<AppState>>,
//     Path(user_id): Path<String>,
// ) -> Result<Json<UserVO>, StatusCode> {
//     let user_service = UserService::new(state.clone());
//     let resp = user_service
//         .get_user_by_id(user_id.as_str())
//         .await
//         .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
//         .expect("User not found");
//     let vo = UserVO::try_from(resp).map_err(|_| StatusCode::NOT_FOUND)?;
//     Ok(Json(vo))
// }

/// 获取当前用户信息
#[utoipa::path(
    get,
    path = "/me",
    tag = TAG_NAME,
    responses(
        (status = 200, description = "当前用户信息", body = UserResponse),
        (status = 401, description = "未授权")
    ),
    security(
        ("jwt" = [])
    )
)]
#[instrument(name="http_get_current_user", skip_all, fields(user_id=%auth_user.user_id))]
pub async fn get_current_user(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
) -> AppResult<ApiResponse<UserResponse>> {
    debug!("get_current_user: {:?}", auth_user);
    Ok(ApiResponse::success(UserResponse::default()))
}

/// 更新当前用户信息
#[utoipa::path(
    put,
    path = "/me",
    tag = TAG_NAME,
    request_body = UpdateUserRequest,
    responses(
        (status = 200, description = "更新当前用户信息成功", body = UserResponse),
        (status = 400, description = "请求参数错误"),
        (status = 401, description = "未授权")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn update_current_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UpdateUserRequest>,
) -> AppResult<ApiResponse<UserResponse>> {
    todo!()
}

pub async fn get_user_profile_public() {
    todo!()
}

/// 创建用户
#[utoipa::path(
    post,
    path = "/user",
    tag = TAG_NAME,
    request_body = CreateUserRequest,
    responses(
        (status = 200, description = "创建成功", body = UserResponse),
        (status = 400, description = "请求参数错误"),
        (status = 401, description = "未授权"),
        (status = 403, description = "权限不足"),
        (status = 409, description = "用户已存在")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateUserRequest>,
) -> AppResult<ApiResponse<UserResponse>> {
    todo!("TODO: 创建用户")
}

/// 用户相关的 API 文档
#[derive(OpenApi)]
#[openapi(
    paths(get_current_user,update_current_user,create_user),
    components(schemas(UserResponse)),
    tags((name = "User API", description = "User management")),
    modifiers(&SecurityAddon)  // 如果需要全局安全配置
)]
pub struct UserApiDoc;
