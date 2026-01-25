use crate::grpc::user::GrpcUserService;
use crate::router::create_router;
use crate::state::{App, AppState};
use anyhow::Context;
use proto::hello::greeter_service_server::GreeterServiceServer;
use proto::user::user_service_grpc_server::UserServiceGrpcServer;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio::signal;
use tonic::transport::Server;
use tonic_health::server::health_reporter;
use tonic_reflection::server::Builder as ReflectionBuilder;
use tracing::log::debug;
use tracing::{error, info};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    debug!("cwd = {:?}", std::env::current_dir());
    // 加载环境变量
    dotenv::from_path(concat!(env!("CARGO_MANIFEST_DIR"), "/.env")).ok();
    // 加载配置
    let config = config::Config::from_env().expect("Failed to load config");

    // 初始化日志
    init::init_logger();

    // let state = Arc::new(AppState::new(config.clone()).await?);
    let app = App::new(config.clone()).await?;
    let state = app.state.clone();

    let router = create_router(state.clone());

    let http_port = std::env::var("HTTP_PORT")
        .unwrap_or_else(|_| "18080".to_string())
        .parse::<u16>()
        .unwrap_or(18080);

    let grpc_addr: SocketAddr = "0.0.0.0:50051".parse()?;
    let http_addr: SocketAddr = format!("0.0.0.0:{}", http_port)
        .parse()
        .expect("Failed to parse listen address");

    // 健康检查
    let (health_reporter, health_service) = health_reporter();

    // 标记 UserService 为 SERVING
    health_reporter
        .set_serving::<UserServiceGrpcServer<GrpcUserService>>()
        .await;
    health_reporter
        .set_serving::<GreeterServiceServer<grpc::hello::GrpcHelloService>>()
        .await;

    // Reflection
    let reflection_service = ReflectionBuilder::configure()
        .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
        .build_v1()?;

    // 创建 gRPC 服务
    let grpc_user_service = app.grpc_user_service();
    let grpc_hello_service = app.grpc_hello_service();

    // 启动 gRPC 服务
    let grpc_server = tokio::spawn(async move {
        info!("Starting gRPC server on {}", grpc_addr);

        Server::builder()
            .add_service(health_service) // 注册健康检查
            .add_service(reflection_service)
            .add_service(UserServiceGrpcServer::new(grpc_user_service))
            .add_service(GreeterServiceServer::new(grpc_hello_service))
            .serve_with_shutdown(grpc_addr, shutdown_signal())
            .await
            .context("gRPC server failed")
    });

    // 启动 HTTP 服务器
    let http_server = tokio::spawn(async move {
        info!("Starting HTTP server on {}", http_addr);
        let listener = TcpListener::bind(http_addr)
            .await
            .context("Failed to bind HTTP address")?;
        axum::serve(listener, router)
            .await
            .context("HTTP server failed")
    });

    // 等待任一服务器出错
    tokio::select! {
        result = grpc_server=>{
            if let Err(e) = result{
                error!("gRPC server error: {:?}",e);
            }
        }
        result = http_server=>{
            if let Err(e)=result{
                error!("HTTP server error: {:?}",e);
            }
        }
    }
    // let a = tokio::try_join!(grpc_server, http_server)?;
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
