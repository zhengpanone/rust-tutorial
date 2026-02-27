use crate::application::services::user_service::UserService;
use crate::domain::identity::models::user::User;
use crate::domain::identity::repositories::user_repository::UserRepository;
use common::security::password::hash_password;
use crate::infrastructure::web::dto::auth::request::validate_password_strength;
use crate::infrastructure::web::dto::user::request;
use crate::infrastructure::web::dto::user::request::{
    CreateUserRequest, UpdateUserRequest, UserFilter,
};
use crate::infrastructure::web::dto::user::response::UserResponse;
use async_trait::async_trait;
use common::error::{AppError, AppResult};
use common::web::pagination::PaginatedData;
use common::web::response::Pagination;
use std::sync::Arc;
use tracing::{error, info};
use uuid::Uuid;
use validator::Validate;

/// 用户应用服务实现
#[derive(Clone)]
pub struct UserServiceImpl {
    user_repository: Arc<dyn UserRepository + Send + Sync>,
    enable_validation: bool,
    enable_audit_logging: bool,
}

impl UserServiceImpl {
    /// 创建新的用户应用服务
    pub fn new(user_repository: Arc<dyn UserRepository + Send + Sync>) -> Self {
        Self {
            user_repository,
            enable_validation: true,
            enable_audit_logging: true,
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

    /// 验证请求
    fn validate_request<T: Validate>(&self, request: &T) -> AppResult<()> {
        if self.enable_validation {
            request
                .validate()
                .map_err(|err| AppError::Validation(err.to_string()))?;
        }
        Ok(())
    }

    /// 记录审计日志
    fn audit_log(&self, action: &str, user_id: Uuid, details: &str) {
        if self.enable_audit_logging {
            info!(
                "Audit log: action={}, user_id={}, details={}",
                action, user_id, details
            );
        }
    }
}

#[async_trait]
impl UserService for UserServiceImpl {
    async fn create_user(&self, request: CreateUserRequest) -> AppResult<User> {
        info!("Creating new user: username={}", request.username);

        // 验证请求
        self.validate_request(&request)?;

        // 检查用户名是否已存在
        if self
            .user_repository
            .exists_by_username(&request.username)
            .await?
        {
            return Err(AppError::Validation(format!(
                "User with username {} already exists",
                request.username
            )));
        }

        // 检查邮箱是否已存在
        if self.user_repository.exists_by_email(&request.email).await? {
            return Err(AppError::Validation(format!(
                "User with email {} already exists",
                request.email
            )));
        }

        if let Some(phone) = &request.phone {
            if self.user_repository.exists_by_phone(phone).await? {
                return Err(AppError::Validation(format!(
                    "User with phone {} already exists",
                    phone
                )));
            }
        }
        let password = match &request.password {
            Some(password) => {
                validate_password_strength(password)
                    .map_err(|e| AppError::Validation(e.to_string()))
                    .expect("password strength check failed");
                hash_password(password).expect("hash password failed")
            }
            None => hash_password("admin123").expect("hash password failed"),
        };
        let user: User = request.into_user(&password);

        let user = self
            .user_repository
            .save(user)
            .await
            .expect("save user failed");
        Ok(user)
    }

    async fn get_user(&self, user_id: &str) -> Result<Option<User>, AppError> {
        let user_id = Uuid::parse_str(user_id).unwrap_or_else(|e| {
            error!("Failed to parse user_id from claims.sub: {}", e);
            Uuid::new_v4()
        });
        let user = self.user_repository.find_by_id(&user_id).await?;
        Ok(user)
    }

    async fn list_users(
        &self,
        filter: UserFilter,
        pagination: Pagination,
    ) -> Result<PaginatedData<Vec<User>>, AppError> {
        todo!()
    }

    async fn update_user(
        &self,
        user_id: &str,
        request: UpdateUserRequest,
    ) -> Result<User, AppError> {
        todo!()
    }

    async fn delete_user(&self, user_id: &str) -> Result<(), AppError> {
        todo!()
    }

    async fn enabled_user(&self, user_id: &str) -> Result<(), AppError> {
        todo!()
    }
}
