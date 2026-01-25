use crate::models::user::UserEntity;
use chrono::Utc;
use sqlx::{Error, PgPool};
use uuid::Uuid;

#[derive(Clone)]
pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_by_id(&self, id: &str) -> sqlx::Result<Option<UserEntity>> {
        sqlx::query_as::<_, UserEntity>(
            r#"
            SELECT *
            FROM sys_user
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn create_user(&self, mut user: UserEntity) -> Result<UserEntity, Error> {
        let now = Utc::now();
        user.id = Uuid::new_v4().to_string();
        sqlx::query_as::<_, UserEntity>(
            r#"
            INSERT INTO sys_user (id, username, email, password_hash, created_at, updated_at) values ($1, $2, $3, $4, $5, $6)
            RETURNING *
            "#,
        )
            .bind(&user.id)
            .bind(&user.username)
            .bind(&user.email)
            .bind(&user.password_hash)
            .bind(now)
            .bind(now)
            .fetch_one(&self.pool)
            .await
    }
}
