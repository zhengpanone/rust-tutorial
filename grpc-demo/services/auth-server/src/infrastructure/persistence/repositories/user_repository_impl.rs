use crate::domain::identity::entities::user::User;
use crate::domain::identity::repositories::user_repository::UserRepository;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use common::error::AppError;
use common::security::jwt::claim::UserStatus;
use sqlx::types::Json;
use sqlx::{PgPool, Row};
use tracing::{debug, error};

/// 用户仓储实现
#[derive(Clone)]
pub struct UserRepositoryImpl {
    pool: PgPool,
}

impl UserRepositoryImpl {
    /// 创建新的用户仓库
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    /// 从数据库行转换为用户实体
    fn row_to_user(&self, row: &sqlx::postgres::PgRow) -> Result<User, AppError> {
        let id: String = row.get("id");
        let username: String = row.get("username");
        let email: String = row.get("email");
        let phone: Option<String> = row.get("phone");
        let password_hash: String = row.get("password_hash");
        let display_name: String = row.get("display_name");
        let avatar_url: Option<String> = row.get("avatar_url");

        // 解析JSON字段
        let roles: Vec<String> = row.get::<Json<Vec<String>>, _>("roles").0;
        let permissions: Vec<String> = row.get::<Json<Vec<String>>, _>("permissions").0;

        let email_verified: bool = row.get("email_verified");
        let phone_verified: bool = row.get("phone_verified");

        let status_str: String = row.get("status");
        let status: UserStatus = match status_str.as_str() {
            "active" => UserStatus::Active,
            "inactive" => UserStatus::Inactive,
            "suspended" => UserStatus::Suspended,
            "locked" => UserStatus::Locked,
            "pending" => UserStatus::Pending,
            "deleted" => UserStatus::Deleted,
            _ => UserStatus::Pending,
        };

        let last_login_at: Option<DateTime<Utc>> = row.get("last_login_at");
        let login_count: i32 = row.get("login_count");
        let failed_login_count: i32 = row.get("failed_login_count");
        let last_failed_login_at: Option<DateTime<Utc>> = row.get("last_failed_login_at");
        let locked_at: Option<DateTime<Utc>> = row.get("locked_at");
        let lock_reason: Option<String> = row.get("lock_reason");
        let password_changed_at: Option<DateTime<Utc>> = row.get("password_changed_at");
        let password_expires_at: Option<DateTime<Utc>> = row.get("password_expires_at");
        let is_first_login: bool = row.get("is_first_login");
        let last_activity_at: Option<DateTime<Utc>> = row.get("last_activity_at");
        let timezone: Option<String> = row.get("timezone");
        let language: Option<String> = row.get("language");
        let metadata: Option<serde_json::Value> = row.get("metadata");
        let created_at: DateTime<Utc> = row.get("created_at");
        let updated_at: DateTime<Utc> = row.get("updated_at");
        let deleted_at: Option<DateTime<Utc>> = row.get("deleted_at");

        let user = User {
            id,
            username,
            email,
            phone,
            password_hash,
            display_name,
            avatar_url,
            roles,
            permissions,
            email_verified,
            phone_verified,
            status,
            last_login_at,
            login_count: login_count as u32,
            failed_login_count: failed_login_count as u32,
            last_failed_login_at,
            locked_at,
            lock_reason,
            password_changed_at,
            password_expires_at,
            is_first_login,
            last_activity_at,
            timezone,
            language,
            metadata,
            created_at,
            updated_at,
            deleted_at,
        };

        Ok(user)
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn find_by_id(&self, user_id: &str) -> Result<User, AppError> {
        debug!("查找用户: id={}", user_id);
        let row = sqlx::query!(
            r#"SELECT * FROM users WHERE id = $1 AND deleted_at IS NULL"#,
            user_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            error!("查找用户失败: {} - {}", user_id, e);
            AppError::Database(e)
        })?;

        match row {
            Some(row) => {
                let user = User {
                    id: row.id,
                    username: row.username,
                    email: row.email,
                    phone: row.phone,
                    password_hash: row.password_hash,
                    display_name: row.display_name,
                    avatar_url: row.avatar_url,
                    roles: row.roles.map(|r| r.0).unwrap_or_default(),
                    permissions: row.permissions.map(|p| p.0).unwrap_or_default(),
                    email_verified: row.email_verified,
                    phone_verified: row.phone_verified,
                    status: match row.status.as_str() {
                        "active" => UserStatus::Active,
                        "inactive" => UserStatus::Inactive,
                        "suspended" => UserStatus::Suspended,
                        "locked" => UserStatus::Locked,
                        "pending" => UserStatus::Pending,
                        "deleted" => UserStatus::Deleted,
                        _ => UserStatus::Pending,
                    },
                    last_login_at: row.last_login_at,
                    login_count: row.login_count as u32,
                    failed_login_count: row.failed_login_count as u32,
                    last_failed_login_at: row.last_failed_login_at,
                    locked_at: row.locked_at,
                    lock_reason: row.lock_reason,
                    password_changed_at: row.password_changed_at,
                    password_expires_at: row.password_expires_at,
                    is_first_login: row.is_first_login,
                    last_activity_at: row.last_activity_at,
                    timezone: row.timezone,
                    language: row.language,
                    metadata: row.metadata,
                    created_at: row.created_at,
                    updated_at: row.updated_at,
                    deleted_at: row.deleted_at,
                };
                Ok(user)
            }
            None => {
                error!("用户不存在: {}", user_id);
                AppError::NotFound(format!("用户ID {} 不存在用户", user_id))
            }
        }
    }

    // TODO
}
