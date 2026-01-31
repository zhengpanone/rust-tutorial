// src/domain/identity/repositories/user_repository.rs

use crate::domain::identity::entities::user::User;
use async_trait::async_trait;
use common::error::AppError;

/// 用户仓储 trait
#[async_trait]
pub trait UserRepository: Send + Sync {
    /// 通过 ID 查找用户
    async fn find_by_id(&self, user_id: &str) -> Result<User, AppError>;

    // TODO
}
