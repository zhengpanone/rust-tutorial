// src/application/services/auth_service.proto

use crate::domain::identity::entities::user::User;
use crate::infrastructure::web::dto::auth::request::{LoginRequest, RegisterRequest};
use crate::infrastructure::web::dto::auth::response::SessionInfoResponse;
use async_trait::async_trait;
use common::error::AppError;
use common::security::jwt::claim::JwtSession;

/// 认证应用服务 trait
#[async_trait]
pub trait AuthService: Send + Sync {
    /// 用户认证
    async fn authenticate_user(&self, username: String, password: String)
    -> Result<User, AppError>;

    /// 检查用户是否存在
    async fn user_exists(&self, username: &str, email: &str) -> Result<bool, AppError>;

    /// 创建用户
    async fn create_user(&self, request: RegisterRequest) -> Result<User, AppError>;

    /// 获取用户
    async fn get_user(&self, user_id: &str) -> Result<User, AppError>;

    /// 通过邮箱获取用户
    async fn get_user_by_email(&self, email: &str) -> Result<User, AppError>;

    /// 生成验证token
    async fn generate_verification_token(&self, user_id: &str) -> Result<String, AppError>;

    /// 生成密码重置token
    async fn generate_password_reset_token(&self, email: &str) -> Result<String, AppError>;

    /// 验证密码重置令牌
    async fn validate_password_reset_token(&self, token: &str) -> Result<(), AppError>;

    /// 验证邮箱验证令牌
    async fn validate_email_verification_token(&self, token: &str) -> Result<String, AppError>;

    /// 重置密码
    async fn reset_password(&self, user_id: &str, new_password: &str) -> Result<(), AppError>;

    /// 验证邮箱
    async fn verify_email(&self, user_id: &str) -> Result<(), AppError>;

    /// 记录登录成功
    async fn record_login_success(
        &self,
        user_id: &str,
        session_id: &str,
        request: &LoginRequest,
    ) -> Result<(), AppError>;

    /// 记录登录失败
    async fn record_login_failure(&self, identifier: &str, reason: &str) -> Result<(), AppError>;

    /// 记录登出
    async fn record_logout(&self, user_id: &str, session_id: Option<&str>) -> Result<(), AppError>;

    /// 获取用户会话
    async fn get_user_sessions(&self, user_id: &str) -> Result<Vec<SessionInfoResponse>, AppError>;

    /// 吊销用户会话
    async fn revoke_user_session(&self, user_id: &str, session_id: &str) -> Result<(), AppError>;

    /// 吊销用户所有会话
    async fn revoke_all_user_sessions(&self, user_id: &str) -> Result<(), AppError>;

    /// 检查密码强度
    fn check_password_strength(&self, password: &str) -> Result<(), AppError>;

    /// 生成会话
    async fn create_session(
        &self,
        user: &User,
        request: &LoginRequest,
    ) -> Result<JwtSession, AppError>;

    /// 验证验证码
    async fn verify_captcha(&self, captcha_id: &str, captcha: &str) -> Result<bool, AppError>;

    /// 发送验证邮件
    async fn send_verification_email(&self, email: &str, token: &str) -> Result<(), AppError>;

    /// 发送密码重置邮件
    async fn send_password_reset_email(&self, email: &str, token: &str) -> Result<(), AppError>;
}
