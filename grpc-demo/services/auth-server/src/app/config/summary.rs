// src/app/config/summary.rs

use crate::app::config::config::{AppConfig, ConfigSummary};

/// 配置摘要生成器
pub struct ConfigSummaryGenerator;

impl ConfigSummaryGenerator {
    /// 生成配置摘要
    pub fn generate(config: &AppConfig) -> ConfigSummary {
        ConfigSummary {
            environment: config.environment.clone(),
            http_port: config.server.port,
            grpc_port: config.grpc.port,
            database_url: Self::mask_sensitive_url(&config.database.url),
            redis_url: config
                .redis
                .as_ref()
                .map(|r| Self::mask_sensitive_url(&r.url)),
            // has_message_queue: config.message_queue.is_some(),
            config_files_count: 1, // TODO：统计配置文件数量
            env_vars_count: Self::count_env_overrides(config),
        }
    }

    pub fn generate_detailed(config: &AppConfig) -> String {
        let rate_limit_info = if config.rate_limit.enabled {
            &format!("{} 请求/分钟", config.rate_limit.requests_per_minute)
        } else {
            "未启用"
        };
        format!(
            r#"
配置摘要:
=========
环境: {}
HTTP 服务器: {}:{}
gRPC 服务器: {}:{}
数据库: {}
Redis: {}
日志级别: {}
日志目录: {}
JWT 过期时间: {} 分钟
限流: {}
        "#,
            config.environment,
            config.server.host,
            config.server.port,
            config.grpc.host,
            config.grpc.port,
            Self::mask_sensitive_url(&config.database.url),
            config
                .redis
                .as_ref()
                .map(|r| Self::mask_sensitive_url(&r.url))
                .unwrap_or_else(|| "未启用".to_string()),
            // config
            //     .message_queue
            //     .as_ref()
            //     .map(|r| r.connection.url.clone())
            //     .unwrap_or_else(|| "未启用".to_string()),
            config.logging.log_level,
            config.logging.log_dir,
            config.security.jwt_expiry_minutes,
            rate_limit_info,
        )
    }
    /// 屏蔽敏感信息的 URL
    fn mask_sensitive_url(url: &str) -> String {
        if url.contains("://") {
            let parts: Vec<&str> = url.splitn(2, "://").collect();
            if parts.len() == 2 {
                let scheme = parts[0];
                let rest = parts[1];

                if rest.contains("@") {
                    let cred_parts: Vec<&str> = rest.splitn(2, "@").collect();
                    if cred_parts.len() == 2 {
                        let host = cred_parts[1];
                        return format!("{}://****@{}", scheme, host);
                    }
                }
                return url.to_string();
            }
        }
        url.to_string()
    }
    /// 统计环境变量覆盖数量
    fn count_env_overrides(_config: &AppConfig) -> usize {
        let mut count = 0;
        if std::env::var("DATABASE_URL").is_ok() {
            count += 1;
        }
        if std::env::var("JWT_SECRET").is_ok() {
            count += 1;
        }
        if std::env::var("REDIS_URL").is_ok() {
            count += 1;
        }
        if std::env::var("HTTP_SERVER_ADDRESS").is_ok() {
            count += 1;
        }
        if std::env::var("GRPC_SERVER_ADDRESS").is_ok() {
            count += 1;
        }
        count
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mask_sensitive_url() {
        let url = "postgres://user:password@host:5432/db";
        let masked = ConfigSummaryGenerator::mask_sensitive_url(&url);
        assert_eq!(masked, "postgres://****@host:5432/db");
    }
}
