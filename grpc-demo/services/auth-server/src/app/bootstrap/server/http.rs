use crate::app::config::server::ServerConfig;
use crate::app::state::AppState;
use crate::application::handlers::health_handler;
use axum::Router;

use std::net::SocketAddr;

use tracing::info;

// src/app/bootstrap/server/http.rs
pub async fn start_http_server(state: AppState, server_config: &ServerConfig) {
    info!(
        "🌐 Starting HTTP server on {}:{}",
        server_config.host, server_config.port
    );

    let app = build_router(state.clone(), server_config);

    let http_addr: SocketAddr = format!("{}:{}", server_config.host, server_config.port)
        .parse()
        .expect("Failed to parse HTTP address");

    info!("Starting HTTP server on {}", http_addr);
    // let listener = TcpListener::bind(http_addr)
    //     .await
    //     .context("Failed to bind HTTP address")?;
    // axum::serve(listener, app)
    //     .await
    //     .context("HTTP server failed");
    todo!()
}

/// 构建路由器
pub async fn build_router(state: AppState, server_config: &ServerConfig) -> Router {
    // let mut router = Router::new();

    // let public_routes = Router::new()
    //     .route("/api/v1/auth/login", post(auth_handler::login))
    //     .route("/api/v1/auth/register", post(auth_handler::register))
    //     .route("/api/v1/auth/logout", post(health_handler::health_check))
    // Swagger UI
    // .route(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api.clone()))
    // Redoc
    // .route(Redoc::with_url("/redoc", api.clone()))
    // .route(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
    // .route(Scalar::with_url("/scalar", api))

    //
    // 业务 API
    // .nest("/users", user_router::routers(state.clone()))
    // .nest("/roles", role_router::role_router(state.clone()))
    // .route("/health", get(health_check))
    // .fallback(not_found)
    // .with_state(state.clone())

    todo!()
}
