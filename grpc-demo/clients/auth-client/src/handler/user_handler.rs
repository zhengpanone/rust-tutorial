use crate::domain::response::common::ApiResponse;
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use chrono::Utc;
use metrics::{counter, gauge, histogram};
use std::sync::Arc;

use crate::state::AppState;
use proto::user::UserResponse;
use std::time::Duration;
use tokio::time::{Instant, sleep};
use tracing::{info, warn};

// 获取用户信息
pub async fn get_user(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<String>,
) -> impl IntoResponse {
    let start_time = Instant::now();

    let mut user_client = state.user_client.lock().await;
    info!("📥 GET /api/v1/users/{}", user_id);

    // 记录请求
    counter!("user_service_requests_total","method" => "GET","endpoint" => "/api/v1/users/{id}","user_id" => user_id.clone())
        .increment(1);

    gauge!("user_service_requests_in_progress",
"endpoint" => "/api/v1/users/{id}")
    .increment(1.0);

    // 模拟处理延迟
    let delay_ms: u64 = rand::random::<u64>() % 200;
    sleep(Duration::from_millis(delay_ms)).await;

    let user = user_client
        .get_user("1".to_string())
        .await
        .expect("get_user response failed");

    info!(
        "user name = {}",
        user.user.ok_or("user not found").unwrap().username
    );
    // 模拟成功率
    let success = rand::random::<f32>() > 0.1; // 90% 成功率

    let result = if success {
        // 模拟成功的响应
        let response = UserResponse {
            user: None,
            error_message: None,
            response: None,
        };

        Ok(Json(ApiResponse {
            code: 0,
            success: true,
            data: Some(response),
            error: None,
            trace_id: None,
            request_id: None,
            timestamp: Some(Utc::now().to_rfc3339()),
        }))
    } else {
        // 模拟失败的响应
        Err((
            StatusCode::NOT_FOUND,
            Json(ApiResponse::<()> {
                code: 0,
                success: false,
                data: None,
                error: Some(format!("User {} not found", user_id)),
                trace_id: None,
                request_id: None,
                timestamp: Some(Utc::now().to_rfc3339()),
            }),
        ))
    };
}
//     let duration = start_time.elapsed();
//     gauge!("user_service_requests_in_progress",
// "endpoint" => "/api/v1/users/{id}")
//     .decrement(1.0);
//
//     // 记录指标
//     let status_label = if success { "success" } else { "error" };
//     histogram!("user_service_request_duration_seconds",
// "method" => "GET",
// "endpoint" => "/api/v1/users/{id}",
// "status" => status_label)
//     .record(duration.as_secs_f64());
//
//     counter!("user_service_request_duration_milliseconds",
// "method" => "GET",
// "endpoint" => "/api/v1/users/{id}",
// "status" => status_label)
//     .increment(duration.as_millis() as u64);
//
//     // 记录结果
//     if success {
//         info!(
//             "✅ GET /api/v1/users/{} succeeded in {:?}",
//             user_id, duration
//         );
//     } else {
//         warn!("⚠️  GET /api/v1/users/{} failed in {:?}", user_id, duration);
//     }
//
//     todo!("TODO: 实现获取用户信息的逻辑")
// }

//
// // 创建用户
// pub async fn create_user(
//     Json(payload): Json<UserRequest>,
// ) -> impl IntoResponse {
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
// ) -> impl IntoResponse {
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
// ) -> impl IntoResponse {
// let start_time = Instant::now();
//
// info! ("🗑️  DELETE /api/v1/users/{}", user_id);
//
// counter ! ("user_service_requests_total",
// "method" => "DELETE",
// "endpoint" => "/api/v1/users/{id}")
// .increment(1);
//
// let delay_ms: u64 = rand::random::< u64 > () % 150;
// sleep(Duration::from_millis(delay_ms)).await;
//
// let duration = start_time.elapsed();
// histogram ! ("user_service_request_duration_seconds",
// "method" => "DELETE",
// "endpoint" => "/api/v1/users/{id}")
// .record(duration.as_secs_f64());
//
// Ok(Json(ApiResponse {
// success: true,
// data: Some(format !("User {} deleted", user_id)),
// error: None,
// timestamp: chrono::Utc::now().to_rfc3339(),
// }))
// }
