use auth_client::router::create_router;
use auth_client::state::AppState;
use metrics::{counter, gauge, histogram};
use metrics_exporter_prometheus::PrometheusBuilder;
use proto::user::GetUserRequest;
use proto::user::user_service_grpc_client::UserServiceGrpcClient;
use rand::random;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpListener;
use tokio::time::{Instant, interval, sleep};
use tracing::{debug, error, info};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载环境变量
    dotenv::from_path(concat!(env!("CARGO_MANIFEST_DIR"), "/.env")).ok();
    // 初始化日志
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    info!("Starting user service client HTTP API with Prometheus metrics...");
    // 获取配置
    let http_port = std::env::var("HTTP_PORT")
        .unwrap_or_else(|_| "8081".to_string())
        .parse::<u16>()
        .unwrap_or(18081);
    let metrics_port = std::env::var("METRICS_PORT")
        .unwrap_or_else(|_| "9000".to_string())
        .parse::<u16>()
        .unwrap_or(9000);
    let metrics_addr: SocketAddr = format!("127.0.0.1:{}", metrics_port)
        .parse()
        .expect("Failed to parse listen address");

    // 初始化应用状态
    let app_state = Arc::new(AppState::new().await);

    info!("Prometheus metrics available at handler://{}", metrics_addr);

    // 初始化 Prometheus
    PrometheusBuilder::new()
        .with_http_listener(metrics_addr)
        .add_global_label("service", "user-client")
        .add_global_label("version", env!("CARGO_PKG_VERSION"))
        .install_recorder()
        .expect("failed to install Prometheus recorder");

    // 记录启动指标
    counter!("user_client_startups_total").increment(1);
    gauge!("user_client_up").set(1.0);

    // 启动后台指标更新任务
    let state_for_metrics = Arc::clone(&app_state);
    tokio::spawn(async move {
        let mut interval = interval(Duration::from_secs(5));
        loop {
            interval.tick().await;

            // 更新自定义指标
            let total_request = state_for_metrics.get_request_count();
            let total_errors = state_for_metrics.get_error_count();
            let uptime = state_for_metrics.get_uptime();

            gauge!("user_client_custom_total_requests").set(total_request as f64);
            gauge!("user_client_custom_total_errors").set(total_errors as f64);
            gauge!("user_client_custom_uptime_seconds").set(uptime as f64);

            if total_request > 0 {
                let error_rate = (total_errors as f64 / total_request as f64) * 100.0;
                gauge!("user_client_custom_error_rate").set(error_rate);
            }
            // 模拟一些业务指标
            gauge!("user_client_active_users").set(random::<f64>() * 1000.0);
            gauge!("user_client_database_connections").set(random::<f64>() * 50.0);
        }
    });

    let router = create_router(app_state.clone());
    let http_addr = SocketAddr::from(([0, 0, 0, 0], http_port));
    info!("HTTP server listening on {}", http_addr);

    let listener = TcpListener::bind(http_addr).await?;

    axum::serve(listener, router).await?;

    gauge!("user_client_up").set(0.0);
    info!("HTTP server shutdown complete");

    Ok(())
}

