// src/domain/identity/models/user.rs

use chrono::{DateTime, Utc};
use common::enums::user::UserStatus;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::types::Json;
use uuid::Uuid;
use validator::Validate;

/// 用户实体
/// 派生特性说明
/// Debug：用于调试打印
/// Clone：允许创建副本
/// Serialize/Deserialize：JSON 序列化支持
/// FromRow：自动将数据库行转换为 Rust 结构
#[derive(Debug, Clone, Serialize, Deserialize, Validate, FromRow)]
pub struct User {
    // 用户ID
    pub id: Uuid,

    /// 用户名
    #[validate(length(min = 3, max = 50, message = "用户名长度必须在3-50个字符之间"))]
    pub username: String,

    /// 邮箱
    #[validate(
        length(max = 100, message = "邮箱长度不能超过100个字符"),
        email(message = "邮箱格式不正确")
    )]
    pub email: String,

    /// 手机号
    #[validate(length(max = 20, message = "手机号长度不能超过20个字符"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    /// 密码哈希
    #[serde(skip)]
    pub password_hash: String,

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
    #[serde(default)]
    pub roles: Json<Vec<String>>,

    /// 权限列表
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub permissions: Json<Vec<String>>,

    /// 是否已验证邮箱
    #[serde(default)]
    pub email_verified: bool,

    /// 是否已验证手机
    #[serde(default)]
    pub phone_verified: bool,

    /// 用户状态
    #[serde(default)]
    pub status: UserStatus,

    /// 最后登录时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_login_at: Option<DateTime<Utc>>,

    /// 登录次数
    #[serde(default)]
    pub login_count: i64, // 使用 i64 接收 bigint

    /// 失败登录次数
    #[serde(default)]
    pub failed_login_count: i64, // 使用 i64 接收 bigint

    /// 最后失败登录时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_failed_login_at: Option<DateTime<Utc>>,

    /// 账户锁定时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locked_at: Option<DateTime<Utc>>,

    /// 账户锁定到期时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locked_until: Option<DateTime<Utc>>,

    /// 账户锁定原因
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_reason: Option<String>,

    /// 密码最后修改时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_changed_at: Option<DateTime<Utc>>,

    /// 密码过期时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_expires_at: Option<DateTime<Utc>>,

    /// 是否首次登录
    #[serde(default)]
    pub is_first_login: bool,

    /// 上次活动时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_activity_at: Option<DateTime<Utc>>,

    /// 时区
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,

    /// 语言
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,

    /// 元数据
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,

    /// 创建时间
    pub created_at: DateTime<Utc>,

    /// 更新时间
    pub updated_at: DateTime<Utc>,

    /// 软删除时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<DateTime<Utc>>,
}

impl User {
    pub fn activate(&mut self) {
        self.status = UserStatus::Activate;
        self.updated_at = Utc::now();
    }

    pub fn deactivate(&mut self) {
        self.status = UserStatus::Deactivate;
        self.updated_at = Utc::now();
    }

    pub fn lock(&mut self, reason: &str, minutes: i64) {
        self.status = UserStatus::Locked;
        self.lock_reason = Some(reason.to_string());
        self.locked_until = Some(Utc::now() + chrono::Duration::minutes(minutes));
        self.locked_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }

    pub fn unlock(&mut self) {
        if self.status == UserStatus::Locked {
            self.status = UserStatus::Activate;
            self.locked_at = None;
            self.locked_until = None;
            self.lock_reason = None;
            self.failed_login_count = 0;
            self.updated_at = Utc::now();
        }
    }

    pub fn record_login_success(&mut self) {
        self.last_login_at = Some(Utc::now());
        self.login_count += 1;
        self.failed_login_count = 0;
        self.last_failed_login_at = None;
        self.last_activity_at = Some(Utc::now());
        if self.status == UserStatus::Locked {
            self.unlock();
        }
        self.updated_at = Utc::now();
    }

    pub fn record_login_failure(&mut self) {
        self.failed_login_count += 1;
        if self.failed_login_count >= 5 {
            self.lock("failed login count exceeded", 10);
        }
        self.last_failed_login_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }

    pub fn is_locked(&self) -> bool {
        if self.status == UserStatus::Locked {
            if let Some(locked_until) = self.locked_until {
                return Utc::now() < locked_until;
            }
        }
        false
    }
}
