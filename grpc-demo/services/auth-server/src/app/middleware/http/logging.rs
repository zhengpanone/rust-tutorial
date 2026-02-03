use crate::app::{middleware::http::auth::extractor::AuthUser, state::AppState};

use axum::{
    body::Body,
    extract::{Request, State},
    http::{HeaderMap, StatusCode, Uri, request::Parts},
    middleware::Next,
    response::Response,
};
use chrono::{DateTime, Utc};
use common::security::jwt::claim::JwtClaim;
use common::web::http_method::HttpMethod;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::{
    sync::Arc,
    time::{Duration, Instant},
};
use tracing::{Span, debug, error, info, warn};
use tracing::{Level, Span, debug, error, info, warn};
use utoipa::openapi::security::Http;
use uuid::Uuid;

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
    pub excluded_methods: Vec<HttpMethod>,

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
            excluded_methods: vec![HttpMethod::OPTIONS],
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
    // 获取
    let config = get_logger_config(&state);
    // 如果未启用，则跳过
    if !config.enabled {
        return next.run(request).await;
    }

    // 检查是否应该跳过日志记录
    if should_skip_loggin(&request, &config) {
        return next.run(request).await;
    }
    // 采样率检查
    if !should_sample(&config) {
        return next.run(request).await;
    }
    // 记录开始时间
    let start_time = Instant::now();
    let request_time = Utc::now();

    // 生成请求ID
    let request_id = Uuid::new_v4().to_string();

    let (parts, body) = request.into_parts();
    let method = HttpMethod::from(parts.method.clone());
    let uri = parts.uri.clone();
    let headers = parts.headers.clone();

    // 获取客户端IP
    let client_ip = extract_client_ip(&headers, &parts);

    // 获取用户ID
    let user_id = extract_user_id(&parts);

    let request_log = RequestLog::new(
        &request_id,
        &method,
        &uri,
        &client_ip,
        &user_id,
        &headers,
        &config,
    );
    request_log.log_start();

    // 记录请求体
    let (body, request_body) = if config.log_request_body {
        extract_request_body(body, &config).await
    } else {
        (Body::empty(), None)
    };

    // 重新构建请求
    let mut request = Request::from_parts(parts, body);

    // 添加请求ID到扩展
    request.extensions_mut().insert(request_id.clone());
    request.extensions_mut().insert(start_time);
    request.extensions_mut().insert(request_time);

    // 处理请求
    let response = next.run(request).await;

    // 记录结束时间
    let duration = start_time.elapsed();

    // 提取响应信息
    let (parts, body) = response.into_parts();
    let status = parts.status;
    let response_headers = parts.headers.clone();

    // 记录响应体
    let (body, response_body) = if config.log_response_body {
        extract_response_body(body, &config).await
    } else {
        (Body::empty(), None)
    };
    // 重新构建响应
    let mut response = Response::from_parts(parts, body);

    // 添加响应头
    response
        .headers_mut()
        .insert("X-Request-ID", request_id.parse().unwrap());

    // 记录请求完成
    let response_log = ResponseLog::new(
        &request_id,
        status,
        &headers,
        response_body.as_deref(),
        duration,
        &config,
    );
    response_log.log_complete(&method, &uri, &client_ip, request_body.as_deref());

    // 记录慢记录
    if duration.as_millis() > config.slow_request_threshold_ms as u128 {
        log_slow_request(&request_id, &method, &uri, duration, &config);
    }

    // TODO
    response
}

/// 请求日志结构
#[derive(Debug, Clone)]
pub struct RequestLog {
    pub request_id: String,
    method: HttpMethod,
    uri: Uri,
    client_ip: String,
    user_id: Option<String>,
    user_agent: Option<String>,
    content_type: Option<String>,
    content_length: Option<u64>,
    headers: Vec<(String, String)>,
    timestamp: DateTime<Utc>,
}

