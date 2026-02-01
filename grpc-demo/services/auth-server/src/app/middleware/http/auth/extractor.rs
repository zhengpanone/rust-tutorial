// src/app/middleware/http/auth/extractor.rs
// 认证用户提取器

use chrono::{DateTime, Utc};
use common::security::jwt::claim::{DeviceInfo, UserStatus};
use serde::{Deserialize, Serialize};
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

// TODO
