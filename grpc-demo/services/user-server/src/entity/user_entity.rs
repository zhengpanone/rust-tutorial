use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct UserEntity {
    pub id: String,
    pub username: String,
    pub password_hash: String,
    pub email: String,
    pub full_name: String,
    pub phone_number: String,
    pub role: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_login_at: Option<DateTime<Utc>>,
}
