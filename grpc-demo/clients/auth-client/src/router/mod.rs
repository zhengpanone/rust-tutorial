mod user_router;

use crate::domain::response::common::ApiResponse;
use crate::handler::health_handler::{get_stats, health_check};
use crate::handler::metrics_handler::{api_handler, custom_metrics_handler};
use crate::middleware::charset_middleware::charset_utf8_middleware;
use crate::middleware::metrics_middleware::metrics_middleware;
use crate::state::AppState;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router, middleware};
use metrics::{counter, histogram};
use std::sync::Arc;
use uuid::Uuid;

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/metrics", get(custom_metrics_handler))
        .route("/metrics/api_handler", get(api_handler))
        .route("/", get(root_handler))
        .route("/api/v1/stats", get(get_stats))
        .layer(middleware::from_fn(charset_utf8_middleware))
        .layer(middleware::from_fn_with_state(
            Arc::clone(&app_state),
            metrics_middleware,
        ))
        // .layer(middleware::from_fn(logging_middleware))
        // .layer(middleware::from_fn(auth_middleware))
        .with_state(Arc::clone(&app_state))
}

async fn root_handler() -> impl IntoResponse {
    Json(ApiResponse {
        code: 0,
        success: true,
        data: Some("User Client Service API v1.0".to_string()),
        error: None,
        trace_id: Some(Uuid::new_v4().to_string()),
        request_id: Some(Uuid::new_v4().to_string()),
        timestamp: Some(chrono::Utc::now().to_rfc3339()),
    })
}

// 就绪检查端点
async fn ready_check() -> impl IntoResponse {
    counter!("user_service_readiness_checks_total").increment(1);

    // 这里可以添加实际的就绪检查逻辑
    let is_ready = true;

    if is_ready {
        (StatusCode::OK, "OK")
    } else {
        (StatusCode::SERVICE_UNAVAILABLE, "Not Ready")
    }
}


//
// // 获取服务信息
// async fn service_info() -> impls IntoResponse {
//     Json(ApiResponse {
//         success: true,
//         data: Some(serde_json::json!({
//             "service": "user-service",
//             "version": env!("CARGO_PKG_VERSION"),
//             "description": "User management HTTP API with Prometheus metrics",
//             "endpoints": [
//                 {"path": "/api/v1/users/{id}", "method": "GET", "description": "Get user by ID"},
//                 {"path": "/api/v1/users", "method": "POST", "description": "Create new user"},
//                 {"path": "/health", "method": "GET", "description": "Health check"},
//                 {"path": "/ready", "method": "GET", "description": "Readiness check"},
//             ],
//             "metrics": "http://localhost:9000/metrics"
//         })),
//         error: None,
//         timestamp: chrono::Utc::now().to_rfc3339(),
//     })
// }
//
// // 获取指标信息
// async fn metrics_info() -> impls IntoResponse {
//     let metrics = vec![
//         MetricsInfo {
//             endpoint: "user_service_requests_total".to_string(),
//             description: "Total number of HTTP requests".to_string(),
//         },
//         MetricsInfo {
//             endpoint: "user_service_request_duration_seconds".to_string(),
//             description: "Request duration in seconds".to_string(),
//         },
//         MetricsInfo {
//             endpoint: "user_service_up".to_string(),
//             description: "Service availability (1=up, 0=down)".to_string(),
//         },
//         MetricsInfo {
//             endpoint: "user_service_uptime_seconds".to_string(),
//             description: "Service uptime in seconds".to_string(),
//         },
//     ];
//
//     Json(ApiResponse {
//         success: true,
//         data: Some(metrics),
//         error: None,
//         timestamp: chrono::Utc::now().to_rfc3339(),
//     })
// }
