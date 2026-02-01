use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;
// src/web/response/api_response.rs
use crate::error::AppError;
use crate::web::pagination::{PaginatedData, PaginationInfo};
use crate::web::response::api_error::{ApiError, convert_validate_errors_to_details};
use axum::http::HeaderValue;
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use utoipa::ToSchema;
use validator::ValidationErrors;

/// API 响应结构
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ApiResponse<T = ()> {
    /// 是否成功
    pub success: bool,
    /// 响应码
    pub code: String,
    /// 响应消息
    pub message: String,
    /// 响应数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    /// 错误详情
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ApiError>,
    /// 分页信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<PaginationInfo>,
    /// 时间戳
    pub timestamp: DateTime<Utc>,
    /// 跟踪ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,
    /// 请求ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// API版本
    pub version: String,
    /// 元数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Value>,
}
/// 分页信息
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Pagination {
    /// 当前页码
    pub page: u64,

    /// 每页数量
    pub page_size: u64,

    /// 总记录数
    pub total: u64,

    /// 总页数
    pub total_pages: u64,

    /// 是否有上一页
    pub has_previous: bool,

    /// 是否有下一页
    pub has_next: bool,

    /// 上一页页码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_page: Option<u64>,

    /// 下一页页码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page: Option<u64>,

    /// 分页链接
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<PaginationLinks>,
}

/// 分页链接
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PaginationLinks {
    /// 首页链接
    pub first: Option<String>,

    /// 上一页链接
    pub prev: Option<String>,

    /// 当前页链接
    pub current: String,

    /// 下一页链接
    pub next: Option<String>,

    /// 末页链接
    pub last: Option<String>,
}

impl<T> ApiResponse<T> {
    /// 创建成功响应
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            code: "200".to_string(),
            message: "操作成功".to_string(),
            data: Some(data),
            error: None,
            pagination: None,
            timestamp: Utc::now(),
            request_id: None,
            trace_id: None,
            version: env!("CARGO_PKG_VERSION").to_string(),
            meta: None,
        }
    }

    /// 创建成功响应（无数据）
    pub fn success_empty() -> Self
    where
        T: Default,
    {
        Self {
            success: true,
            code: "204".to_string(),
            message: "没有数据".to_string(),
            data: None,
            error: None,
            pagination: None,
            timestamp: Utc::now(),
            request_id: None,
            trace_id: None,
            version: env!("CARGO_PKG_VERSION").to_string(),
            meta: None,
        }
    }

    /// 创建成功响应（自定义消息）
    pub fn success_with_message(data: T, message: impl Into<String>) -> Self {
        Self {
            success: true,
            code: "200".to_string(),
            message: message.into(),
            data: Some(data),
            error: None,
            pagination: None,
            timestamp: Utc::now(),
            request_id: None,
            trace_id: None,
            version: env!("CARGO_PKG_VERSION").to_string(),
            meta: None,
        }
    }

    pub fn created(data: T) -> Self {
        Self {
            success: true,
            code: "201".to_string(),
            message: "created".to_string(),
            data: Some(data),
            error: None,
            pagination: None,
            timestamp: Utc::now(),
            request_id: None,
            trace_id: None,
            version: env!("CARGO_PKG_VERSION").to_string(),
            meta: None,
        }
    }
    /// 创建分页响应
    pub fn paginated(data: PaginatedData<T>) -> Self {
        let pagination = PaginationInfo {
            page: data.page,
            page_size: data.page_size,
            total: data.total,
            total_pages: data.total_pages,
            has_previous: data.has_previous,
            has_next: data.has_next,
            previous_page: data.previous_page,
            next_page: data.next_page,
            links: None,
        };

        Self {
            success: true,
            code: "200".to_string(),
            message: "查询成功".to_string(),
            data: Some(data.items),
            error: None,
            pagination: Some(pagination),
            timestamp: Utc::now(),
            request_id: None,
            trace_id: None,
            version: env!("CARGO_PKG_VERSION").to_string(),
            meta: None,
        }
    }

    /// 创建分页响应（自定义消息）
    pub fn paginated_with_message(data: PaginatedData<T>, message: impl Into<String>) -> Self {
        let mut response = Self::paginated(data);
        response.message = message.into();
        response
    }

    /// 创建错误响应
    pub fn error(status: StatusCode, error: ApiError) -> Self
    where
        T: Default,
    {
        let code = status.as_u16().to_string();
        Self {
            success: false,
            code,
            message: error.message.clone(),
            data: None,
            error: Some(error),
            pagination: None,
            timestamp: Utc::now(),
            request_id: None,
            trace_id: None,
            version: env!("CARGO_PKG_VERSION").to_string(),
            meta: None,
        }
    }
    /// 从AppError创建错误响应
    pub fn from_app_error(error: AppError) -> Self
    where
        T: Default,
    {
        let status = error.status_code();
        let api_error = ApiError {
            code: error.error_code().to_string(),
            message: error.user_message(),
            details: Some(error.to_string()),
            request_id: None,
            timestamp: Utc::now(),
            validation_errors: None,
            stack_trace: None,
            documentation_url: None,
            suggestion: None,
            original_error: None,
        };
        Self::error(status, api_error)
    }
    /// 从验证错误创建响应
    pub fn from_validation_errors(errors: ValidationErrors) -> Self
    where
        T: Default,
    {
        let validation_errors = convert_validate_errors_to_details(errors);

        let api_error = ApiError {
            code: "VALIDATION_ERROR".to_string(),
            message: "请求参数验证失败".to_string(),
            details: Some("请检查输入参数".to_string()),
            request_id: None,
            timestamp: Utc::now(),
            validation_errors: Some(validation_errors),
            stack_trace: None,
            documentation_url: None,
            suggestion: None,
            original_error: None,
        };
        Self::error(StatusCode::BAD_REQUEST, api_error)
    }
    /// 设置请求ID
    pub fn with_request_id(mut self, request_id: impl Into<String>) -> Self {
        self.request_id = Some(request_id.into());
        self
    }
    /// 设置元数据
    pub fn with_meta(mut self, meta: serde_json::Value) -> Self {
        self.meta = Some(meta);
        self
    }
    /// 设置分页链接
    pub fn with_pagination_links(mut self, links: PaginationLinks) -> Self {
        // 检查 self.pagination 是否为Some
        // 如果是Some，则将内部的值解包并赋值给 pagination 变量
        // 如果是 None，则跳过代码块（不执行任何操作）
        //
        // 为什么使用 &mut？
        // &mut self.pagination 获取 pagination 字段的可变引用
        // 这样可以在不拥有所有权的情况下修改 pagination 内部的 links 字段
        // 避免克隆整个 pagination 结构体
        if let Some(pagination) = &mut self.pagination {
            pagination.links = Some(links);
        }
        self
    }

    /// 检查是否成功
    pub fn is_success(&self) -> bool {
        self.success
    }

    /// 获取数据（如果有）
    pub fn data(&self) -> Option<&T> {
        self.data.as_ref()
    }

    /// 获取错误（如果有）
    pub fn get_error(&self) -> Option<&ApiError> {
        self.error.as_ref()
    }
}

