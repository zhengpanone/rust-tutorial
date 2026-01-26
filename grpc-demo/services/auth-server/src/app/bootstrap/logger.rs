use crate::app::config::config::LogConfig;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Layer, fmt};

/// 初始化日志系统，返回 WorkerGuard
/// 注意：必须保留返回的 guard，否则文件日志会停止工作
pub fn init_logger(config: &LogConfig) -> WorkerGuard {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(config.log_level.clone()));

    // 控制台输出
    let stdout_layer = fmt::layer()
        .with_writer(std::io::stdout)
        .with_target(true) // 显示目标模块
        .with_line_number(true) // 显示行号
        .with_ansi(true)
        .with_filter(env_filter.clone());

    // 文件输出
    let file_appender = rolling::daily(&config.log_dir, "auth-server.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    let file_layer = fmt::layer()
        .with_writer(non_blocking)
        .with_ansi(false)
        .json()
        .with_filter(env_filter.clone());

    // 设置全局日志
    // 设置全局日志
    tracing_subscriber::registry()
        .with(stdout_layer)
        .with(file_layer)
        .init();
    // 返回 guard，由调用者持有
    guard
}