impl RequestLog {
    fn new(
        request_id: &str,
        method: &HttpMethod,
        uri: &Uri,
        client_ip: &str,
        user_id: &Option<String>,
        headers: &HeaderMap,
        config: &RequestLoggerConfig,
    ) -> Self {
        let user_agent = headers
            .get("user-agent")
            .and_then(|h| h.to_str().ok())
            .map(|s| s.to_string());

        let content_type = headers
            .get("content-type")
            .and_then(|h| h.to_str().ok())
            .map(|s| s.to_string());

        let content_length = headers
            .get("content-length")
            .and_then(|h| h.to_str().ok())
            .and_then(|s| s.parse().ok());
        let mut logged_headers = Vec::new();
        for header_name in &config.log_headers {
            if let Some(value) = headers.get(header_name) {
                if let Ok(value_str) = value.to_str() {
                    // 处理敏感头信息
                    let logged_value =
                        if header_name == "authorization" && !config.log_sensitive_info {
                            mask_sensitive_value(value_str)
                        } else {
                            value_str.to_string()
                        };
                    logged_headers.push((header_name.clone(), logged_value));
                }
            }
        }
        Self {
            request_id: request_id.to_string(),
            method: method.clone(),
            uri: uri.clone(),
            client_ip: client_ip.to_string(),
            user_id: user_id.clone(),
            user_agent,
            content_type,
            content_length,
            headers: logged_headers,
            timestamp: Utc::now(),
        }
    }

    fn log_start(&self) {
        let span = Span::current();
        span.record("request_id", &self.request_id);
        span.record("method", &self.method.as_str());
        span.record("uri", &self.uri.to_string());
        span.record("client_ip", &self.client_ip);

        if let Some(user_id) = &self.user_id {
            span.record("user_id", &user_id);
        }
        debug!(request_id = %self.request_id,
            method = %self.method,
            uri = %self.uri,
            client_ip = %self.client_ip,
            user_id = ?self.user_id,
            user_agent = ?self.user_agent,
            content_type = ?self.content_type,
            content_length = ?self.content_length,
            headers = ?self.headers,
            "Incoming request"
        );
    }
}

#[derive(Debug, Clone)]
pub struct ResponseLog {
    pub request_id: String,
    pub status: StatusCode,
    pub content_type: Option<String>,
    pub content_length: Option<u64>,
    pub headers: Vec<(String, String)>,
    pub duration: Duration,
    pub timestamp: DateTime<Utc>,
}

impl ResponseLog {
    fn new(
        request_id: &str,
        status: StatusCode,
        headers: &HeaderMap,
        response_body: Option<&str>,
        duration: Duration,
        config: &RequestLoggerConfig,
    ) -> Self {
        let content_type = headers
            .get("content-type")
            .and_then(|h| h.to_str().ok())
            .map(|s| s.to_string());

        let content_length = if let Some(body) = response_body {
            Some(body.len() as u64)
        } else {
            headers
                .get("content-length")
                .and_then(|h| h.to_str().ok())
                .map(|s| s.parse::<u64>().unwrap_or(0))
        };

        let mut logged_headers = Vec::new();
        for header_name in &config.log_headers {
            if let Some(value) = headers.get(header_name) {
                if let Ok(value_str) = value.to_str() {
                    logged_headers.push((header_name.clone(), value_str.to_string()));
                }
            }
        }

        Self {
            request_id: request_id.to_string(),
            status,
            content_type,
            content_length,
            headers: logged_headers,
            duration,
            timestamp: Utc::now(),
        }
    }

    fn log_complete(
        &self,
        method: &HttpMethod,
        uri: &Uri,
        client_ip: &str,
        request_body: Option<&str>,
    ) {
        let status_class = self.status.as_u16() / 100;
        let duration_ms = self.duration.as_millis();
        let duration_secs = self.duration.as_secs_f64();

        // 运行期等级 —— 只给 event 用
        let log_level = match status_class {
            5 => tracing::Level::ERROR,
            4 => tracing::Level::WARN,
            _ => tracing::Level::INFO,
        };

        // Span：必须是编译期常量 level
        let span = tracing::span!(
            target: "http_request",
            tracing::Level::INFO,
            "http_request",
            request_id = %self.request_id,
            method = %method,
            uri = %uri,
            status = %self.status,
            duration_ms,
            duration_secs,
            client_ip = %client_ip,
            content_type = ?self.content_type,
            content_length = ?self.content_length,
            request_body = ?request_body.map(|b| {
                if b.len() > 100 {
                    format!("{}...", &b[..100])
                } else {
                    b.to_string()
                }
            }),
        );

        span.in_scope(|| {
            match log_level {
                tracing::Level::ERROR => {
                    error!("Request completed with error");
                }
                tracing::Level::WARN => {
                    warn!("Request completed with warning");
                }
                _ => {
                    info!("Request completed successfully");
                }
            }

            debug!(
                headers = ?self.headers,
                "Response sent"
            );
        });
    }
}

/// 获取日志配置
fn get_logger_config(state: &Arc<AppState>) -> RequestLoggerConfig {
    // TODO 从配置或状态获取
    RequestLoggerConfig::default()
}

