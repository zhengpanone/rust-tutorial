use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use utoipa::ToSchema;
use validator::ValidationErrors;

/// API错误响应
#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub struct ApiError {
    /// 错误码
    pub code: String,

    /// 错误消息
    pub message: String,

    /// 错误详情
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,

    /// 请求ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,

    /// 时间戳
    pub timestamp: DateTime<Utc>,

    /// 验证错误详情
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_errors: Option<Vec<ValidationErrorDetail>>,

    /// 错误堆栈（仅开发环境）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_trace: Option<String>,

    /// 错误文档链接
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation_url: Option<String>,

    /// 错误解决方案建议
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggestion: Option<String>,

    /// 原始错误（仅开发环境）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_error: Option<String>,
}

/// 验证错误详情
#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub struct ValidationErrorDetail {
    /// 字段名
    pub field: String,
    /// 错误消息
    pub message: String,
    /// 错误码
    pub code: String,
    /// 错误参数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Vec<(String, String)>>,
}

impl ApiError {
    /// 创建API错误
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            details: None,
            request_id: None,
            timestamp: Utc::now(),
            validation_errors: None,
            stack_trace: None,
            documentation_url: None,
            suggestion: None,
            original_error: None,
        }
    }
    /// 创建认证错误
    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self {
            code: "UNAUTHORIZED".into(),
            message: message.into(),
            details: None,
            request_id: None,
            timestamp: Utc::now(),
            validation_errors: None,
            stack_trace: None,
            documentation_url: Some("https://docs.example.com/api/error/unauthorized".to_string()),
            suggestion: Some("请提供有效的认证令牌".to_string()),
            original_error: None,
        }
    }

    /// 创建权限错误
    pub fn forbidden(message: impl Into<String>) -> Self {
        Self {
            code: "FORBIDDEN".to_string(),
            message: message.into(),
            details: None,
            request_id: None,
            timestamp: Utc::now(),
            validation_errors: None,
            stack_trace: None,
            documentation_url: Some("https://docs.example.com/permissions".to_string()),
            suggestion: Some("请检查您的权限或联系管理员".to_string()),
            original_error: None,
        }
    }

    /// 创建资源未找到错误
    pub fn not_found(message: impl Into<String>) -> Self {
        Self {
            code: "NOT_FOUND".to_string(),
            message: message.into(),
            details: None,
            request_id: None,
            timestamp: Utc::now(),
            validation_errors: None,
            stack_trace: None,
            documentation_url: None,
            suggestion: Some("请检查资源ID是否正确".to_string()),
            original_error: None,
        }
    }

    /// 创建资源已存在错误
    pub fn conflict(message: impl Into<String>) -> Self {
        Self {
            code: "CONFLICT".to_string(),
            message: message.into(),
            details: None,
            request_id: None,
            timestamp: Utc::now(),
            validation_errors: None,
            stack_trace: None,
            documentation_url: None,
            suggestion: Some("请检查资源是否已存在".to_string()),
            original_error: None,
        }
    }

    /// 创建验证错误
    pub fn validation(message: impl Into<String>) -> Self {
        Self {
            code: "VALIDATION_ERROR".to_string(),
            message: message.into(),
            details: None,
            request_id: None,
            timestamp: Utc::now(),
            validation_errors: None,
            stack_trace: None,
            documentation_url: Some("https://docs.example.com/validation".to_string()),
            suggestion: Some("请检查输入参数".to_string()),
            original_error: None,
        }
    }

    /// 创建验证错误（带详细错误信息）
    pub fn validation_detailed(errors: Vec<ValidationErrorDetail>) -> Self {
        Self {
            code: "VALIDATION_ERROR".to_string(),
            message: "请求参数验证失败".to_string(),
            details: None,
            request_id: None,
            timestamp: Utc::now(),
            validation_errors: Some(errors),
            stack_trace: None,
            documentation_url: Some("https://docs.example.com/validation".to_string()),
            suggestion: Some("请检查输入参数".to_string()),
            original_error: None,
        }
    }

    /// 创建限流错误
    pub fn rate_limit(message: impl Into<String>) -> Self {
        Self {
            code: "RATE_LIMIT_EXCEEDED".to_string(),
            message: message.into(),
            details: None,
            request_id: None,
            timestamp: Utc::now(),
            validation_errors: None,
            stack_trace: None,
            documentation_url: Some("https://docs.example.com/rate-limiting".to_string()),
            suggestion: Some("请稍后重试".to_string()),
            original_error: None,
        }
    }

    /// 创建内部服务器错误
    pub fn internal(message: impl Into<String>) -> Self {
        Self {
            code: "INTERNAL_SERVER_ERROR".to_string(),
            message: message.into(),
            details: None,
            request_id: None,
            timestamp: Utc::now(),
            validation_errors: None,
            stack_trace: None,
            documentation_url: Some("https://docs.example.com/errors".to_string()),
            suggestion: Some("请稍后重试或联系技术支持".to_string()),
            original_error: None,
        }
    }

    /// 从验证错误创建
    pub fn from_validation_errors(errors: ValidationErrors) -> Self {
        let validation_errors: Vec<ValidationErrorDetail> = convert_validate_errors_to_details(errors);
        Self::validation_detailed(validation_errors)
    }
    /// 设置错误详情
    pub fn with_details(mut self, details: impl Into<String>) -> Self {
        self.details = Some(details.into());
        self
    }

    /// 设置请求ID
    pub fn with_request_id(mut self, request_id: impl Into<String>) -> Self {
        self.request_id = Some(request_id.into());
        self
    }

    /// 设置错误堆栈
    pub fn with_stack_trace(mut self, stack_trace: impl Into<String>) -> Self {
        self.stack_trace = Some(stack_trace.into());
        self
    }

    /// 设置文档链接
    pub fn with_documentation_url(mut self, url: impl Into<String>) -> Self {
        self.documentation_url = Some(url.into());
        self
    }

    /// 设置解决方案建议
    pub fn with_suggestion(mut self, suggestion: impl Into<String>) -> Self {
        self.suggestion = Some(suggestion.into());
        self
    }

    /// 设置原始错误
    pub fn with_original_error(mut self, error: impl std::fmt::Display) -> Self {
        self.original_error = Some(error.to_string());
        self
    }
    /// 获取HTTP状态码
    pub fn status_code(&self) -> Option<axum::http::StatusCode> {
        match self.code.as_str() {
            "SUCCESS" | "CREATED" | "ACCEPTED" => Some(axum::http::StatusCode::OK),
            "BAD_REQUEST" | "VALIDATION_ERROR" => Some(axum::http::StatusCode::BAD_REQUEST),
            "UNAUTHORIZED" => Some(axum::http::StatusCode::UNAUTHORIZED),
            "FORBIDDEN" => Some(axum::http::StatusCode::FORBIDDEN),
            "NOT_FOUND" => Some(axum::http::StatusCode::NOT_FOUND),
            "CONFLICT" => Some(axum::http::StatusCode::CONFLICT),
            "RATE_LIMIT_EXCEEDED" => Some(axum::http::StatusCode::TOO_MANY_REQUESTS),
            "INTERNAL_SERVER_ERROR" => Some(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
            "SERVICE_UNAVAILABLE" => Some(axum::http::StatusCode::SERVICE_UNAVAILABLE),
            "GATEWAY_TIMEOUT" => Some(axum::http::StatusCode::GATEWAY_TIMEOUT),
            _ => None,
        }
    }

    /// 检查是否为客户端错误
    pub fn is_client_error(&self) -> bool {
        matches!(
            self.code.as_str(),
            "BAD_REQUEST"
                | "VALIDATION_ERROR"
                | "UNAUTHORIZED"
                | "FORBIDDEN"
                | "NOT_FOUND"
                | "CONFLICT"
                | "RATE_LIMIT_EXCEEDED"
        )
    }

    /// 检查是否为服务器错误
    pub fn is_server_error(&self) -> bool {
        matches!(
            self.code.as_str(),
            "INTERNAL_SERVER_ERROR" | "SERVICE_UNAVAILABLE" | "GATEWAY_TIMEOUT"
        )
    }

    /// 获取错误类型
    pub fn error_type(&self) -> ErrorType {
        match self.code.as_str() {
            "BAD_REQUEST" | "VALIDATION_ERROR" => ErrorType::Client,
            "UNAUTHORIZED" | "FORBIDDEN" => ErrorType::Security,
            "NOT_FOUND" => ErrorType::Resource,
            "CONFLICT" => ErrorType::Conflict,
            "RATE_LIMIT_EXCEEDED" => ErrorType::RateLimit,
            "INTERNAL_SERVER_ERROR" => ErrorType::Server,
            "SERVICE_UNAVAILABLE" => ErrorType::Service,
            "GATEWAY_TIMEOUT" => ErrorType::Timeout,
            _ => ErrorType::Unknown,
        }
    }
}

