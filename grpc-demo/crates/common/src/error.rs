use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Formatter;
use thiserror::Error;
use tracing::{Level, error};
use utoipa::ToSchema;

/// 应用结果类型别名
pub type AppResult<T> = Result<T, AppError>;

/// 应用错误枚举
#[derive(Debug, Error, Serialize, Deserialize, ToSchema)]
#[serde(tag = "error_type", content = "error_data")]
pub enum AppError {
    /// 认证错误
    #[error("Authentication error: {0}")]
    Authentication(String),

    /// 授权错误
    #[error("Authorization error: {0}")]
    Authorization(String),

    /// 资源不存在错误
    #[error("Not found error: {0}")]
    NotFound(String),

    /// 资源已存在错误
    #[error("Already exists error: {0}")]
    AlreadyExists(String),

    /// 验证错误
    #[error("Validation error: {0}")]
    Validation(String),

    /// 业务规则错误
    #[error("Business rule error: {0}")]
    BusinessRule(String),

    #[error("Database error: {0}")]
    Database(String),

    /// 缓存错误
    #[error("Cache error: {0}")]
    Cache(String),

    /// 网络错误
    #[error("网络错误: {0}")]
    Network(String),

    /// 外部服务错误
    #[error("外部服务错误: {0}")]
    ExternalService(String),

    /// 限流错误
    #[error("请求过于频繁: {0}")]
    RateLimit(String),

    /// 请求超时
    #[error("请求超时: {0}")]
    Timeout(String),

    /// 内部服务器错误
    #[error("服务器内部错误: {0}")]
    Internal(String),

    /// 配置错误
    #[error("配置错误: {0}")]
    Config(String),

    /// 文件操作错误
    #[error("文件操作失败: {0}")]
    File(String),

    /// 序列化/反序列化错误
    #[error("序列化错误: {0}")]
    Serialization(String),

    /// 并发错误
    #[error("并发操作冲突: {0}")]
    Concurrency(String),

    /// 不支持的媒体类型
    #[error("不支持的媒体类型: {0}")]
    UnsupportedMediaType(String),

    /// 请求体过大
    #[error("请求体过大: {0}")]
    PayloadTooLarge(String),

    /// 方法不允许
    #[error("请求方法不允许: {0}")]
    MethodNotAllowed(String),

    #[error("Password hashing error: {0}")]
    Hashing(String),
}