impl<T: Serialize> IntoResponse for ApiResponse<T>
where
    T: Serialize + Send + Sync + 'static,
{
    fn into_response(self) -> Response {
        let status = if self.success {
            StatusCode::OK
        } else {
            // 从错误码解析状态码
            if let Some(ref error) = self.error {
                if let Ok(status_code) = error.code.parse::<u16>() {
                    StatusCode::from_u16(status_code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
                } else {
                    match error.code.as_str() {
                        "VALIDATION_ERROR" => StatusCode::BAD_REQUEST,
                        "AUTHENTICATION_ERROR" => StatusCode::UNAUTHORIZED,
                        "AUTHORIZATION_ERROR" => StatusCode::FORBIDDEN,
                        "NOT_FOUND" => StatusCode::NOT_FOUND,
                        "ALREADY_EXISTS" => StatusCode::CONFLICT,
                        "RATE_LIMIT_EXCEEDED" => StatusCode::TOO_MANY_REQUESTS,
                        _ => StatusCode::INTERNAL_SERVER_ERROR,
                    }
                }
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };
        // 构建响应头
        let mut headers = HashMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert(
            "X-API-Version",
            HeaderValue::from_static(env!("CARGO_PKG_VERSION")),
        );
        headers.insert(
            "X-Request-ID",
            HeaderValue::from_str(self.request_id.as_deref().unwrap_or("unknown")).unwrap(),
        );

        if let Some(pagination) = &self.pagination {
            // 添加分页相关的头部
            headers.insert("X-Page", HeaderValue::from(pagination.page));
            headers.insert("X-Page-Size", HeaderValue::from(pagination.page_size));
            headers.insert("X-Total", HeaderValue::from(pagination.total));
            headers.insert("X-Total-Pages", HeaderValue::from(pagination.total_pages));

            // 添加分页链接头部
            if let Some(links) = &pagination.links {
                if let Some(first) = &links.first {
                    headers.insert(
                        "Link",
                        HeaderValue::from_str(&format!(r#"<{}>; rel="first""#, first)).unwrap(),
                    );
                }

                if let Some(prev) = &links.prev {
                    headers.insert(
                        "Link",
                        HeaderValue::from_str(&format!(r#"<{}>; rel="prev""#, prev)).unwrap(),
                    );
                }

                if let Some(next) = &links.next {
                    headers.insert(
                        "Link",
                        HeaderValue::from_str(&format!(r#"<{}>; rel="next""#, next)).unwrap(),
                    );
                }

                if let Some(last) = &links.last {
                    headers.insert(
                        "Link",
                        HeaderValue::from_str(&format!(r#"<{}>; rel="last""#, last)).unwrap(),
                    );
                }
            }
        }

        (status, Json(self)).into_response()
    }
}

impl<T> From<T> for ApiResponse<T> {
    fn from(data: T) -> Self {
        Self::success(data)
    }
}

impl<T> fmt::Display for ApiResponse<T>
where
    T: Serialize,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.success {
            write!(f, "ApiResponse(success: true, message: {})", self.message)
        } else {
            write!(f, "ApiResponse(success: false, error: {:?})", self.error)
        }
    }
}
