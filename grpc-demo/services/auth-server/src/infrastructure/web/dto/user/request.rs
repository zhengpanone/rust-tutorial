use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use common::security::jwt::claim::UserStatus;

/// 用户响应
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UserRequest {
    /// 用户ID
    pub id: Uuid,

    /// 用户名
    pub username: String,

    /// 邮箱
    pub email: String,

    /// 手机号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    /// 显示名称
    pub display_name: String,

    /// 头像URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,

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

    /// 登录次数
    pub login_count: u32,

    /// 失败登录次数
    pub failed_login_count: u32,

    /// 最后失败登录时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_failed_login_at: Option<DateTime<Utc>>,

    /// 创建时间
    pub created_at: DateTime<Utc>,

    /// 更新时间
    pub updated_at: DateTime<Utc>,

    /// 元数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}
