use tracing::{Level, info};

pub fn init_logger() {
    let log_level = std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());

    // 将字符串转换为 Level
    let level = match log_level.to_lowercase().as_str() {
        "trace" => Level::TRACE,
        "debug" => Level::DEBUG,
        "info" => Level::INFO,
        "warn" => Level::WARN,
        "error" => Level::ERROR,
        _ => Level::INFO,
    };

    tracing_subscriber::fmt().with_max_level(level).init();
    info!("🚀 init logger with log level: {}", level);
}
