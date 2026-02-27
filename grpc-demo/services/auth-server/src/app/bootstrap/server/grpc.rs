// src/app/bootstrap/server/grpc.rs

use crate::api::grpc::v1::services::user::GrpcUserService;
use crate::app::config::config::GrpcConfig;
use crate::app::state::AppState;
use anyhow::Context;
use common::error::AppResult;
use proto::user::user_service_grpc_server::UserServiceGrpcServer;
use std::net::SocketAddr;
use tokio::signal;
use tonic::transport::Server;
use tonic_reflection::server::Builder as ReflectionBuilder;
use tracing::{error, info};

/// 启动gRPC服务器
pub async fn start_grpc_server(state: AppState, config: &GrpcConfig) -> AppResult<()> {
    let grpc_addr: SocketAddr =
        format!("{}:{}", config.host, config.port)
            .parse()
            .map_err(|e| {
                error!("Invalid gRPC address: {}", e);
                anyhow::anyhow!("Invalid gRPC address: {}", e)
            })?;

    // Reflection
    let reflection_service = ReflectionBuilder::configure()
        .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
        .build_v1()
        .expect("Failed to build reflection service");
    // // 创建拦截器
    // let auth_interceptor = AuthInterceptor::new(state.jwt_service.clone());
    // let logging_interceptor = LoggingInterceptor::new();

    // 创建 gRPC 服务
    let grpc_user_service = GrpcUserService::new(state.user_service.clone());
    info!("Starting gRPC server on {}", grpc_addr);

    // 启动 gRPC 服务
    Server::builder()
        .add_service(reflection_service)
        .add_service(UserServiceGrpcServer::new(grpc_user_service))
        .serve_with_shutdown(grpc_addr, shutdown_signal())
        .await
        .context("gRPC server failed")
        .expect("gRPC server failed");

    Ok(())
}

// 关机信号处理
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    info!("Shutdown signal received");
}
