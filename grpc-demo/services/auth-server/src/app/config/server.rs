use crate::app::config::cors::CorsConfig;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// 服务器配置
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ServerConfig {
    /// 主机地址
    #[validate(length(min = 1, message = "主机地址不能为空"))]
    pub host: String,
    /// 端口
    #[validate(range(min = 1, max = 65535))]
    pub port: u16,

    /// 是否启用CORS
    pub enable_cors: bool,
    pub cors: Option<CorsConfig>,

    /// 请求超时时间(秒)
    #[validate(range(min = 1, max = 300, message = "请求超时时间必须在1-300秒之间"))]
    pub request_timeout_secs: u64,

    /// 请求体大小限制
    #[validate(range(
        min = 1024,
        max = 104857600,
        message = "请求体大小限制必须在1KB-100MB之间"
    ))] // 1KB- 100MB
    pub body_limit: usize,

    /// 是否启用健康检查
    pub enable_health_check: bool,

    /// 健康检查端口
    #[validate(range(min = 1, max = 65535, message = "健康检查端口必须在1-65535之间"))]
    pub health_check_port: u16,

    /// 是否启用混合模式
    pub enable_hybrid: bool,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8080,
            enable_cors: true,
            cors: None,
            request_timeout_secs: 10,
            enable_health_check: true,
            health_check_port: 8888,
            body_limit: 10000,
            enable_hybrid: false,
        }
    }
}
