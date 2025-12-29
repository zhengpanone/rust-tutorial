use serde::Deserialize;
use std::time::Duration;

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RedisConfig {
    pub url: String,
    pub max_connections: usize,
    pub connection_timeout: Duration,
    pub pool_timeout: Duration,
    pub ping_on_checkout: bool,
}

impl Default for RedisConfig {
    fn default() -> Self {
        Self {
            url: "redis://127.0.0.1:6379".to_string(),
            max_connections: 10,
            connection_timeout: Duration::from_secs(5),
            pool_timeout: Duration::from_secs(10),
            ping_on_checkout: true,
        }
    }
}
