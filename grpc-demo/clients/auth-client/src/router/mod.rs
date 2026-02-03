use crate::domain::response::common::ApiResponse;
use crate::handler::health_handler::{get_stats, health_check};
use crate::handler::metrics_handler::{api_handler, custom_metrics_handler};
use crate::state::AppState;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{middleware, Json, Router};
use metrics::{counter};
use std::sync::Arc;
use tokio::time::{sleep, Instant};
use uuid::Uuid;
use crate::middleware::charset_middleware::charset_utf8_middleware;
use crate::middleware::metrics_middleware::metrics_middleware;

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/metrics", get(custom_metrics_handler))
        .route("/metrics/api_handler", get(api_handler))
        .route("/", get(root_handler))
        .route("/api/v1/stats", get(get_stats))
        .layer(middleware::from_fn(charset_utf8_middleware))
        .layer(middleware::from_fn_with_state(Arc::clone(&app_state),metrics_middleware))
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
// // 获取用户信息
// async fn get_user(
//     Path(user_id): Path<String>,
// ) -> impls IntoResponse {
//     let start_time = Instant::now();
//
//     info!("📥 GET /api/v1/users/{}", user_id);
//
//     // 记录请求
//     counter!("user_service_requests_total",
//              "method" => "GET",
//              "endpoint" => "/api/v1/users/{id}",
//              "user_id" => &user_id)
//         .increment(1);
//
//     gauge!("user_service_requests_in_progress",
//            "endpoint" => "/api/v1/users/{id}")
//         .increment(1.0);
//
//     // 模拟处理延迟
//     let delay_ms: u64 = rand::random::<u64>() % 200;
//     sleep(Duration::from_millis(delay_ms)).await;
//
//     // 模拟成功率
//     let success = rand::random::<f32>() > 0.1; // 90% 成功率
//
//     let result = if success {
//         // 模拟成功的响应
//         let response = UserResponse {
//             id: user_id.clone(),
//             name: format!("User {}", user_id),
//             email: format!("user{}@example.com", user_id),
//             created_at: chrono::Utc::now().to_rfc3339(),
//             error_message: None,
//         };
//
//         Ok(Json(ApiResponse {
//             success: true,
//             data: Some(response),
//             error: None,
//             timestamp: chrono::Utc::now().to_rfc3339(),
//         }))
//     } else {
//         // 模拟失败的响应
//         Err((StatusCode::NOT_FOUND, Json(ApiResponse::<()> {
//             success: false,
//             data: None,
//             error: Some(format!("User {} not found", user_id)),
//             timestamp: chrono::Utc::now().to_rfc3339(),
//         })))
//     };
//
//     let duration = start_time.elapsed();
//     gauge!("user_service_requests_in_progress",
//            "endpoint" => "/api/v1/users/{id}")
//         .decrement(1.0);
//
//     // 记录指标
//     let status_label = if success { "success" } else { "error" };
//     histogram!("user_service_request_duration_seconds",
//                "method" => "GET",
//                "endpoint" => "/api/v1/users/{id}",
//                "status" => status_label)
//         .record(duration.as_secs_f64());
//
//     counter!("user_service_request_duration_milliseconds",
//              "method" => "GET",
//              "endpoint" => "/api/v1/users/{id}",
//              "status" => status_label)
//         .increment(duration.as_millis() as u64);
//
//     // 记录结果
//     if success {
//         info!("✅ GET /api/v1/users/{} succeeded in {:?}", user_id, duration);
//     } else {
//         warn!("⚠️  GET /api/v1/users/{} failed in {:?}", user_id, duration);
//     }
//
//     result
// }
//
// // 创建用户
// async fn create_user(
//     Json(payload): Json<UserRequest>,
// ) -> impls IntoResponse {
//     let start_time = Instant::now();
//
//     info!("📝 POST /api/v1/users with ID: {}", payload.id);
//
//     // 记录请求
//     counter!("user_service_requests_total",
//              "method" => "POST",
//              "endpoint" => "/api/v1/users")
//         .increment(1);
//
//     gauge!("user_service_requests_in_progress",
//            "endpoint" => "/api/v1/users")
//         .increment(1.0);
//
//     // 验证输入
//     if payload.id.is_empty() {
//         gauge!("user_service_requests_in_progress",
//                "endpoint" => "/api/v1/users")
//             .decrement(1.0);
//
//         let duration = start_time.elapsed();
//         histogram!("user_service_request_duration_seconds",
//                    "method" => "POST",
//                    "endpoint" => "/api/v1/users",
//                    "status" => "error")
//             .record(duration.as_secs_f64());
//
//         return Err((StatusCode::BAD_REQUEST, Json(ApiResponse::<()> {
//             success: false,
//             data: None,
//             error: Some("User ID cannot be empty".to_string()),
//             timestamp: chrono::Utc::now().to_rfc3339(),
//         })));
//     }
//
//     // 模拟处理延迟
//     let delay_ms: u64 = rand::random::<u64>() % 300;
//     sleep(Duration::from_millis(delay_ms)).await;
//
//     let response = UserResponse {
//         id: payload.id.clone(),
//         name: format!("Created User {}", payload.id),
//         email: format!("created{}@example.com", payload.id),
//         created_at: chrono::Utc::now().to_rfc3339(),
//     };
//
//     let duration = start_time.elapsed();
//     gauge!("user_service_requests_in_progress",
//            "endpoint" => "/api/v1/users")
//         .decrement(1.0);
//
//     // 记录指标
//     histogram!("user_service_request_duration_seconds",
//                "method" => "POST",
//                "endpoint" => "/api/v1/users",
//                "status" => "success")
//         .record(duration.as_secs_f64());
//
//     info!("✅ POST /api/v1/users succeeded in {:?}", duration);
//
//     Ok((StatusCode::CREATED, Json(ApiResponse {
//         success: true,
//         data: Some(response),
//         error: None,
//         timestamp: chrono::Utc::now().to_rfc3339(),
//     })))
// }
//
// // 更新用户
// async fn update_user(
//     Path(user_id): Path<String>,
//     Json(payload): Json<UserRequest>,
// ) -> impls IntoResponse {
//     let start_time = Instant::now();
//
//     info!("🔄 POST /api/v1/users/{} (update)", user_id);
//
//     counter!("user_service_requests_total",
//              "method" => "POST",
//              "endpoint" => "/api/v1/users/{id}")
//         .increment(1);
//
//     let delay_ms: u64 = rand::random::<u64>() % 250;
//     sleep(Duration::from_millis(delay_ms)).await;
//
//     let response = UserResponse {
//         id: user_id,
//         name: format!("Updated User {}", payload.id),
//         email: format!("updated{}@example.com", payload.id),
//         created_at: chrono::Utc::now().to_rfc3339(),
//     };
//
//     let duration = start_time.elapsed();
//     histogram!("user_service_request_duration_seconds",
//                "method" => "POST",
//                "endpoint" => "/api/v1/users/{id}")
//         .record(duration.as_secs_f64());
//
//     Ok(Json(ApiResponse {
//         success: true,
//         data: Some(response),
//         error: None,
//         timestamp: chrono::Utc::now().to_rfc3339(),
//     }))
// }
//
// // 删除用户
// async fn delete_user(
//     Path(user_id): Path<String>,
// ) -> impls IntoResponse {
//     let start_time = Instant::now();
//
//     info!("🗑️  DELETE /api/v1/users/{}", user_id);
//
//     counter!("user_service_requests_total",
//              "method" => "DELETE",
//              "endpoint" => "/api/v1/users/{id}")
//         .increment(1);
//
//     let delay_ms: u64 = rand::random::<u64>() % 150;
//     sleep(Duration::from_millis(delay_ms)).await;
//
//     let duration = start_time.elapsed();
//     histogram!("user_service_request_duration_seconds",
//                "method" => "DELETE",
//                "endpoint" => "/api/v1/users/{id}")
//         .record(duration.as_secs_f64());
//
//     Ok(Json(ApiResponse {
//         success: true,
//         data: Some(format!("User {} deleted", user_id)),
//         error: None,
//         timestamp: chrono::Utc::now().to_rfc3339(),
//     }))
// }
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