/// 检查是否应该跳过日志
fn should_skip_loggin(request: &Request, config: &RequestLoggerConfig) -> bool {
    let path = request.uri().path();
    let method = request.method();

    let http_method = HttpMethod::from(method.clone());

    // 检查排除的路径
    for excluded_path in &config.excluded_paths {
        if path.starts_with(excluded_path) {
            return true;
        }
    }

    // 检查排除的方法
    if config.excluded_methods.contains(&http_method) {
        return true;
    }
    false
}

/// 采样率检查
fn should_sample(config: &RequestLoggerConfig) -> bool {
    if config.sample_rate >= 1.0 {
        return true;
    }
    let mut rng = rand::rng();
    rng.random_bool(config.sample_rate)
}

/// 提取客户端IP
fn extract_client_ip(headers: &HeaderMap, parts: &Parts) -> String {
    // 1. 检查X-Forwarded-For
    if let Some(forward_for) = headers.get("x-forwarded-for") {
        if let Ok(forword_for_str) = forward_for.to_str() {
            // 取第一个IP
            if let Some(first_ip) = forword_for_str.split(",").next() {
                return first_ip.trim().to_string();
            }
        }
    }
    // 2. 检查X-Real-IP
    if let Some(real_ip) = headers.get("x-real-ip") {
        if let Ok(real_ip_str) = real_ip.to_str() {
            return real_ip_str.to_string();
        }
    }

    // 3. 从扩展中获取
    if let Some(remote_addr) = parts.extensions.get::<std::net::SocketAddr>() {
        return remote_addr.ip().to_string();
    }
    // 默认值
    "unknown".to_string()
}

/// 提取用户ID
fn extract_user_id(parts: &Parts) -> Option<String> {
    // 从扩展中获取认证用户
    if let Some(auth_user) = parts.extensions.get::<AuthUser>() {
        return Some(auth_user.user_id.to_string());
    }
    // 从JWT声明中获取
    if let Some(claims) = parts.extensions.get::<JwtClaim>() {
        return Some(claims.sub.to_string());
    }
    None
}

fn mask_sensitive_value(value: &str) -> String {
    if value.len() <= 10 {
        return "***".to_string();
    }
    let prefix = &value[..5];
    let suffix = &value[value.len() - 5..];
    format!("{}...{}", prefix, suffix)
}

async fn extract_request_body(body: Body, config: &RequestLoggerConfig) -> (Body, Option<String>) {
    let bytes = axum::body::to_bytes(body, config.request_body_limit)
        .await
        .ok();
    if let Some(bytes) = bytes {
        let body_str = String::from_utf8_lossy(&bytes).to_string();

        // 检查内容类型,只记录特定类型的body
        let should_log = !body_str.is_empty() && body_str.len() <= config.request_body_limit;

        if should_log {
            // 重新创建Body
            let new_body = Body::from(bytes.clone());
            return (new_body, Some(body_str));
        }
    }
    (Body::empty(), None)
}
/// 提取响应体
async fn extract_response_body(body: Body, config: &RequestLoggerConfig) -> (Body, Option<String>) {
    let bytes = axum::body::to_bytes(body, config.response_body_limit)
        .await
        .ok();
    if let Some(bytes) = bytes {
        let body_str = String::from_utf8_lossy(&bytes).to_string();

        // 检查内容类型
        let should_log = !body_str.is_empty() && body_str.len() <= config.response_body_limit;

        if should_log {
            // 重新创建Body
            let new_body = Body::from(bytes.clone());
            return (new_body, Some(body_str));
        }
    }
    (Body::empty(), None)
}

/// 记录慢请求
fn log_slow_request(
    request_id: &str,
    method: &HttpMethod,
    uri: &Uri,
    duration: Duration,
    config: &RequestLoggerConfig,
) {
    let duration_ms = duration.as_millis();

    warn!(
        request_id=%request_id,
        method=%method,
        uri = %uri,
        duration_ms=%duration_ms,
        threshold_ms = config.slow_request_threshold_ms,
        "Slow request detected"
    );
}

/// 记录指标
fn record_metrics(
    state: &AppState,
    method: &HttpMethod,
    uri: &Uri,
    status: StatusCode,
    duration: Duration,
) {
    // 记录请求统计
    state.record_request(
        uri.path(),
        method.as_str(),
        duration.as_millis() as u64,
        status.is_success(),
    );
    // TODO
}

fn record_audit_log(
    state: &AppState,
    request_log: &RequestLog,
    response_log: &ResponseLog,
    request_body: Option<&str>,
) {
    // 记录到审计日志表
    // TODO
}
