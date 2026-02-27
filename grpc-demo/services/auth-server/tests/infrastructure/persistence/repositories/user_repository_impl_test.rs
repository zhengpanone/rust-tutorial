#[cfg(test)]
mod tests {
    use auth_server::app::bootstrap::database::init_database;
    use auth_server::app::config::loader::ConfigLoader;
    use auth_server::domain::identity::entities::user::User;
    use auth_server::domain::identity::repositories::user_repository::UserRepository;
    use auth_server::infrastructure::persistence::repositories::user_repository_impl::UserRepositoryImpl;
    use chrono::Utc;
    use common::security::jwt::claim::UserStatus;
    use sqlx::PgPool;
    use tracing::error;
    use uuid::Uuid;

    /// 创建测试数据库连接池
    async fn setup_test_pool() -> PgPool {
        // 加载配置
        let config = ConfigLoader::load_and_validate().expect("加载配置失败");
        init_database(&config.database)
            .await
            .expect("初始化数据库失败")
    }
    /// 清理测试数据
    #[allow(dead_code)]
    async fn cleanup_test_data(pool: &PgPool, user_id: &Uuid) {
        let _ = sqlx::query!(
            r#"
            DELETE FROM sys_user WHERE id = $1
            "#,
            user_id
        )
        .execute(pool)
        .await;
    }

    #[tokio::test]
    async fn create_test_user() {
        let pool = setup_test_pool().await;
        let repository = UserRepositoryImpl::new(pool.clone(), None);

        let user_id = Uuid::new_v4();
        let username = format!("test_user_{}", user_id);
        let password = "admin123456";
        let email = format!("test_{}@test.com", user_id);
        let phone = "13800138000";

        let user = User {
            id: user_id,
            username: username.to_string(),
            email: email.to_string(),
            phone: Some(phone.to_string()),
            password_hash: password.to_string(),
            display_name: "".to_string(),
            avatar_url: None,
            roles: Default::default(),
            permissions: Default::default(),
            email_verified: false,
            phone_verified: false,
            status: UserStatus::Active,
            last_login_at: None,
            login_count: 0,
            failed_login_count: 0,
            last_failed_login_at: None,
            locked_at: None,
            lock_reason: None,
            password_changed_at: None,
            password_expires_at: None,
            is_first_login: false,
            last_activity_at: None,
            timezone: None,
            language: None,
            metadata: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
        };

        repository.save(user).await.expect("创建用户失败");
    }

    #[tokio::test]
    async fn test_find_by_id() {
        let pool = setup_test_pool().await;
        let repository = UserRepositoryImpl::new(pool.clone(), None);
        let user_id = Uuid::parse_str("ae63c5c2-d672-4e18-b78f-d353f870d44c").unwrap_or_else(|e| {
            error!("Failed to parse user_id from claims.sub: {}", e);
            Uuid::new_v4()
        });
        let found_user = repository.find_by_id(&user_id).await.unwrap();
        assert!(found_user.is_some());
        assert_eq!(found_user.unwrap().id, user_id);
    }
}