// #[tokio::main]
async fn main1() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化 Prometheus 监控
    let builder = PrometheusBuilder::new();

    // 可选：设置 HTTP 端口供 Prometheus 抓取
    // 默认会在 9000 端口暴露 /metrics 端点
    // 设置 Prometheus 监听端口
    let port = std::env::var("METRICS_PORT")
        .unwrap_or_else(|_| "9000".to_string())
        .parse::<u16>()
        .unwrap_or(9000);
    // 创建 SocketAddr
    let listen_addr: SocketAddr = format!("0.0.0.0:{}", port)
        .parse()
        .expect("Failed to parse listen address");
    info!(
        "📊 Prometheus metrics available at handler://{}",
        listen_addr
    );

    // 安装 recorder
    let builder = builder
        .with_http_listener(listen_addr)
        .add_global_label("service", "user-client")
        .add_global_label("version", env!("CARGO_PKG_VERSION"))
        .set_buckets(&[
            0.001, 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,
        ])
        .expect("failed to set histogram buckets");

    builder
        .install_recorder()
        .expect("failed to install Prometheus recorder");

    info!("Prometheus recorder installed successfully");
    // 记录启动指标
    counter!("user_client_startups_total").increment(1);
    gauge!("user_client_up").set(1.0);

    // 连接到 gRPC 服务
    let server_address =
        std::env::var("USER_SERVICE_ADDR").unwrap_or_else(|_| "http://127.0.0.1:50051".to_string());
    info!("Connecting to user service at {}", server_address);

    // 重试连接逻辑
    let mut retry_count = 0;
    let max_retries = 5;
    let mut client = loop {
        match UserServiceGrpcClient::connect(server_address.clone()).await {
            Ok(c) => {
                info!("Successfully connected to user service");
                break c;
            }
            Err(e) => {
                error!("Failed to connect to user service: {:?}", e);
                if retry_count >= max_retries {
                    return Err(format!("Failed to connect after {} retries", max_retries).into());
                }
                retry_count += 1;
                info!(
                    "Retrying connection (attempt {}/{}) in 2 seconds...",
                    retry_count, max_retries
                );
                sleep(Duration::from_secs(2)).await;
            }
        }
    };

    // 可选：持续发送请求进行测试
    let test_count: usize = std::env::var("TEST_COUNT")
        .unwrap_or_else(|_| "1".to_string())
        .parse()
        .unwrap_or(1);

    info!("Starting test with {} request(s)", test_count);
    for i in 0..test_count {
        let start_time = Instant::now();
        gauge!("user_client_requests_in_progress", "endpoint" => "get_user").increment(1.0);

        let request = tonic::Request::new(GetUserRequest {
            include_sensitive: false,
            fields: vec![],
            identifier: None,
        });

        debug!("Sending request #{}, ID: {}", i + 1, i);

        match client.get_user(request).await {
            Ok(response) => {
                let duration = start_time.elapsed().as_secs_f64();
                debug!(
                    "Response #{}, duration: {:?}, RESPONSE={:?}",
                    i + 1,
                    duration,
                    response
                );

                // 记录成功的指标

                counter!("user_client_requests_total", "status" => "success", "endpoint" => "get_user").increment(1);
                histogram!("user_client_request_duration_seconds", "endpoint" => "get_user")
                    .record(duration);

                info!(
                    "Request #{} completed successfully in {:?}",
                    i + 1,
                    duration
                );
            }
            Err(err) => {
                error!("Request #{} failed: {:?}", i + 1, err);

                // 记录失败的指标
                counter!("user_client_requests_total", "status" => "error", "endpoint" => "get_user", "error_type" => err.code().to_string())
                    .increment(1);

                // 如果是可重试的错误，可以添加重试逻辑
                match err.code() {
                    tonic::Code::Unavailable | tonic::Code::DeadlineExceeded => {
                        if retry_count < 3 {
                            info!("Retrying request #{} due to {:?}", i + 1, err.code());
                            sleep(Duration::from_secs(1)).await;
                            continue;
                        }
                    }
                    _ => {}
                }
            }
        }
        gauge!("user_client_requests_in_progress", "endpoint" => "get_user").decrement(1.0);

        // 如果不是最后一个请求，等待一段时间
        if i < test_count - 1 {
            let interval: u64 = std::env::var("REQUEST_INTERVAL_MS")
                .unwrap_or_else(|_| "1000".to_string())
                .parse()
                .unwrap_or(1000);

            sleep(Duration::from_millis(interval)).await;
        }
    }

    info!("All requests completed successfully");

    // 如果需要保持运行以供 Prometheus 抓取指标
    if std::env::var("KEEP_ALIVE").unwrap_or_else(|_| "false".to_string()) == "true" {
        info!("Keeping service alive for metrics collection. Press Ctrl+C to exit.");
        tokio::signal::ctrl_c().await?;
        info!("Shutting down...");
    }
    Ok(())
}
