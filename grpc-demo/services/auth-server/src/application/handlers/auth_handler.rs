use crate::app::state::AppState;
use crate::infrastructure::web::dto::auth::request::{LoginRequest, RegisterRequest};
use crate::infrastructure::web::dto::auth::response::{LoginResponse, RegisterResponse};
use axum::Json;
use axum::extract::{Path, State};
use common::error::{AppError, AppResult};
use common::web::response::ApiResponse;
use std::sync::Arc;
use tracing::{error, info, instrument};
use utoipa::OpenApi;
use validator::Validate;

const TAG_NAME: &str = "Auth API";


// pub async fn register_user(
//     State(state): State<Arc<AppState>>,
//     Json(register_request): Json<RegisterDTO>,
// ) -> Result<Json<UserVO>, StatusCode> {
//     let user_service = UserService::new(state.clone());
//     let resp = user_service
//         .create_user(&register_request)
//         .await
//         .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
//     let vo = UserVO::try_from(resp).map_err(|_| StatusCode::NOT_FOUND)?;
//     Ok(Json(vo))
// }

/// 用户注册
#[utoipa::path(
    post,
    path = "/register",
    tag = TAG_NAME,
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "注册成功", body = RegisterResponse),
        (status = 400, description = "请求参数错误"),
        (status = 409, description = "用户已存在")
    )
)]
pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RegisterRequest>,
) -> AppResult<ApiResponse<RegisterResponse>> {
    info!("用户注册尝试: {}", payload.username);

    // 验证请求
    payload
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    // 检查用户是否存在
    if state
        .auth_service
        .user_exists(&payload.username, &payload.email)
        .await?
    {
        return Err(AppError::AlreadyExists("用户已存在".to_string()));
    }
    // 创建用户
    let user = state.auth_service.create_user(payload).await?;

    info!("用户注册成功: {} ({})", user.username, user.email);

    // 生成验证令牌
    let verification_token = state
        .auth_service
        .generate_verification_token(&user.id.to_string())
        .await?;

    // 先提取需要的信息，避免移动后无法访问
    let user_email = user.email.clone();
    let email_verified = user.email_verified;
    let phone_verified = user.phone_verified;
    let phone_provided = user.phone.is_some();
    let display_name = user.display_name.clone();
    let created_at = user.created_at;

    // 发送验证邮件
    tokio::spawn(async move {
        if let Err(e) = send_verification_email(&user_email, &verification_token).await {
            error!("发送验证邮件失败: {}", e);
        }
    });

    let response = RegisterResponse {
        user: user.into(),
        requires_email_verification: !email_verified,
        requires_phone_verification: phone_provided && !phone_verified,
        message: "注册成功, 请检查邮箱完成验证".to_string(),
        welcome_message: Some(format!("欢迎, {}! 您的账户已成功创建。", display_name)),
        created_at,
    };
    Ok(ApiResponse::success(response))
}

async fn send_verification_email(email: &str, _token: &str) -> Result<(), AppError> {
    // 这里集成邮件服务
    // 例如使用lettre、sendgrid等
    info!("发送验证邮件到: {}", email);
    Ok(())
}

/// 用户登陆
#[utoipa::path(
    post,
    path = "/login",
    tag=TAG_NAME,
    request_body = LoginRequest,
    responses(
        (status = 200, description = "登录成功", body = LoginResponse),
        (status = 400, description = "请求参数错误"),
        (status = 401, description = "用户名或密码错误"),
        (status = 403, description = "账户被锁定或禁用"),
        (status = 429, description = "请求频率过高"),
    )
)]
#[instrument(name = "http_login", skip(state), fields(identifier = %payload.identifier, client_id = ?payload.device_id))]
pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginRequest>,
) -> AppResult<ApiResponse<LoginResponse>> {
    info!("用户登陆尝试: {}", payload.identifier);
    // 验证请求
    payload
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    // 检查登陆尝试次数
    todo!()
}

pub async fn refresh_token() {
    todo!()
}

pub async fn forgot_password() {
    todo!()
}

pub async fn reset_password() {
    todo!()
}

pub async fn verify_email() {
    todo!()
}

pub async fn resend_verification() {}

/// 退出登录
#[utoipa::path(
    post,
    path = "/logout",
    tag = TAG_NAME,
    responses(
        (status = 204, description = "退出成功"),
        (status = 401, description = "未认证"),
    ),
    security(("bearer_auth" =[]))
)]
#[instrument(name = "http_logout", skip(_state))]
pub async fn logout(State(_state): State<Arc<AppState>>) -> ApiResponse<()> {
    info!("退出成功");
    ApiResponse::success_empty("退出成功")
}

#[utoipa::path(
    post,
    path = "/admin/force-logout/{user_id}",
    tag = TAG_NAME,
    params(
        ("user_id"=String,Path,description="用户ID")
    ),
    responses(
        (status = 204, description = "强制退出成功"),
        (status = 401, description = "未认证"),
        (status = 403, description = "权限不足"),
    ),
    security(("bearer_auth" =[]))
)]
#[instrument(name="http_force_logout", skip(state),
    fields(
// admin_user_id = %auth_user.user_id,
target_user_id= %user_id))]
pub async fn force_logout(
    State(state): State<Arc<AppState>>,
    // auth_user: AuthUser,
    Path(user_id): Path<String>,
) -> ApiResponse<()> {
    info!("强制退出用户: {}", user_id);
    ApiResponse::success_empty("强制退出成功")
}

pub async fn change_password() {
    todo!()
}

pub async fn get_user_sessions() {
    todo!()
}

pub async fn revoke_user_session() {
    todo!()
}

pub async fn update_current_user() {
    todo!()
}

#[derive(OpenApi)]
#[openapi(
    paths(register,logout,force_logout),
    tags((name = "Auth API", description = "Auth management"))
)]
pub struct AuthApiDoc;
