use crate::app::config::features::RuleCondition::UserId;
use crate::app::state::AppState;
use crate::domain::shared::auth::user::AuthUser;
use crate::infrastructure::web::dto::auth::request::{ForgotPasswordRequest, LoginRequest, RefreshTokenRequest, RegisterRequest, ResetPasswordRequest};
use crate::infrastructure::web::dto::auth::response::{
    AuthUserResponse, LoginResponse, RegisterResponse,
};
use axum::Json;
use axum::extract::{Path, State};
use common::error::{AppError, AppResult};
use common::security::jwt::claim::{JwtSession, JwtUser};
use common::web::response::ApiResponse;
use sqlx::encode::IsNull::No;
use std::sync::Arc;
use tracing::{debug, error, info, instrument};
use utoipa::OpenApi;
use validator::Validate;

const TAG_NAME: &str = "Auth API";

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
    info!("用户登陆尝试: {}", &payload.identifier);
    // 验证请求
    payload
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    // 检查登陆尝试次数

    let user = state
        .auth_service
        .authenticate_user(payload.identifier, payload.password)
        .await?;

    // 创建会话
    let session = JwtSession {
        id: Default::default(),
        device_info: None,
        is_first_login: false,
        login_time: Default::default(),
        last_activity: Default::default(),
        metadata: Default::default(),
    };

    let jwt_user = JwtUser {
        id: "".to_string(),
        username: "".to_string(),
        email: "".to_string(),
        display_name: "".to_string(),
        roles: Default::default(),
        permissions: Default::default(),
        status: Default::default(),
        email_verified: false,
        phone_verified: false,
        metadata: Default::default(),
    };
    // 生成访问令牌
    let (access_token, access_claims) = state
        .jwt_service
        .generate_access_token(jwt_user, session.clone())
        .await?;

    let (refresh_token, refresh_claims) = state
        .jwt_service
        .generate_refresh_token(user.id.to_string(), session)
        .await?;
    let response = LoginResponse {
        access_token,
        refresh_token,
        token_type: "".to_string(),
        expires_in: 0,
        user: AuthUserResponse {
            id: Default::default(),
            username: "".to_string(),
            email: "".to_string(),
            display_name: "".to_string(),
            avatar_url: None,
            phone: None,
            roles: vec![],
            permissions: vec![],
            email_verified: false,
            phone_verified: false,
            status: Default::default(),
            last_login_at: None,
            created_at: Default::default(),
            updated_at: Default::default(),
            metadata: None,
        },
        requires_mfa: false,
        mfa_type: None,
        session_id: Default::default(),
        issued_at: Default::default(),
        expires_at: Default::default(),
        is_first_login: false,
    };
    Ok(ApiResponse::success(response))
}

/// 刷新令牌
#[utoipa::path(
    post,
    path = "/refresh",
    request_body = RefreshTokenRequest,
    responses(
        (status = 200, description = "刷新成功", body = LoginResponse),
        (status = 400, description = "请求参数错误"),
        (status = 401, description = "无效的刷新令牌")
    )
)]
#[instrument(name = "http_refresh_token", skip_all)]
pub async fn refresh_token(
    State(state): State<AppState>,
    Json(payload): Json<RefreshTokenRequest>,
) -> AppResult<ApiResponse<LoginResponse>> {
    payload
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    debug!("Token refresh attempt");

    let new_session = JwtSession {
        id: Default::default(),
        device_info: None,
        is_first_login: false,
        login_time: Default::default(),
        last_activity: Default::default(),
        metadata: Default::default(),
    };

    let (new_access_token, new_claims) = state
        .auth_service
        .refresh_token(&payload.refresh_token, new_session)
        .await?;

    let user = state.auth_service.get_user(&new_claims.sub).await?;

    let response = LoginResponse {
        access_token: "".to_string(),
        refresh_token: "".to_string(),
        token_type: "".to_string(),
        expires_in: 0,
        user: user.into(),
        requires_mfa: false,
        mfa_type: None,
        session_id: Default::default(),
        issued_at: Default::default(),
        expires_at: Default::default(),
        is_first_login: false,
    };

    Ok(ApiResponse::success(response))
}

/// 忘记密码
#[utoipa::path(
    post,
    path = "/api/v1/auth/forgot-password",
    request_body = ForgotPasswordRequest,
    responses(
        (status = 200, description = "密码重置邮件已发送"),
        (status = 400, description = "请求参数错误"),
    )
)]
#[instrument(name = "http_forgot_password", skip_all, fields(identifier = %request.identifier))]
pub async fn forgot_password(
    State(state): State<Arc<AppState>>,
    Json(request): Json<ForgotPasswordRequest>,
) -> AppResult<ApiResponse<LoginResponse>> {
    request.validate().map_err(|e| AppError::Validation(e.to_string()))?;

    debug!("Forgot password for identifier: {}", request.identifier);

    // 构建请求
    let forgot_password_request = ForgotPasswordRequest {
        email: "".to_string(),
        identifier: request.identifier,
        reset_url: request.reset_url,
        expires_in_minutes: request.expires_in_minutes,
        captcha: None,
        client_id: None,
        captcha_id: None,
    };

    // 调用认证服务
    // state.auth_service
    //     .forgot_password(forgot_password_request)
    //     .await?;

    // Ok(ApiResponse::success_with_message("密码重置链接已发送到您的邮箱"))
    todo!()
}

/// 重置密码
#[utoipa::path(
    post,
    path = "/api/v1/auth/reset-password",
    request_body = ResetPasswordRequest,
    responses(
        (status = 204, description = "密码重置成功"),
        (status = 400, description = "请求参数错误或令牌无效"),
    )
)]
#[instrument(name = "http_reset_password", skip_all)]
pub async fn reset_password(State(state): State<Arc<AppState>>,
                            Json(request): Json<ResetPasswordRequest>,
) -> AppResult<ApiResponse> {
    request.validate().map_err(|e| AppError::Validation(e.to_string()))?;

    debug!("Reset password attempt");

    // 检查密码匹配
    if request.new_password != request.confirm_password {
        return Err(AppError::Validation("新密码不匹配".to_string()));
    }

    // 构建请求
    let reset_password_request = ResetPasswordRequest {
        token: request.token,
        new_password: request.new_password,
        confirm_password: request.confirm_password,
        // client_id: None,
        // verification_code: request.verification_code,
    };

    // 调用认证服务
    // state.auth_service
    //     .reset_password(reset_password_request,"")
    //     .await?;

    info!("Password reset successfully");

    Ok(ApiResponse::success_empty("密码重置成功"))
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
#[instrument(name = "http_logout", skip_all, fields(user_id = %auth_user.user_id))]
pub async fn logout(State(state): State<Arc<AppState>>, auth_user: AuthUser) -> ApiResponse<()> {
    info!("退出成功");

    state
        .auth_service
        .logout(auth_user.token_id)
        .await
        .expect("TODO: panic message");

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
    paths(register,login, logout,force_logout),
    tags((name = "Auth API", description = "Auth management"))
)]
pub struct AuthApiDoc;
