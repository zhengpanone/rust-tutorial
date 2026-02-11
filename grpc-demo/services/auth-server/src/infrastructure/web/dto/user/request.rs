use chrono::{DateTime, Utc};
use common::security::jwt::claim::UserStatus;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

/// 创建用户请求
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateUserRequest {
    /// 用户名
    #[validate(
        length(min = 3, max = 50, message = "用户名长度必须在3-50个字符之间"),
        custom(function = "validate_username")
    )]
    pub username: String,

    /// 邮箱
    #[validate(
        length(max = 128, message = "邮箱长度不能超过128个字符"),
        email(message = "邮箱格式不正确")
    )]
    pub email: String,


    /// 手机号
    #[validate(
        length(max = 20, message = "手机号长度不能超过20个字符"),
        custom(function = "validate_phone")
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    /// 密码
    #[validate(
        length(min = 6, max = 128, message = "密码长度必须在6-128个字符之间"),
        custom(function = "validate_password_strength")
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,

    /// 显示名称
    #[validate(length(max = 100, message = "显示名称长度不能超过100个字符"))]
    pub display_name: String,

    /// 头像URL
    #[validate(
        length(max = 500, message = "头像URL长度不能超过500个字符"),
        url(message = "头像URL格式不正确")
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,

    /// 角色列表
    #[validate(length(max = 20, message = "角色长度不能超过20个"))]
    #[serde(default)]
    pub roles: Vec<String>,

    /// 是否已验证邮箱
    #[serde(default)]
    pub email_verified: bool,

    /// 是否已验证手机
    #[serde(default)]
    pub phone_verified: bool,

    /// 账户状态
    #[serde(default)]
    pub status: UserStatus,

    /// 元数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct UserFilter {
    pub status: Option<UserStatus>,
    pub role: Option<String>,
    pub email_verified: Option<bool>,
    pub phone_verified: Option<bool>,
    pub created_after: Option<DateTime<Utc>>,
    pub created_before: Option<DateTime<Utc>>,
    pub search: Option<String>,
}

/// 更新用户请求
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct UpdateUserRequest{

    /// 用户名
    #[validate(
        length(min = 3, max = 50, message = "用户名长度必须在3-50个字符之间"),
        custom(function = "validate_username")
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,

    /// 邮箱
    #[validate(
        length(max = 128, message = "邮箱长度不能超过128个字符"),
        email(message = "邮箱格式不正确")
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// 手机号
    #[validate(
        length(max = 20, message = "手机号长度不能超过20个字符"),
        custom(function = "validate_phone")
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    /// 显示名称
    #[validate(length(max = 100, message = "显示名称长度不能超过100个字符"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name:  Option<String>,

    /// 头像URL
    #[validate(
        length(max = 500, message = "头像URL长度不能超过500个字符"),
        url(message = "头像URL格式不正确")
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,

    /// 角色列表
    #[validate(length(max = 20, message = "角色长度不能超过20个"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<String>>,

    /// 是否已验证邮箱
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_verified: Option<bool>,

    /// 是否已验证手机
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_verified: Option<bool>,

    /// 账户状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<UserStatus>,

    /// 元数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,

}

/// 用户请求
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

use super::super::auth::request::{validate_password_strength, validate_phone, validate_username};
