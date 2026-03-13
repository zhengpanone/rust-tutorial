use crate::domain::identity::entity::user::User;
use crate::domain::identity::repository::user_repository::UserRepository;
use async_trait::async_trait;
use common::error::{AppError, AppResult};
use std::sync::Arc;

use crate::domain::identity::{Email, Username};

use crate::infrastructure::persistence::models::user_row::UserRow;
use crate::infrastructure::web::dto::user::request::UserFilter;
use common::web::pagination::{PaginationParams, SortOrder};
use sqlx::{PgPool, Postgres, QueryBuilder};
use tracing::{debug, error, info};
use uuid::Uuid;

/// 用户仓储实现
#[derive(Clone)]
pub struct UserRepositoryImpl {
    pool: PgPool,
    redis_pool: Option<Arc<deadpool_redis::Pool>>,
}

impl UserRepositoryImpl {
    /// 创建新的用户仓库
    pub fn new(pool: PgPool, redis_pool: Option<Arc<deadpool_redis::Pool>>) -> Self {
        Self { pool, redis_pool }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn find_by_id(&self, user_id: &Uuid) -> AppResult<Option<User>> {
        debug!("查找用户: id={}", user_id);

        let row = sqlx::query_as!(
            UserRow,
            r#"SELECT
                id,
                username,
                email,
                phone,
                password_hash,
                display_name,
                avatar_url,
                roles AS "roles: sqlx::types::Json<Vec<String>>",
                permissions AS "permissions: sqlx::types::Json<Vec<String>>",
                email_verified,
                phone_verified,
                status        AS "status: common::enums::user::UserStatus",
                last_login_at,
                login_count,
                failed_login_count,
                last_failed_login_at,
                locked_at,
                locked_until,
                lock_reason,
                password_changed_at,
                password_expires_at,
                is_first_login,
                last_activity_at,
                timezone,
                language,
                metadata AS "metadata: serde_json::Value",
                created_at,
                updated_at,
                deleted_at
            FROM sys_user
            WHERE id = $1
              AND deleted_at IS NULL
      "#,
            *user_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            error!("查找用户失败: {} - {}", user_id, e);
            AppError::Database(e.to_string())
        })?;

        // 转换 UserRow -> User 并处理可能的解析错误
        match row {
            Some(r) => {
                let user = r.to_domain(); // to_domain 返回 AppResult<User>
                Ok(Some(user))
            }
            None => Ok(None),
        }
    }
    /// 通过用户名查找用户
    async fn find_by_username(&self, username: &Username) -> AppResult<Option<User>> {
        debug!("查找用户: username={}", username);

        // 先查出用户 ID
        let row = sqlx::query!(
            r#"
            SELECT id FROM sys_user WHERE username = $1 AND deleted_at IS NULL
            "#,
            username.as_str()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            error!("通过用户名查找用户失败: {} - {}", username, e);
            AppError::Database(e.to_string())
        })?;
        match row {
            Some(row) => self.find_by_id(&row.id).await,
            None => Ok(None),
        }
    }

    /// 通过邮箱查找用户
    async fn find_by_email(&self, email: &Email) -> AppResult<Option<User>> {
        debug!("查找用户: email={}", email);

        let row = sqlx::query!(
            r#"
            SELECT id FROM sys_user WHERE email = $1 AND deleted_at IS NULL
            "#,
            email.as_str()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            error!("查找用户失败: {} - {}", email, e);
            AppError::Database(e.to_string())
        })?;
        match row {
            Some(row) => self.find_by_id(&row.id).await,
            None => Ok(None),
        }
    }

    /// 通过手机号查找用户
    async fn find_by_phone(&self, phone: &str) -> AppResult<Option<User>> {
        debug!("查找用户: phone={}", phone);

        let row = sqlx::query!(
            r#"
            SELECT id FROM sys_user WHERE phone = $1 AND deleted_at IS NULL
            "#,
            phone
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            error!("查找用户失败: {} - {}", phone, e);
            AppError::Database(e.to_string())
        })?;
        match row {
            Some(row) => self.find_by_id(&row.id).await,
            None => Ok(None),
        }
    }

    /// 通过用户名查找用户是否存在
    async fn exists_by_username(&self, username: &str) -> AppResult<bool> {
        debug!("检查用户是否存在: username={}", username);

        let result = sqlx::query!(
            r#"
            SELECT EXISTS(SELECT 1 FROM sys_user WHERE username = $1 AND deleted_at IS NULL) as "exists!"
            "#,
            username
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            error!("检查用户是否存在失败: {} - {}", username, e);
            AppError::Database(e.to_string())
        })?;
        Ok(result.exists)
    }

    /// 通过邮箱查找用户是否存在
    async fn exists_by_email(&self, email: &str) -> AppResult<bool> {
        debug!("检查用户是否存在: email={}", email);

        let result = sqlx::query!(
            r#"
            SELECT EXISTS(SELECT 1 FROM sys_user WHERE email = $1 AND deleted_at IS NULL) as "exists!"
            "#,
            email
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            error!("检查用户是否存在失败: {} - {}", email, e);
            AppError::Database(e.to_string())
        })?;
        Ok(result.exists)
    }

    /// 通过手机号查找用户是否存在
    async fn exists_by_phone(&self, phone: &str) -> AppResult<bool> {
        debug!("检查用户是否存在: phone={}", phone);

        let result = sqlx::query!(
            r#"
            SELECT EXISTS(SELECT 1 FROM sys_user WHERE phone = $1 AND deleted_at IS NULL) as "exists!"
            "#,
            phone
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            error!("检查用户是否存在失败: {} - {}", phone, e);
            AppError::Database(e.to_string())
        })?;
        Ok(result.exists)
    }

    /// 保存用户
    async fn save(&self, user: User) -> AppResult<User> {
        debug!("保存用户: {}", user.id);

        // 提前序列化，避免在宏内做复杂操作
        let roles = serde_json::to_value(&*user.roles).unwrap_or(serde_json::json!([]));
        let permissions = serde_json::to_value(&*user.permissions).unwrap_or(serde_json::json!([]));
        let login_count = user.login_count as i32;
        let failed_login_count = user.failed_login_count as i32;

        let _ = sqlx::query!(
            r#"
            INSERT INTO sys_user (
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
                login_count,
                failed_login_count,
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
                deleted_at
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12::user_status_enum, -- 加类型转换
                $13, $14, $15, $16, $17, $18,$19,$20,$21,$22,$23,$24,$25,$26,$27,$28)
                "#,
            Uuid::try_from(user.id.clone())?,
            user.username,
            user.email,
            user.phone,
            user.password_hash,
            user.display_name,
            user.avatar_url,
            roles,
            permissions,
            user.email_verified,
            user.phone_verified,
            user.status as _, // as _ 跳过编译期检查
            user.last_login_at,
            login_count,
            failed_login_count,
            user.last_failed_login_at,
            user.locked_at,
            user.lock_reason,
            user.password_changed_at,
            user.password_expires_at,
            user.is_first_login,
            user.last_activity_at,
            user.timezone,
            user.language,
            user.metadata,
            user.created_at,
            user.updated_at,
            user.deleted_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| {
            error!("保存用户失败: {} - {}", user.id, e);
            AppError::Database(e.to_string())
        })?;

        Ok(user)
    }

    /// 获取所有用户
    async fn find_all(
        &self,
        filter: &UserFilter,
        pagination: &PaginationParams,
    ) -> AppResult<Vec<User>> {
        debug!("Finding all users with filter: {:?}", filter);

        // 构建查询
        let mut query_builder =
            QueryBuilder::<Postgres>::new("SELECT * FROM sys_user WHERE deleted_at IS NULL");

        // 应用过滤器
        if let Some(status) = &filter.status {
            query_builder.push(" AND status = ");
            query_builder.push_bind(status.to_string());
        }

        if let Some(email_verified) = filter.email_verified {
            query_builder.push(" AND email_verified = ");
            query_builder.push_bind(email_verified);
        }

        if let Some(created_after) = filter.created_after {
            query_builder.push(" AND created_at > ");
            query_builder.push_bind(created_after);
        }

        if let Some(created_before) = filter.created_before {
            query_builder.push(" AND created_at < ");
            query_builder.push_bind(created_before);
        }

        if let Some(search) = &filter.search {
            query_builder.push(" AND (username ILIKE ");
            query_builder.push_bind(format!("%{}%", search));
            query_builder.push(" OR email ILIKE ");
            query_builder.push_bind(format!("%{}%", search));
            query_builder.push(" OR first_name ILIKE ");
            query_builder.push_bind(format!("%{}%", search));
            query_builder.push(" OR last_name ILIKE ");
            query_builder.push_bind(format!("%{}%", search));
            query_builder.push(")");
        }

        // 应用排序
        query_builder.push(" ORDER BY ");
        // match sort_by {
        //     UserSortBy::Username => query_builder.push("username"),
        //     UserSortBy::Email => query_builder.push("email"),
        //     UserSortBy::FirstName => query_builder.push("first_name"),
        //     UserSortBy::LastName => query_builder.push("last_name"),
        //     UserSortBy::CreatedAt => query_builder.push("created_at"),
        //     UserSortBy::UpdatedAt => query_builder.push("updated_at"),
        // }

        // match sort_direction {
        //     SortOrder::Asc => query_builder.push(" ASC"),
        //     SortOrder::Desc => query_builder.push(" DESC"),
        // }

        // 应用分页
        query_builder.push(" LIMIT ");
        query_builder.push_bind(pagination.page_size as i64);
        query_builder.push(" OFFSET ");
        query_builder.push_bind((pagination.page_size * pagination.page - 1) as i64);

        // 查询 UserRow
        let user_rows = query_builder
            .build_query_as::<UserRow>()
            .fetch_all(&self.pool)
            .await
            .map_err(|e| {
                error!("Database error finding all users: {}", e);
                AppError::Database(e.to_string())
            })?;

        // 转换为 User 领域实体
        let users: Vec<User> = user_rows.into_iter().map(|row| row.to_domain()).collect();

        Ok(users)
    }

    async fn count(&self, filter: &UserFilter) -> AppResult<u64> {
        info!("Counting users with filter: {:?}", filter);
        // 构建查询
        let mut query_builder =
            QueryBuilder::<Postgres>::new("SELECT COUNT(*) FROM users WHERE deleted_at IS NULL");

        // 应用过滤器
        if let Some(status) = &filter.status {
            query_builder.push(" AND status = ");
            query_builder.push_bind(status.to_string());
        }

        if let Some(email_verified) = filter.email_verified {
            query_builder.push(" AND email_verified = ");
            query_builder.push_bind(email_verified);
        }

        if let Some(created_after) = filter.created_after {
            query_builder.push(" AND created_at > ");
            query_builder.push_bind(created_after);
        }

        if let Some(created_before) = filter.created_before {
            query_builder.push(" AND created_at < ");
            query_builder.push_bind(created_before);
        }

        if let Some(search) = &filter.search {
            query_builder.push(" AND (username ILIKE ");
            query_builder.push_bind(format!("%{}%", search));
            query_builder.push(" OR email ILIKE ");
            query_builder.push_bind(format!("%{}%", search));
            query_builder.push(" OR first_name ILIKE ");
            query_builder.push_bind(format!("%{}%", search));
            query_builder.push(" OR last_name ILIKE ");
            query_builder.push_bind(format!("%{}%", search));
            query_builder.push(")");
        }

        let result = query_builder
            .build_query_scalar::<i64>()
            .fetch_one(&self.pool)
            .await
            .map_err(|e| {
                error!("Database error counting users: {}", e);
                AppError::Database(e.to_string())
            })?;

        Ok(result as u64)
    }
}
