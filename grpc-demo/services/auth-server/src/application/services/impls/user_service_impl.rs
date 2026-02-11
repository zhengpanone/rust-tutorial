use crate::domain::identity::repositories::user_repository::UserRepository;
use std::sync::Arc;
use async_trait::async_trait;
use common::error::AppError;
use common::web::pagination::PaginatedData;
use common::web::response::Pagination;
use crate::application::services::user_service::UserService;
use crate::domain::identity::entities::user::User;
use crate::infrastructure::web::dto::user::request::{CreateUserRequest, UpdateUserRequest, UserFilter};
use crate::infrastructure::web::dto::user::response::UserResponse;

/// 用户应用服务实现
#[derive(Clone)]
pub struct UserServiceImpl {
    user_repository: Arc<dyn UserRepository + Send + Sync>,
}

impl UserServiceImpl {
    /// 创建新的用户应用服务
    pub fn new(user_repository: Arc<dyn UserRepository + Send + Sync>) -> Self {
        Self { user_repository }
    }
    /// 转换为用户响应
    fn to_user_response(&self, user: User) -> UserResponse {
        todo!()
        // UserResponse {
        //     id: user.id,
        //     username: user.username,
        //     email: user.email,
        //     phone: user.phone,
        //     display_name: user.display_name,
        //     avatar_url: user.avatar_url,
        //     roles: user.roles,
        //     permissions: user.permissions,
        //     email_verified: user.email_verified,
        //     phone_verified: user.phone_verified,
        //     status: self.to_dto_user_status(&user.status),
        //     last_login_at: user.last_login_at,
        //     login_count: user.login_count,
        //     failed_login_count: user.failed_login_count,
        //     last_failed_login_at: user.last_failed_login_at,
        //     created_at: user.created_at,
        //     updated_at: user.updated_at,
        //     metadata: user.metadata,
        // }
    }
}

#[async_trait]
impl UserService for UserServiceImpl {
    async fn create_user(&self, request: CreateUserRequest) -> Result<User, AppError> {
        todo!()
    }

    async fn get_user(&self, user_id: &str) -> Result<User, AppError> {
        // self.user_repository.get_user(user_id).await
        todo!()
    }

    async fn list_users(&self, filter: UserFilter, pagination: Pagination) -> Result<PaginatedData<Vec<User>>, AppError> {
        todo!()
    }

    async fn update_user(&self, user_id: &str, request: UpdateUserRequest) -> Result<User, AppError> {
        todo!()
    }

    async fn delete_user(&self, user_id: &str) -> Result<(), AppError> {
        todo!()
    }

    async fn enabled_user(&self, user_id: &str) -> Result<(), AppError> {
        todo!()
    }
}
