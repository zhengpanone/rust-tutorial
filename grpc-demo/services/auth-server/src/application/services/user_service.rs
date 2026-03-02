// src/application/services/user_service

use crate::domain::identity::entity::user::User;
use crate::infrastructure::web::dto::user::request::{
    CreateUserRequest, UpdateUserRequest, UserFilter,
};
use async_trait::async_trait;
use common::error::{AppError, AppResult};
use common::web::pagination::PaginatedData;
use common::web::response::Pagination;

/// 用户服务 trait
/// 应用服务 - 协调多个聚合、基础设施组件
/// 职责：事务管理、事件发布、外部服务调用
#[async_trait]
pub trait UserService: Send + Sync {
    /// 创建用户
    async fn create_user(&self, request: CreateUserRequest) -> AppResult<User>;

    /// 获取单个用户
    async fn get_user(&self, user_id: &str) -> Result<Option<User>, AppError>;

    /// 获取用户列表
    async fn list_users(
        &self,
        filter: UserFilter,
        pagination: Pagination,
    ) -> Result<PaginatedData<Vec<User>>, AppError>;

    async fn update_user(
        &self,
        user_id: &str,
        request: UpdateUserRequest,
    ) -> Result<User, AppError>;

    /// 删除用户
    async fn delete_user(&self, user_id: &str) -> Result<(), AppError>;

    /// 启用用户
    async fn enabled_user(&self, user_id: &str) -> Result<(), AppError>;

    // TODO
}
