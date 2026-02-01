use crate::app::state::AppState;
use crate::infrastructure::web::dto::auth::request::{LoginRequest, RegisterRequest};
use crate::infrastructure::web::dto::auth::response::RegisterResponse;
use axum::Json;
use axum::extract::State;
use common::error::{AppError, AppResult};
use common::web::response::ApiResponse;
use std::sync::Arc;
use tracing::{error, info};
use validator::Validate;

/// 用户注册
#[utoipa::path(
    post,
    path = "/api/v1/auth/register",
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
        .auth_app
        .user_exists(&payload.username, &payload.email)
        .await?
    {
        return Err(AppError::AlreadyExists("用户已存在".to_string()));
    }
    // 创建用户
    let user = state.auth_app.create_user(payload).await?;
    info!("用户注册成功: {} ({})", user.username, user.email);

    // 生成验证令牌
    let verification_token = state.auth_app.generate_verification_token(&user.id).await?;
    
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

async fn send_verification_email(email: &str, token: &str) -> Result<(), AppError> {
    // 这里集成邮件服务
    // 例如使用lettre、sendgrid等
    info!("发送验证邮件到: {}", email);
    Ok(())
}

pub async fn login(State(state): State<Arc<AppState>>, Json(payload): Json<LoginRequest>) {
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

pub async fn logout() {
    todo!()
}

pub async fn get_current_user() {
    todo!()
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
