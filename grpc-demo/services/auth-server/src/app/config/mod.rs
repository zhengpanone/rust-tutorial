// src/app/config/mod.rs
pub mod config;
pub mod constants;
pub mod error;
pub mod loader;
pub mod security;
pub mod summary;
pub mod validator;
mod server;
mod database;

#[cfg(test)]
mod tests {
    use crate::app::config::config::{AppConfig, ConfigSummary};
    use crate::app::config::loader::ConfigLoader;
    use crate::app::config::summary::ConfigSummaryGenerator;

    #[test]
    fn test_config_usage() {
        // 加载配置
        let config: AppConfig = ConfigLoader::load_and_validate().expect("加载配置失败");
        println!("{}", config.environment);

        // 单独加载和验证
        let config = ConfigLoader::load().expect("加载配置失败");
        config.validate().expect("验证配置失败");
        println!("{}", config.environment);

        // 生成摘要
        let summary: ConfigSummary = ConfigSummaryGenerator::generate(&config);

        println!("{}", summary.environment);

        // 生成详细摘要
        let detailed = ConfigSummaryGenerator::generate_detailed(&config);
        println!("{}", detailed);
    }
}
