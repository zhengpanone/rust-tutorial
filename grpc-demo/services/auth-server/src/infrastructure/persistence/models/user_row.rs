// src/infrastructure/persistence/entity/user_row
// 数据库模型
use crate::domain::identity::entity::user::User;
use crate::domain::shared::id::DomainId;
use chrono::{DateTime, Utc};
use common::enums::user::UserStatus;
use common::error::{AppError, AppResult};
use sqlx::FromRow;
use sqlx::types::Json;
use uuid::Uuid;

/// 用户实体
/// 派生特性说明
/// Debug：用于调试打印
/// Clone：允许创建副本
/// Serialize/Deserialize：JSON 序列化支持
/// FromRow：自动将数据库行转换为 Rust 结构
#[derive(Debug, Clone, FromRow)]
pub struct UserRow {
    // 用户ID
    pub id: Uuid,

    /// 用户名
    pub username: String,

    /// 邮箱
    pub email: String,

    /// 手机号
    pub phone: Option<String>,

    /// 密码哈希
    pub password_hash: String,

    /// 显示名称
    pub display_name: String,

    /// 头像URL
    pub avatar_url: Option<String>,

    /// 角色列表
    pub roles: Json<Vec<String>>,

    /// 权限列表
    pub permissions: Json<Vec<String>>,

    /// 是否已验证邮箱
    pub email_verified: bool,

    /// 是否已验证手机
    pub phone_verified: bool,

    /// 用户状态
    pub status: UserStatus,

    /// 最后登录时间
    pub last_login_at: Option<DateTime<Utc>>,

    /// 登录次数
    pub login_count: i64, // 使用 i64 接收 bigint

    /// 失败登录次数
    pub failed_login_count: i64, // 使用 i64 接收 bigint

    /// 最后失败登录时间
    pub last_failed_login_at: Option<DateTime<Utc>>,

    /// 账户锁定时间
    pub locked_at: Option<DateTime<Utc>>,

    /// 账户锁定到期时间
    pub locked_until: Option<DateTime<Utc>>,

    /// 账户锁定原因
    pub lock_reason: Option<String>,

    /// 密码最后修改时间
    pub password_changed_at: Option<DateTime<Utc>>,

    /// 密码过期时间
    pub password_expires_at: Option<DateTime<Utc>>,

    /// 是否首次登录
    pub is_first_login: bool,

    /// 上次活动时间
    pub last_activity_at: Option<DateTime<Utc>>,

    /// 时区
    pub timezone: Option<String>,

    /// 语言
    pub language: Option<String>,

    /// 元数据
    pub metadata: Option<serde_json::Value>,

    /// 创建时间
    pub created_at: DateTime<Utc>,

    /// 更新时间
    pub updated_at: DateTime<Utc>,

    /// 软删除时间
    pub deleted_at: Option<DateTime<Utc>>,
}

impl UserRow {
    /// 从领域实体转换
    pub fn from_domain(user: &User) -> Self {
        let uuid = Uuid::parse_str(user.id.as_str()).expect("Failed to parse UserId to Uuid");

        Self {
            id: uuid,
            username: user.username.clone(),
            email: user.email.clone(),
            phone: user.phone.clone(),
            password_hash: user.password_hash.clone(),
            display_name: user.display_name.clone(),
            avatar_url: user.avatar_url.clone(),
            roles: Json(user.roles.iter().map(|r| r.to_string()).collect()),
            permissions: Json(user.permissions.iter().map(|p| p.to_string()).collect()),
            email_verified: user.email_verified,
            phone_verified: user.phone_verified,
            status: user.status,
            last_login_at: user.last_login_at,
            login_count: user.login_count,
            failed_login_count: user.failed_login_count,
            last_failed_login_at: user.last_failed_login_at,
            locked_at: user.locked_at,
            locked_until: user.locked_until,
            lock_reason: user.lock_reason.clone(),
            password_changed_at: user.password_changed_at,
            password_expires_at: user.password_expires_at,
            is_first_login: user.is_first_login,
            last_activity_at: user.last_activity_at,
            timezone: user.timezone.clone(),
            language: user.language.clone(),
            metadata: user.metadata.clone(),
            created_at: user.created_at,
            updated_at: user.updated_at,
            deleted_at: user.deleted_at,
        }
    }

    /// 转换到领域实体
    pub fn to_domain(&self) -> User {
        use crate::domain::identity::entity::{
            permission::PermissionId, role::RoleId, user::UserId,
        };

        // roles 转换
        let roles: Vec<RoleId> = self
            .roles
            .0
            .iter()
            .map(|r| RoleId::from(r.clone()))
            .collect();

        // permissions 转换，解析失败会返回 AppError
        let permissions: Vec<PermissionId> = self
            .permissions
            .0
            .iter()
            .map(|p| PermissionId::parse(p))
            .collect::<AppResult<Vec<_>>>()
            .expect("Failed to parse PermissionId");

        User {
            id: UserId::from(self.id.to_string()),
            username: self.username.clone(),
            email: self.email.clone(),
            phone: self.phone.clone(),
            password_hash: self.password_hash.clone(),
            display_name: self.display_name.clone(),
            avatar_url: self.avatar_url.clone(),
            roles,
            permissions,
            email_verified: self.email_verified,
            phone_verified: self.phone_verified,
            status: self.status,
            last_login_at: self.last_login_at,
            login_count: self.login_count,
            failed_login_count: self.failed_login_count,
            last_failed_login_at: self.last_failed_login_at,
            locked_at: self.locked_at,
            locked_until: self.locked_until,
            lock_reason: self.lock_reason.clone(),
            password_changed_at: self.password_changed_at,
            password_expires_at: self.password_expires_at,
            is_first_login: self.is_first_login,
            last_activity_at: self.last_activity_at,
            timezone: self.timezone.clone(),
            language: self.language.clone(),
            metadata: self.metadata.clone(),
            created_at: self.created_at,
            updated_at: self.updated_at,
            deleted_at: self.deleted_at,
        }
    }
}

// 为数据库操作实现转换 trait
impl From<User> for UserRow {
    fn from(user: User) -> Self {
        UserRow::from_domain(&user)
    }
}

impl From<&User> for UserRow {
    fn from(user: &User) -> Self {
        UserRow::from_domain(user)
    }
}


impl TryFrom<UserRow> for User {
    type Error = AppError;

    fn try_from(row: UserRow) -> Result<Self, Self::Error> {
        Ok(row.to_domain())
    }
}

impl TryFrom<&UserRow> for User {
    type Error = AppError;

    fn try_from(row: &UserRow) -> Result<Self, Self::Error> {
        Ok(row.to_domain())
    }
}
