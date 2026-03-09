// handler/user_handler

use crate::app::middleware::http::auth::extractor::AuthUser;
use crate::app::state::AppState;
use crate::application::handlers::SecurityAddon;
use crate::domain::identity::entity::user::UserId;
use crate::domain::shared::auth::permissions::require_role;
use crate::domain::shared::id::DomainId;
use crate::infrastructure::web::dto::user::request::{CreateUserRequest, UpdateUserRequest};
use crate::infrastructure::web::dto::user::response::UserResponse;
use axum::Json;
use axum::extract::{Path, State};
use common::error::{AppError, AppResult};
use common::web::response::ApiResponse;
use std::sync::Arc;
use tracing::{debug, info, instrument};
use utoipa::OpenApi;
use validator::Validate;

const TAG_NAME: &str = "User API";
const ADMIN_TAG_NAME: &str = "Admin User API";

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
    State(_state): State<Arc<AppState>>,
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
    State(_state): State<Arc<AppState>>,
    Json(_payload): Json<UpdateUserRequest>,
) -> AppResult<ApiResponse<UserResponse>> {
    todo!()
}

pub async fn get_user_profile_public() {
    todo!()
}

/// 创建用户
#[utoipa::path(
    post,
    path = "",
    tag = ADMIN_TAG_NAME,
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
// #[instrument(name="http_create_user", skip_all, fields(user_id=%auth_user.user_id))]
pub async fn create_user(
    State(state): State<Arc<AppState>>,
    // auth_user: AuthUser,
    Json(payload): Json<CreateUserRequest>,
) -> AppResult<ApiResponse<UserResponse>> {
    // 检查权限
    // require_role(&auth_user, "admin")?;
    debug!("Create user: {:?}", payload);

    // 验证请求参数
    payload
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    let user = state.user_service.create_user(payload).await?;
    let resp = UserResponse::from(user);

    // info!("User created successfully by admin: {}", auth_user.user_id);
    Ok(ApiResponse::created(resp))
}

/// 删除用户（管理员）
#[utoipa::path(
    delete,
    path = "/{id}",
tag = ADMIN_TAG_NAME,
    params(
        ("id" = String, Path, description = "用户ID")
    ),
    responses(
        (status = 204, description = "删除成功"),
        (status = 401, description = "未授权"),
        (status = 403, description = "权限不足"),
        (status = 404, description = "用户不存在")
    ),
    security(
        ("jwt" = [])
    )
)]
#[instrument(name="http_delete_user", skip_all, fields(user_id=%auth_user.user_id))]
pub async fn delete_user(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Path(id): Path<String>,
) -> AppResult<ApiResponse<()>> {
    let user_id = UserId::try_from(id.clone()).map_err(|_| AppError::InvalidId(id.clone()))?;
    let auth_id = UserId::try_from(auth_user.user_id)
        .map_err(|_| AppError::InvalidId(auth_user.user_id.to_string()))?;
    require_role(&auth_user, "admin")?;
    if user_id == auth_id {
        return Err(AppError::Authentication("不能删除自己的账户".to_string()));
    }
    state.user_service.delete_user(user_id.as_str()).await?;
    info!("User deleted successfully by admin: {}", auth_user.user_id);
    Ok(ApiResponse::success_empty("User deleted successfully"))
}

/// 用户相关的 API 文档
#[derive(OpenApi)]
#[openapi(
    paths(get_current_user,update_current_user),
    components(schemas(UserResponse)),
    tags((name = "User API", description = "User management")),
    modifiers(&SecurityAddon)  // 如果需要全局安全配置
)]
pub struct UserApiDoc;

/// 管理员用户相关的 API 文档
#[derive(OpenApi)]
#[openapi(
    paths(create_user, delete_user),
    components(schemas(UserResponse)),
    tags((name = "Admin User API", description = "Admin User management")),
    modifiers(&SecurityAddon)  // 如果需要全局安全配置
)]
pub struct AdminUserApiDoc;