/// 错误类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub enum ErrorType {
    Client = 1,
    Security = 2,
    Resource = 3,
    Conflict = 4,
    RateLimit = 5,
    Server = 6,
    Service = 7,
    Timeout = 8,
    Unknown = 9,
}

impl std::fmt::Display for ErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorType::Client => write!(f, "CLIENT"),
            ErrorType::Security => write!(f, "SECURITY"),
            ErrorType::Resource => write!(f, "RESOURCE"),
            ErrorType::Conflict => write!(f, "CONFLICT"),
            ErrorType::RateLimit => write!(f, "RATE_LIMIT"),
            ErrorType::Server => write!(f, "SERVER"),
            ErrorType::Service => write!(f, "SERVICE"),
            ErrorType::Timeout => write!(f, "TIMEOUT"),
            ErrorType::Unknown => write!(f, "UNKNOWN"),
        }
    }
}

pub fn convert_validate_errors_to_details(errors: ValidationErrors) -> Vec<ValidationErrorDetail> {
    errors
        .field_errors()
        .iter()
        .flat_map(|(field, errors)| {
            errors.iter().map(move |error| ValidationErrorDetail {
                field: field.to_string(),
                message: error
                    .message
                    .as_ref()
                    .map(|m| m.to_string())
                    .unwrap_or_else(|| error.code.to_string()),
                code: error.code.to_string(),
                params: if error.params.is_empty() {
                    None
                } else {
                    Some(
                        error
                            .params
                            .iter()
                            .map(|(k, v)| (k.to_string(), v.to_string()))
                            .collect(),
                    )
                },
            })
        })
        .collect()
}
