use crate::domain::response::common::ApiResponse;
use crate::state::AppState;
use axum::Json;
use axum::extract::{MatchedPath, Request, State};
use axum::response::IntoResponse;
use chrono::Utc;
use metrics::{counter, histogram};
use rand::random;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use axum::http::StatusCode;
use axum::middleware::Next;
use defer::defer;
use tokio::time::Instant;

use uuid::Uuid;
use prometheus::{
    Encoder, TextEncoder, Registry,
    Counter, Opts, Gauge, Histogram, HistogramOpts,
    labels, IntCounter, IntGauge,
};

pub async fn health_check() -> impl IntoResponse {
    let start_time = Instant::now();

    counter!("user_client_health_check_total").increment(1);
    let checks = vec![
        ("database", true),
        ("redis", true),
        ("external_service", random()),
    ];

    let all_healthy = checks.iter().all(|(_, status)| *status);

    let duration = start_time.elapsed();

    histogram!("user_client_health_check_duration_seconds").record(duration.as_secs_f64());
    let status = if all_healthy { "healthy" } else { "unhealthy" };

    Json(ApiResponse {
        success: true,
        code: 0,
        data: Some(status),
        error: if !all_healthy {
            Some("unhealthy".to_string())
        } else {
            None
        },
        trace_id: Some(Uuid::new_v4().to_string()),
        request_id: Some(Uuid::new_v4().to_string()),
        timestamp: Some(Utc::now().to_rfc3339()),
    })
}

// 获取统计信息（带状态管理）
pub async fn get_stats(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let stats = serde_json::json!({
        "total_requests": state.get_request_count(),
        "total_errors": state.get_error_count(),
        "uptime_seconds": state.get_uptime(),
        "error_rate": if state.get_request_count() > 0 {
            (state.get_error_count() as f64 / state.get_request_count() as f64) * 100.0
        } else {
            0.0
        },
        "timestamp": Utc::now().to_rfc3339(),
    });

    Json(ApiResponse {
        code: 0,
        success: true,
        data: Some(stats),
        error: None,
        trace_id: Some(Uuid::new_v4().to_string()),
        request_id: Some(Uuid::new_v4().to_string()),
        timestamp: Some(Utc::now().to_rfc3339()),
    })
}

