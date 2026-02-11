use crate::app::config::cors::CorsConfig;
use axum::extract::Request;
use axum::http;
use axum::http::{HeaderName, HeaderValue, Method, StatusCode};
use axum::middleware::Next;
use axum::response::Response;
use std::sync::Arc;
use std::time::Duration;
use tower_http::cors::{AllowHeaders, AllowMethods, AllowOrigin, CorsLayer, ExposeHeaders};

/// 创建CORS层
pub fn create_cors_layer(config: &CorsConfig) -> CorsLayer {
    let mut cors = CorsLayer::new();

    // 转换允许的来源为HeaderValue
    let allowed_origins: Vec<HeaderValue> = config
        .allowed_origins
        .iter()
        .filter_map(|origin| origin.parse().ok())
        .collect();
    cors = cors.allow_origin(AllowOrigin::list(allowed_origins));

    // 转换允许的方法为Method
    let allowed_methods: Vec<Method> = config
        .allowed_methods
        .iter()
        .filter_map(|method| method.parse().ok())
        .collect();
    cors = cors.allow_methods(AllowMethods::list(allowed_methods));

    // 转换允许的头为HeaderValue
    let allowed_headers: Vec<HeaderName> = config
        .allowed_headers
        .iter()
        .filter_map(|header| header.parse().ok())
        .collect();
    cors = cors.allow_headers(AllowHeaders::list(allowed_headers));

    // 设置暴露的头
    let exposed_headers: Vec<HeaderName> = config
        .exposed_headers
        .iter()
        .map(|header| header.parse().unwrap())
        .collect();
    cors = cors.expose_headers(ExposeHeaders::list(exposed_headers));

    // 设置是否允许凭证
    if config.allow_credentials {
        cors = cors.allow_credentials(true);
    }

    // 设置预检请求缓存时间
    if let Some(max_age) = config.max_age_seconds {
        cors = cors.max_age(Duration::from_secs(max_age));
    }
    cors
}

/// CORS预处理中间件
/// 处理预检请求
pub async fn cors_middleware(
    // 使用Arc共享配置，避免克隆整个配置
    config: Arc<CorsConfig>,
    request: Request,
    next: Next,
) -> Response {
    // 在处理请求前提取Origin头
    let origin_header = request.headers().get("Origin").cloned();
    // 1. 处理预检请求
    if request.method() == Method::OPTIONS {
        return handle_preflight(&config, origin_header);
    }
    // 2. 处理正常请求
    let mut response = next.run(request).await;
    add_cors_headers(&config, origin_header.as_ref(), response.headers_mut());

    response
}

/// 处理预检请求
fn handle_preflight(config: &CorsConfig, origin_header: Option<HeaderValue>) -> Response {
    let mut builder = Response::builder().status(StatusCode::NO_CONTENT);

    // 添加允许的来源
    if let Some(origin) = get_allowed_origin(config, origin_header) {
        builder = builder.header("Access-Control-Allow-Origin", origin);
    }
    // 添加其他CORS头
    builder = builder
        .header(
            "Access-Control-Allow-Methods",
            config.allowed_methods.join(","),
        )
        // 添加请求头
        .header(
            "Access-Control-Allow-Headers",
            config.allowed_headers.join(","),
        )
        .header(
            "Access-Control-Max-Age",
            config.max_age_seconds.unwrap_or(86400).to_string(),
        );
    if config.allow_credentials {
        builder = builder.header("Access-Control-Allow-Credentials", "true");
    }
    builder.body(axum::body::Body::empty()).unwrap()
}

/// 添加CORS头到响应
fn add_cors_headers(
    config: &CorsConfig,
    origin_header: Option<&HeaderValue>,
    headers: &mut http::HeaderMap,
) {
    // 添加Origin头
    if let Some(origin) = get_allowed_origin(config, origin_header.cloned()) {
        headers.insert("Access-Control-Allow-Origin", origin);

        if config.allow_credentials {
            headers.insert(
                "Access-Control-Allow-Credentials",
                HeaderValue::from_static("true"),
            );
        }
    }
    // 添加暴露的头
    if !config.exposed_headers.is_empty() {
        headers.insert(
            "Access-Control-Expose-Headers",
            HeaderValue::from_str(&config.exposed_headers.join(", ")).unwrap(),
        );
    }
}

/// 获取允许的Origin
fn get_allowed_origin(
    config: &CorsConfig,
    origin_header: Option<HeaderValue>,
) -> Option<HeaderValue> {
    origin_header.and_then(|origin| {
        let origin_str = origin.to_str().ok()?;

        // 检查是否在允许列表中
        if config
            .allowed_origins
            .iter()
            .any(|o| o == "*" || o == origin_str)
        {
            // 创建新的HeaderValue，避免借用问题
            HeaderValue::from_str(origin_str).ok()
        } else {
            None
        }
    })
}
