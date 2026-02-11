// src/app/bootstrap/server/http.rs
use crate::api::http::v1::v1_routes;
use crate::app::config::cors::CorsConfig;
use crate::app::config::server::ServerConfig;
use crate::app::middleware::http::cors::cors_middleware;
use crate::app::middleware::http::logging::request_logger;
use crate::app::state::AppState;
use crate::application::handlers::metrics_handler::prometheus_metrics;
use crate::application::handlers::{ApiDoc, health_handler};
use axum::extract::Request;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::routing::get;
use axum::{Router, middleware};
use common::error::{AppError, AppResult};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::info;
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_scalar::{Scalar, Servable as ScalarServable};
use utoipa_swagger_ui::SwaggerUi;

/// 启动HTTP服务器
pub async fn start_http_server(state: AppState, server_config: &ServerConfig) -> AppResult<()> {
    let addr_str = format!("{}:{}", server_config.host, server_config.port);
    let http_addr: SocketAddr = addr_str.parse().map_err(|e| {
        AppError::Internal(format!(
            "Failed to parse HTTP address '{}': {}",
            addr_str, e
        ))
    })?;

    let app = build_router(state).await;

    dbg!(std::any::type_name_of_val(&app));

    info!("🚀 HTTP server listening on {}", http_addr);

    let listener = TcpListener::bind(&http_addr).await.map_err(|e| {
        AppError::Internal(format!("Failed to bind HTTP address: {}: {}", http_addr, e))
    })?;

    axum::serve(listener, app)
        .await
        .map_err(|e| AppError::Internal(format!("HTTP server failed: {}", e)))?;
    Ok(())
}

/// 添加全局中间件
fn add_global_middleware(app_state: Arc<AppState>, app: Router) -> Router {
    let mut app = app;

    // 添加请求日志中间件
    app = app.layer(middleware::from_fn_with_state(
        app_state.clone(),
        request_logger,
    ));

    // 添加CORS中间件（如果启用）
    let enable_cors = app_state.config.server.enable_cors;
    if enable_cors {
        if let Some(cors_config) = &app_state.config.server.cors {
            // 将CorsConfig包装在Arc中
            let cors_config = Arc::new(cors_config.clone());

            app = app.layer(middleware::from_fn(move |req: Request, next: Next| {
                let config = cors_config.clone();
                async move { cors_middleware(config, req, next).await }
            }));
        }
    }
    app
}

/// 添加OpenAPI文档
fn add_openapi_docs(app: Router) -> Router {
    // 生成 OpenAPI 文档实例
    let api = ApiDoc::openapi();
    // Swagger UI
    app.merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api.clone()))
        // Redoc
        .merge(Redoc::with_url("/redoc", api.clone()))
        .merge(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
        .merge(Scalar::with_url("/scalar", api))
}

fn add_monitoring_endpoints(app: Router) -> Router {
    // 健康检查
    app.route("/health", get(health_handler::health_check))
        .route("/ready", get(health_handler::readiness_check))
        .route("/live", get(health_handler::liveness_check))
        .route("/metrics", get(prometheus_metrics))
        // 版本信息
        .route(
            "/version",
            get(|| async { format!("Microservice Manager v{}", env!("CARGO_PKG_VERSION")) }),
        )
}

/// 构建路由器
pub async fn build_router(state: AppState) -> Router {
    let app_state = Arc::new(state);
    // 构建基础路由
    let mut router = Router::new()
        .nest("/api/v1", v1_routes())
        .fallback(not_found)
        .with_state(Arc::clone(&app_state));

    // 添加监控端点
    if app_state.config.server.enable_metrics {
        router = add_monitoring_endpoints(router)
    }

    // 添加 OpenAPI 文档路由
    if app_state.config.server.enable_openapi {
        router = add_openapi_docs(router);
    }

    router = add_global_middleware(app_state, router);

    router
}

async fn not_found() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "NOT Found")
}
