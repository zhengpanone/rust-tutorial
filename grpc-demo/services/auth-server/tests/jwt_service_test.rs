#[cfg(test)]
mod tests {
    use super::*;
    use auth_server::app::bootstrap::InfrastructureServices;
    use auth_server::app::bootstrap::database::init_database;
    use auth_server::app::bootstrap::server::jwt::init_jwt_service;
    use auth_server::app::config::config::SecurityConfig;
    use serde::de::Unexpected::Option;

    async fn test_jwt_service_init() {
        // 创建测试配置
        let config = SecurityConfig {
            jwt_secret: "test-secret-key-for-testing-only-change-in-production".to_string(),
            jwt_expiry_minutes: 5,
            refresh_token_expiry_days: 1,
            password_hash_cost: 10,
        };

        // let database_pool = init_database(&config.database);
        // let services = InfrastructureServices {
        //     database_pool,
        //     redis_pool: None,
        // };
        //
        // let jwt_service = init_jwt_service(&config, None)
        //     .await
        //     .expect("Failed to initialize JWT service");
        // assert!(jwt_service.is_some());
    }
}