impl AppError {
    /// 获取HTTP状态码
    pub fn status_code(&self) -> StatusCode {
        match self {
            AppError::Authentication(_) => StatusCode::UNAUTHORIZED,
            AppError::Authorization(_) => StatusCode::FORBIDDEN,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::AlreadyExists(_) => StatusCode::CONFLICT,
            AppError::Validation(_) => StatusCode::BAD_REQUEST,
            AppError::BusinessRule(_) => StatusCode::UNPROCESSABLE_ENTITY,
            AppError::RateLimit(_) => StatusCode::TOO_MANY_REQUESTS,
            AppError::Timeout(_) => StatusCode::REQUEST_TIMEOUT,
            AppError::UnsupportedMediaType(_) => StatusCode::UNSUPPORTED_MEDIA_TYPE,
            AppError::PayloadTooLarge(_) => StatusCode::PAYLOAD_TOO_LARGE,
            AppError::MethodNotAllowed(_) => StatusCode::METHOD_NOT_ALLOWED,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn error_code(&self) -> &'static str {
        match self {
            AppError::Authentication(_) => "AUTH_ERROR",
            AppError::Authorization(_) => "FORBIDDEN",
            AppError::NotFound(_) => "NOT_FOUND",
            AppError::AlreadyExists(_) => "ALREADY_EXISTS",
            AppError::Validation(_) => "VALIDATION_ERROR",
            AppError::BusinessRule(_) => "BUSINESS_RULE_VIOLATION",
            AppError::Database(_) => "DATABASE_ERROR",
            AppError::Cache(_) => "CACHE_ERROR",
            AppError::Network(_) => "NETWORK_ERROR",
            AppError::ExternalService(_) => "EXTERNAL_SERVICE_ERROR",
            AppError::RateLimit(_) => "RATE_LIMIT_EXCEEDED",
            AppError::Timeout(_) => "TIMEOUT",
            AppError::Internal(_) => "INTERNAL_ERROR",
            AppError::Config(_) => "CONFIG_ERROR",
            AppError::File(_) => "FILE_ERROR",
            AppError::Serialization(_) => "SERIALIZATION_ERROR",
            AppError::Concurrency(_) => "CONCURRENCY_ERROR",
            AppError::UnsupportedMediaType(_) => "UNSUPPORTED_MEDIA_TYPE",
            AppError::PayloadTooLarge(_) => "PAYLOAD_TOO_LARGE",
            AppError::MethodNotAllowed(_) => "METHOD_NOT_ALLOWED",
            AppError::Hashing(_) => "HASHING_ERROR",
        }
    }

    /// 获取错误严重级别
    pub fn severity(&self) -> ErrorSeverity {
        match self {
            AppError::Internal(_)
            | AppError::Database(_)
            | AppError::Network(_)
            | AppError::ExternalService(_) => ErrorSeverity::Error,
            AppError::Validation(_)
            | AppError::BusinessRule(_)
            | AppError::NotFound(_)
            | AppError::AlreadyExists(_) => ErrorSeverity::Warning,
            _ => ErrorSeverity::Info,
        }
    }

    pub fn should_log(&self) -> bool {
        match self.severity() {
            ErrorSeverity::Error => true,
            ErrorSeverity::Warning => true,
            ErrorSeverity::Info => false,
        }
    }

    /// 是否为客户端错误
    pub fn is_client_error(&self) -> bool {
        matches!(
            self,
            AppError::Authentication(_)
                | AppError::Authorization(_)
                | AppError::NotFound(_)
                | AppError::AlreadyExists(_)
                | AppError::Validation(_)
                | AppError::BusinessRule(_)
                | AppError::RateLimit(_)
                | AppError::Timeout(_)
                | AppError::UnsupportedMediaType(_)
                | AppError::PayloadTooLarge(_)
                | AppError::MethodNotAllowed(_)
        )
    }
    /// 是否为服务器错误
    pub fn is_server_error(&self) -> bool {
        !self.is_client_error()
    }

    /// 获取用户友好消息
    pub fn user_message(&self) -> String {
        match self {
            AppError::Authentication(msg) => format!("身份验证失败: {}", msg),
            AppError::Authorization(msg) => format!("没有权限: {}", msg),
            AppError::NotFound(msg) => format!("找不到资源: {}", msg),
            AppError::AlreadyExists(msg) => format!("资源已存在: {}", msg),
            AppError::Validation(msg) => format!("输入验证失败: {}", msg),
            AppError::BusinessRule(msg) => format!("操作不允许: {}", msg),
            AppError::Database(_) => "数据库操作失败，请稍后重试".to_string(),
            AppError::Cache(_) => "缓存服务异常，请稍后重试".to_string(),
            AppError::Network(_) => "网络连接异常，请检查网络".to_string(),
            AppError::ExternalService(_) => "外部服务异常，请稍后重试".to_string(),
            AppError::RateLimit(msg) => format!("请求过于频繁: {}", msg),
            AppError::Timeout(msg) => format!("请求超时: {}", msg),
            AppError::Internal(_) => "服务器内部错误，请联系管理员".to_string(),
            AppError::Config(_) => "系统配置错误，请联系管理员".to_string(),
            AppError::File(msg) => format!("文件操作失败: {}", msg),
            AppError::Serialization(_) => "数据处理失败，请稍后重试".to_string(),
            AppError::Concurrency(msg) => format!("操作冲突: {}", msg),
            AppError::UnsupportedMediaType(msg) => format!("不支持的媒体类型: {}", msg),
            AppError::PayloadTooLarge(msg) => format!("请求体过大: {}", msg),
            AppError::MethodNotAllowed(msg) => format!("请求方法不允许: {}", msg),
            _ => "未知错误，请联系管理员".to_string(),
        }
    }

    pub fn log(&self, request_id: Option<&str>, path: Option<&str>) {
        if !self.should_log() {
            return;
        }

        let level = match self.severity() {
            ErrorSeverity::Error => Level::ERROR,
            ErrorSeverity::Warning => Level::WARN,
            ErrorSeverity::Info => Level::INFO,
        };

        let log_message = format!(
            "[{}] {}: {} (code: {})",
            level,
            self.error_code(),
            self,
            self.status_code()
        );

        let mut log_details = Vec::new();
        if let Some(req_id) = request_id {
            log_details.push(format!("request_id: {}", req_id));
        }
        if let Some(p) = path {
            log_details.push(format!("path: {}", p));
        }
        if !log_details.is_empty() {
            error!("{} [{}]", log_message, log_details.join(", "));
        } else {
            error!("{}", log_message)
        }
    }
}

/// 实现 IntoResponse 支持 Axum HTTP 响应
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // 记录错误日志
        // self.log(None,None);
        // let status = self.status_code();
        // let headers = self.build_headers();
        //
        // let api_error = ApiError {
        //     error_type: self.error_data(),
        //     error_message: self.to_string(),
        //     error_code: self.error_code(),
        //     error_severity: self.severity().to_string(),
        // };
        todo!()
    }
}

/// 为 sqlx::Error 提供 From 实现
impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        Self::Database(err.to_string())
    }
}

/// 为 sqlx::migrate::MigrateError 提供 From 实现
impl From<sqlx::migrate::MigrateError> for AppError {
    fn from(err: sqlx::migrate::MigrateError) -> Self {
        Self::Database(err.to_string())
    }
}

impl From<redis::RedisError> for AppError {
    fn from(err: redis::RedisError) -> Self {
        Self::Cache(err.to_string())
    }
}

impl From<deadpool_redis::CreatePoolError> for AppError {
    fn from(err: deadpool_redis::CreatePoolError) -> Self {
        Self::Cache(err.to_string())
    }
}

impl From<deadpool_redis::PoolError> for AppError {
    fn from(err: deadpool_redis::PoolError) -> Self {
        Self::Cache(err.to_string())
    }
}

impl From<deadpool_redis::redis::RedisError> for AppError {
    fn from(err: deadpool_redis::redis::RedisError) -> Self {
        Self::Cache(err.to_string())
    }
}

/// 为 std::io::Error 提供 From 实现
impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        Self::Internal(err.to_string())
    }
}


/// 错误严重级别
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub enum ErrorSeverity {
    Error = 3,
    Warning = 2,
    Info = 1,
}

impl fmt::Display for ErrorSeverity {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ErrorSeverity::Error => write!(f, "ERROR"),
            ErrorSeverity::Warning => write!(f, "WARNING"),
            ErrorSeverity::Info => write!(f, "INFO"),
        }
    }
}
