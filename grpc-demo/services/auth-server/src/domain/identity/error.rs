// src/domain/identity/error.rs
use thiserror::Error;

#[derive(Debug, Error)]
pub enum IdentityError {
    #[error("用户未找到: {0}")]
    UserNotFound(String),

    #[error("邮箱已被注册: {0}")]
    EmailAlreadyExists(String),

    #[error("用户名已被占用: {0}")]
    UsernameAlreadyExists(String),

    #[error("手机号已被注册: {0}")]
    PhoneAlreadyExists(String),

    #[error("用户年龄不足，需要至少13岁")]
    UnderageUser,

    #[error("无效的用户名格式")]
    InvalidUsernameFormat,

    #[error("用户冲突: {field} 已被使用")]
    UserConflict { field: String },

    #[error("用户被锁定，原因: {reason}, 直到: {until}")]
    UserLocked { reason: String, until: String },

    #[error("密码强度不足: {0}")]
    WeakPassword(String),

    #[error("无效的角色: {0}")]
    InvalidRole(String),
}

impl From<IdentityError> for common::error::AppError {
    fn from(err: IdentityError) -> Self {
        match &err {
            IdentityError::UserNotFound(_) => common::error::AppError::NotFound(err.to_string()),
            IdentityError::EmailAlreadyExists(_)
            | IdentityError::UsernameAlreadyExists(_)
            | IdentityError::PhoneAlreadyExists(_) => {
                common::error::AppError::AlreadyExists(err.to_string())
            }
            IdentityError::UnderageUser
            | IdentityError::InvalidUsernameFormat
            | IdentityError::UserConflict { .. }
            | IdentityError::WeakPassword(_)
            | IdentityError::InvalidRole(_) => {
                common::error::AppError::BusinessRule(err.to_string())
            }
            IdentityError::UserLocked { .. } => {
                common::error::AppError::Authorization(err.to_string())
            }
        }
    }
}
