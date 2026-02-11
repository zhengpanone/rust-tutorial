// src/app/middleware/http/auth/extractor.rs
// 认证用户提取器

use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use chrono::{DateTime, Utc};
use common::error::AppError;
use common::security::jwt::claim::{DeviceInfo, JwtClaim, UserStatus};
use serde::{Deserialize, Serialize};
use tracing::{debug, error};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

/// 认证用户信息
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct AuthUser {
    /// 用户ID
    pub user_id: Uuid,
    /// 用户名
    pub username: String,

    /// 邮箱
    pub email: String,

    /// 显示名称
    pub display_name: String,

    /// 头像URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,

    /// 手机号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    /// 角色列表
    pub roles: Vec<String>,

    /// 权限列表
    pub permissions: Vec<String>,

    /// 令牌ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_id: Option<Uuid>,

    /// 会话ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<Uuid>,

    /// 令牌类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_type: Option<String>,

    /// 是否已验证邮箱
    #[serde(default)]
    pub email_verified: bool,

    /// 是否已验证手机
    #[serde(default)]
    pub phone_verified: bool,

    /// 账户状态
    pub status: UserStatus,

    /// 令牌颁发时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issued_at: Option<DateTime<Utc>>,

    /// 令牌过期时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<DateTime<Utc>>,

    /// 是否首次登录
    #[serde(default)]
    pub is_first_login: bool,

    /// 登录时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_time: Option<DateTime<Utc>>,

    /// 最后活动时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_activity: Option<DateTime<Utc>>,

    /// 设备信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_info: Option<DeviceInfo>,

    /// 元数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

impl AuthUser {
    pub fn from_claims(claims: &JwtClaim) -> Self {
        let device_info = claims.session.device_info.as_ref().map(|info| DeviceInfo {
            device_id: info.device_id.clone(),
            device_type: info.device_type.clone(),
            user_agent: info.user_agent.clone(),
            ip_address: info.ip_address.clone(),
            location: info.location.clone(),
        });

        Self {
            user_id: Uuid::parse_str(&claims.sub).unwrap_or_else(|e| {
                error!("Failed to parse user_id from claims.sub: {}", e);
                Uuid::new_v4()
            }),
            username: claims.user.username.clone(),
            email: claims.user.email.clone(),
            display_name: claims.user.display_name.clone(),
            avatar_url: None,
            phone: None,
            roles: claims.user.roles.clone(),
            permissions: claims.user.permissions.clone(),
            token_id: Some(Uuid::parse_str(&claims.jti).unwrap_or_else(|e| {
                error!("Failed to parse user_id from claims.sub: {}", e);
                Uuid::new_v4()
            })),
            session_id: Some(claims.session.id),
            token_type: Some(claims.typ.to_string()),
            email_verified: claims.user.email_verified,
            phone_verified: claims.user.phone_verified,
            status: claims.user.status,
            issued_at: Some(DateTime::from_timestamp(claims.iat, 0).unwrap_or(Utc::now())),
            expires_at: Some(DateTime::from_timestamp(claims.exp, 0).unwrap_or(Utc::now())),
            is_first_login: claims.session.is_first_login,
            login_time: Some(claims.session.login_time),
            last_activity: Some(claims.session.last_activity),
            device_info,
            metadata: serde_json::to_value(claims.session.metadata.clone())
                .ok()
                .filter(|v| !v.is_null()),
        }
    }
}

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        debug!("🔍 Extracting AuthUser from request");

        // 从扩展中获取JWT声明
        let claims = parts.extensions.get::<JwtClaim>().ok_or_else(|| {
            error!("JWT claims not found in request extensions");
            AppError::Authentication("需要认证".to_string())
        })?;

        // 从扩展中获取用户信息
        if let Some(auth_user) = parts.extensions.get::<AuthUser>() {
            debug!(
                "AuthUser found in request extensions: {}",
                auth_user.user_id
            );
            return Ok(auth_user.clone());
        }
        // 从JWT声明中提取用户信息
        let auth_user = AuthUser::from_claims(claims);
        debug!("AuthUser found in JWT claims: {}", auth_user.user_id);
        Ok(auth_user)
    }
}


