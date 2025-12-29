use crate::config::database::DatabaseConfig;
use anyhow::{Context, Error};
use serde::Deserialize;
use std::env;

pub mod database;

// 常量定义，便于维护
const ENV_DATABASE_URL: &str = "DATABASE_URL";
const ENV_DATABASE_MAX_CONNECTIONS: &str = "DATABASE_MAX_CONNECTIONS";
const DEFAULT_MAX_CONNECTIONS: u32 = 5;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub database: DatabaseConfig,
}

impl Config {
    pub fn from_env() -> Result<Self, Error> {

        let max_connections: u32 = env::var(ENV_DATABASE_MAX_CONNECTIONS)
            .unwrap_or_else(|_| DEFAULT_MAX_CONNECTIONS.to_string())
            .parse::<u32>()
            .context("Invalid DATABASE_MAX_CONNECTIONS. Must be a positive integer")?;

        // 验证合理性
        if max_connections == 0 {
            anyhow::bail!("{} cannot be 0", ENV_DATABASE_MAX_CONNECTIONS);
        }

        let database = DatabaseConfig {
            url: env::var(ENV_DATABASE_URL).context("DATABASE_URL must be set")?,
            max_connections,
        };
        Ok(Config { database })
    }
}
