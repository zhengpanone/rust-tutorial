use crate::state::AppState;
use axum::extract::{MatchedPath, Request, State};
use axum::middleware::Next;
use axum::response::IntoResponse;
use defer::defer;
use prometheus::labels;
use std::sync::Arc;
use tokio::time::Instant;

// 指标中间件
pub async fn metrics_middleware(
    State(state): State<Arc<AppState>>,
    request: Request,
    next: Next,
) -> impl IntoResponse {
    // 记录活跃连接数
    state.metrics.active_connections.inc();
    defer!(state.metrics.active_connections.dec());

    // 获取请求路径
    let path = if let Some(matched_path) = request.extensions().get::<MatchedPath>() {
        matched_path.as_str().to_string()
    } else {
        request.uri().path().to_string()
    };

    // 记录请求开始
    let method = request.method().to_string();
    let start_time = Instant::now();

    // 处理请求
    let response = next.run(request).await;

    // 记录持续时间
    let duration = start_time.elapsed();
    state
        .metrics
        .http_request_duration_seconds
        .observe(duration.as_secs_f64());

    // 添加自定义标签
    state
        .metrics
        .http_requests_total
        .with(&labels! {
            "method" => method,
            "path" => path,
            "status" => response.status().as_u16().to_string(),
        })
        .inc();

    response
}
