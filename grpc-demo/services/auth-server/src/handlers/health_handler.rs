use crate::state::AppState;
use anyhow::anyhow;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use std::sync::Arc;
use tracing::{debug, error, info};

#[derive(Debug, serde::Serialize)]
struct HealthResponse {
    status: String,
    timestamp: String,
    services: ServicesHealth,
    version: String,
}
#[derive(Debug, serde::Serialize)]
struct ServicesHealth {
    database: ServiceStatus,
    redis: ServiceStatus,
}

#[derive(Debug, serde::Serialize)]
struct ServiceStatus {
    status: String,
    message: Option<String>,
    latency_ms: u128,
}

pub async fn health_check(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let start_time = std::time::Instant::now();
    // 并行检查数据库和Redis
    let (db_result, redis_result) =
        tokio::join!(check_database(&state.db), check_redis(&state.redis_pool));
    let total_latency = start_time.elapsed();
    info!("Health check completed in {}ms", total_latency.as_millis());

    // 确定总体状态
    let overall_status = if db_result.is_ok() && redis_result.is_ok() {
        "healthy"
    } else {
        "unhealthy"
    };
    // 解析数据库检查结果
    let (db_status, db_message, db_latency) = result_to_status(&db_result, "Postgres");
    // 解析Redis检查结果
    let (redis_status, redis_message, redis_latency) = result_to_status(&redis_result, "Redis");
    // 构建响应
    let response = HealthResponse {
        status: overall_status.to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
        services: ServicesHealth {
            database: ServiceStatus {
                status: db_status,
                message: db_message,
                latency_ms: db_latency,
            },
            redis: ServiceStatus {
                status: redis_status,
                message: redis_message,
                latency_ms: redis_latency,
            },
        },
        version: env!("CARGO_PKG_VERSION").to_string(),
    };
    // 设置HTTP状态码
    let status_code = if overall_status == "healthy" {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };

    (status_code, Json(response))
}

// 辅助函数：将检查结果转换为状态信息
fn result_to_status<T>(
    result: &Result<T, anyhow::Error>,
    service_name: &str,
) -> (String, Option<String>, u128)
where
    T: Copy + Into<u128> + std::fmt::Debug,
{
    debug!("{} result: {:?}", service_name, result);
    match result {
        Ok(latency) => ("healthy".to_string(), None, (*latency).into()),
        Err(e) => ("unhealthy".to_string(), Some(e.to_string()), 0),
    }
}

// 检查数据库连接
async fn check_database(pool: &sqlx::PgPool) -> Result<u128, anyhow::Error> {
    let start_time = std::time::Instant::now();
    match sqlx::query("SELECT 1").fetch_one(pool).await {
        Ok(_) => {
            let latency = start_time.elapsed().as_millis();
            info!("Postgres connection is healthy (latency: {}ms)", latency);
            Ok(latency)
        }
        Err(e) => {
            error!("Postgres connection is unhealthy: {}", e);
            Err(anyhow!("Postgres connection error: {}", e))
        }
    }
}

async fn check_redis(pool: &deadpool_redis::Pool) -> Result<u128, anyhow::Error> {
    let start_time = std::time::Instant::now();
    // 检查redis连接
    match pool.get().await {
        Ok(mut conn) => match deadpool_redis::redis::cmd("PING")
            .query_async::<String>(&mut conn)
            .await
        {
            Ok(response) => {
                if response == "PONG" {
                    let latency = start_time.elapsed().as_millis();
                    debug!("Redis health check: PONG received (latency: {}ms)", latency);
                    Ok(latency)
                } else {
                    error!("Redis PING returned unexpected response: {}", response);
                    Err(anyhow::anyhow!("Unexpected Redis response: {}", response))
                }
            }
            Err(e) => {
                error!("Redis PING command failed: {}", e);
                Err(anyhow::anyhow!("Redis error: {}", e))
            }
        },
        Err(e) => {
            error!("Failed to get Redis connection from pool: {}", e);
            Err(anyhow::anyhow!("Redis connection error: {}", e))
        }
    }
}
