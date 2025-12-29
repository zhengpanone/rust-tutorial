use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct UserEntity {
    pub id: String,
    pub username: String,
    pub email: String,
    pub full_name: String,
    pub phone_number: String,
    pub role: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
