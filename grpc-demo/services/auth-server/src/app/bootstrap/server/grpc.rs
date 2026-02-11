use crate::app::config::config::GrpcConfig;
use crate::app::state::AppState;
use common::error::AppResult;
use std::net::SocketAddr;
use tracing::{error, info};
use proto::user::user_service_grpc_server::UserServiceGrpcServer;
/// 启动gRPC服务器
pub async fn start_grpc_server(state: AppState, config: &GrpcConfig) -> AppResult<()> {
    info!("⚡ Starting gRPC server on {}:{}", config.host, config.port);
    let addr: SocketAddr = format!("{}:{}", config.host, config.port)
        .parse()
        .map_err(|e| {
            error!("Invalid gRPC address: {}", e);
            anyhow::anyhow!("Invalid gRPC address: {}", e)
        })?;

    // // 创建拦截器
    // let auth_interceptor = AuthInterceptor::new(state.jwt_service.clone());
    // let logging_interceptor = LoggingInterceptor::new();

        // 创建 gRPC 服务
    let grpc_user_service = UserServiceGrpcServer::new(state.user_service.clone());
        // let grpc_user_service = app.grpc_user_service();
        // let grpc_hello_service = app.grpc_hello_service();
    Ok(())
}
