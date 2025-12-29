use std::sync::Arc;
use std::sync::atomic::Ordering;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use prometheus::{Encoder, TextEncoder};
use crate::state::AppState;

// 自定义指标端点
pub async fn custom_metrics_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    // 递增自定义计数器
    state.metrics.custom_counter.inc();

    // 获取所有指标
    let encoder = TextEncoder::new();
    let metric_families = state.metrics.registry.gather();
    let mut buffer = vec![];
    encoder.encode(&metric_families, &mut buffer).unwrap();

    (
        StatusCode::OK,
        [(axum::http::header::CONTENT_TYPE, "text/plain; version=0.0.4")],
        buffer,
    )
}


// Prometheus 标准格式端点
async fn prometheus_metrics(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let encoder = TextEncoder::new();
    let metric_families = state.metrics.registry.gather();
    let mut buffer = vec![];
    encoder.encode(&metric_families, &mut buffer).unwrap();

    (
        StatusCode::OK,
        [(axum::http::header::CONTENT_TYPE, "text/plain; version=0.0.4")],
        buffer,
    )
}


// 业务端点
pub async fn api_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let  counter = state.request_counter.fetch_add(1, Ordering::Relaxed)+1;
    format!("API 调用次数: {}", counter)
}