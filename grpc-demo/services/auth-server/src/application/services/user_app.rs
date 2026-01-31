// src/application/services/user_app.rs

use crate::domain::identity::entities::user::User;
use async_trait::async_trait;
use common::error::AppError;

/// 用户应用服务 trait
#[async_trait]
pub trait UserApp: Send + Sync {
    /// 获取单个用户
    async fn get_user(&self, user_id: &str) -> Result<User, AppError>;

    // TODO 
}
