use auth_server::app::bootstrap::AppBootstrap;
use auth_server::app::config::config::AppConfig;
use auth_server::app::config::loader::ConfigLoader;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. 加载配置
    let _config = AppConfig::load().expect("Failed to load config");
    let config: AppConfig = ConfigLoader::load_and_validate().expect("加载配置失败");
    info!("config: {:?}", config);
    // 2. 创建并运行启动器
    let mut bootstrap = AppBootstrap::new(config).await;
    bootstrap.run().await.expect("Failed to run app");

    Ok(())
}

