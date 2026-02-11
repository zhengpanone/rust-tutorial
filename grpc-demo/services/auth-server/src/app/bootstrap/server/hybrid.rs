use crate::app::config::config::AppConfig;
use crate::app::state::AppState;
use common::error::AppResult;
use std::sync::Arc;
use tokio::task::JoinSet;
use tracing::info;
use crate::app::bootstrap::server::grpc::start_grpc_server;

/// 启动混合服务器（HTTP + gRPC）
pub async fn start_hybrid_server(state: Arc<AppState>, config: &AppConfig) -> AppResult<()> {
    info!("🚀 Starting hybrid server (HTTP + gRPC)...");
    // let mut tasks = JoinSet::new();
    // 克隆状态用于grpc服务

    let grpc_state = state.clone();
    let grpc_config = config.grpc.clone();

    todo!()
    // tasks.spawn(async move{
    //     info!("⚡ Starting gRPC server on port {}...", grpc_config.port);
    //     if let Err(e) = start_grpc_server(grpc_state, &grpc_config).await {
    //         error!("❌ gRPC server failed: {}", e);
    //     }
    // });
}
