// src/domain/identity/repository/user_repository.rs

use crate::domain::identity::models::user::User;
use crate::domain::identity::{Email, Username};
use async_trait::async_trait;
use common::error::AppResult;
use uuid::Uuid;

/// 用户仓储 trait
#[async_trait]
pub trait UserRepository: Send + Sync {
    /// 通过 ID 查找用户
    async fn find_by_id(&self, user_id: &Uuid) -> AppResult<Option<User>>;

    /// 通过用户名查找用户
    async fn find_by_username(&self, username: &Username) -> AppResult<Option<User>>;

    /// 通过邮箱查找用户
    async fn find_by_email(&self, email: &Email) -> AppResult<Option<User>>;

    /// 通过手机号查找用户
    async fn find_by_phone(&self, phone: &str) -> AppResult<Option<User>>;

    /// 通过用户名查找用户是否存在
    async fn exists_by_username(&self, username: &str) -> AppResult<bool>;

    /// 通过邮箱查找用户是否存在
    async fn exists_by_email(&self, email: &str) -> AppResult<bool>;

    /// 通过手机号查找用户是否存在
    async fn exists_by_phone(&self, phone: &str) -> AppResult<bool>;

    async fn save(&self, user: User) -> AppResult<User>;
}
