use crate::domain::identity::entities::user::User;
use crate::domain::identity::entities::user::UserStatus;
use crate::domain::identity::repositories::user_repository::UserRepository;
use async_trait::async_trait;
use common::error::AppError;
use sqlx::types::Json;
use std::sync::Arc;

use sqlx::PgPool;
use tracing::{debug, error};

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
    async fn find_by_id(&self, user_id: &str) -> Result<User, AppError> {
        debug!("查找用户: id={}", user_id);
        let user = sqlx::query_as!(
            User,
            r#"SELECT
                id,
                username,
                email,
                phone,
                password_hash,
                display_name,
                avatar_url,
                roles AS "roles: Json<Vec<String>>",
                permissions AS "permissions: Json<Vec<String>>",
                email_verified,
                phone_verified,
                status        AS "status: crate::domain::identity::entities::user::UserStatus",
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
            FROM sys_user
            WHERE id = $1
              AND deleted_at IS NULL
      "#,
            user_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            error!("查找用户失败: {} - {}", user_id, e);
            AppError::Database(e.to_string())
        })?
        .ok_or_else(|| {
            error!("用户不存在: {}", user_id);
            AppError::NotFound(format!("用户ID {} 不存在用户", user_id))
        })?;
        Ok(user)
    }
}

// TODO
