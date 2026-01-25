// src/domain/services/error.rs
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("服务不存在: {0}")]
    NotFound(String),

    #[error("服务编码已存在: {0}")]
    DuplicateCode(String),

    #[error("服务编码错误: {0}")]
    InvalidCode(#[from] super::value_objects::ServiceCodeError),

    #[error("服务名称错误: {0}")]
    InvalidName(#[from] super::value_objects::ServiceNameError),

    #[error("无效的服务ID: {0}")]
    InvalidId(String),

    #[error("数据库错误: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("内部错误: {0}")]
    Internal(String),
}
