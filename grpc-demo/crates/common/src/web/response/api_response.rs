// src/web/response/api_response.rs
use crate::web::response::api_error::ApiError;
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use utoipa::ToSchema;

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
    pub pagination: Option<Pagination>,
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

pub fn paginated(data: PaginatedData<T>)->Self {
// TODO 
}


}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        let status = match self.code {
            200 => StatusCode::OK,
            201 => StatusCode::CREATED,
            204 => StatusCode::NO_CONTENT,
            400 => StatusCode::BAD_REQUEST,
            404 => StatusCode::NOT_FOUND,
            500 => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::OK,
        };
        (status, Json(self)).into_response()
    }
}
