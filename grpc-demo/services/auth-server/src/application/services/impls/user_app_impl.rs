use crate::domain::identity::repositories::user_repository::UserRepository;
use crate::infrastructure::persistence::repositories::user_repository_impl::UserRepositoryImpl;
use std::sync::Arc;

use crate::domain::identity::entities::user::User;
use crate::infrastructure::web::dto::user::response::UserResponse;

/// 用户应用服务实现
#[derive(Clone)]
pub struct UserAppImpl {
    user_repository: Arc<dyn UserRepository + Send + Sync>,
    db_pool: sqlx::PgPool,
}

impl UserAppImpl {
    /// 创建新的用户应用服务
    pub fn new(db_pool: sqlx::PgPool) -> Self {
        let user_repository = Arc::new(UserRepositoryImpl::new(db_pool.clone()));

        Self {
            user_repository,
            db_pool,
        }
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
