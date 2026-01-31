use crate::app::state::AppState;
use axum::extract::{Request, State};
use axum::http::Method;
use axum::middleware::Next;
use axum::response::Response;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 请求日志配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestLoggerConfig {
    /// 是否启用请求日志
    pub enabled: bool,

    /// 日志级别
    pub level: LogLevel,

    /// 要记录的请求头
    pub log_headers: Vec<String>,

    /// 要记录的请求体大小限制
    pub request_body_limit: usize,

    /// 要记录的响应体大小限制
    pub response_body_limit: usize,

    /// 慢请求阈值（毫秒）
    pub slow_request_threshold_ms: u64,

    /// 是否记录敏感信息
    pub log_sensitive_info: bool,

    /// 是否记录请求体
    pub log_request_body: bool,

    /// 是否记录响应体
    pub log_response_body: bool,

    /// 排除的路径
    pub excluded_paths: Vec<String>,

    /// 排除的方法
    pub excluded_methods: Vec<Method>,

    /// 采样率（0.0-1.0）
    pub sample_rate: f64,
}

impl Default for RequestLoggerConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            level: LogLevel::Info,
            log_headers: vec![
                "user-agent".to_string(),
                "content-type".to_string(),
                "content-length".to_string(),
                "authorization".to_string(),
                "x-forwarded-for".to_string(),
                "x-real-ip".to_string(),
            ],
            request_body_limit: 1024,        // 1KB
            response_body_limit: 1024,       // 1KB
            slow_request_threshold_ms: 1000, // 1秒
            log_sensitive_info: false,
            log_request_body: false,
            log_response_body: false,
            excluded_paths: vec![
                "/health".to_string(),
                "/ready".to_string(),
                "/live".to_string(),
                "/metrics".to_string(),
            ],
            excluded_methods: vec![Method::OPTIONS],
            sample_rate: 1.0,
        }
    }
}

/// 日志级别
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

/// 请求日志中间件
pub async fn request_logger(
    State(state): State<Arc<AppState>>,
    request: Request,
    next: Next,
) -> Response {
    let config = get_logger_config(&state);
    todo!()
}

/// 获取日志配置
fn get_logger_config(state: &Arc<AppState>) -> RequestLoggerConfig {
    // TODO 从配置或状态获取
    RequestLoggerConfig::default()
}
