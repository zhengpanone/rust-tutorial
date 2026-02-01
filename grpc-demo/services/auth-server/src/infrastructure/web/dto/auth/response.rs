use chrono::{DateTime, Utc};
use common::security::jwt::claim::{DeviceInfo, UserStatus};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// 注册响应
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RegisterResponse {
    /// 用户信息
    pub user: AuthUserResponse,

    /// 是否需要邮箱验证
    pub requires_email_verification: bool,

    /// 是否需要手机验证
    #[serde(default)]
    pub requires_phone_verification: bool,

    /// 消息
    pub message: String,

    /// 欢迎消息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub welcome_message: Option<String>,

    /// 创建时间
    pub created_at: DateTime<Utc>,
}

/// 认证用户响应
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AuthUserResponse {
    /// 用户ID
    pub id: String,

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

    /// 是否已验证邮箱
    #[serde(default)]
    pub email_verified: bool,

    /// 是否已验证手机
    #[serde(default)]
    pub phone_verified: bool,

    /// 账户状态
    pub status: UserStatus,

    /// 最后登录时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_login_at: Option<DateTime<Utc>>,

    /// 创建时间
    pub created_at: DateTime<Utc>,

    /// 更新时间
    pub updated_at: DateTime<Utc>,

    /// 元数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

/// 会话信息响应
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SessionInfoResponse {
    /// 会话ID
    pub session_id: String,

    /// 设备信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_info: Option<DeviceInfo>,

    /// 登录时间
    pub login_time: DateTime<Utc>,

    /// 最后活动时间
    pub last_activity: DateTime<Utc>,

    /// 是否为当前会话
    #[serde(default)]
    pub is_current: bool,

    /// 是否有效
    #[serde(default = "default_true")]
    pub is_active: bool,

    /// 会话元数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

fn default_true() -> bool {
    true
}
