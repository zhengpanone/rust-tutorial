// src/app/bootstrap/services.rs

use crate::app::bootstrap::server::jwt::JwtService;
use crate::app::bootstrap::{InfrastructureServices, server};
use crate::app::config::config::AppConfig;
use crate::app::state::AppState;
use crate::application::services::auth_app::AuthApp;
use crate::application::services::impls::auth_app_impl::AuthAppImpl;
use crate::application::services::impls::user_app_impl::UserAppImpl;
use crate::application::services::user_app::UserApp;
use common::error::AppError;
use std::sync::Arc;
use tracing::info;

// 应用服务容器
pub struct AppServiceContainer {
    pub jwt_service: Arc<JwtService>,
    // pub service_app: Arc<ServiceApp>,
    // pub module_app: Arc<ModuleApp>,
    // pub api_app: Arc<dyn ApiApp>,
    pub auth_app: Arc<dyn AuthApp>,
    pub user_app: Arc<dyn UserApp>,
    // pub role_app: Arc<dyn RoleApp>,
}

/// 初始化应用状态
pub async fn init_app_state(
    config: Arc<AppConfig>,
    services: &InfrastructureServices,
) -> Result<Arc<AppState>, AppError> {
    info!("🎯 Initializing application state...");
    // 初始化应用服务容器
    let service_container = init_app_services(config.clone(), services.clone()).await;

    let state = Arc::new(AppState {
        db_pool: services.database_pool.clone(),
        redis_pool: services.redis_pool.clone(),
        jwt_service: service_container?.jwt_service.clone(),
        auth_app: service_container.auth_app.clone(),
        user_app: service_container.user_app.clone(),
    });
    todo!()
}

pub async fn init_app_services(
    config: Arc<AppConfig>,
    services: InfrastructureServices,
) -> Result<AppServiceContainer, AppError> {
    info!("🏗️  Initializing application services...");

    // 初始化JWT服务
    let jwt_service = server::jwt::init_jwt_service(&config.security, &services).await?;

    // 创建服务容器
    let container = AppServiceContainer {
        jwt_service: jwt_service.clone(),
        // service_app: Arc::new(ServiceAppImpl::new(services.database_pool.clone())),
        // module_app: Arc::new(ModuleAppImpl::new(services.database_pool.clone())),
        // api_app: Arc::new(ApiAppImpl::new(services.database_pool.clone())),
        auth_app: Arc::new(AuthAppImpl::new(
            jwt_service.clone(),
            services.database_pool.clone(),
            services.redis_pool.clone(),
        )),
        user_app: Arc::new(UserAppImpl::new(services.database_pool.clone())),
        // role_app: Arc::new(RoleAppImpl::new(services.database_pool.clone())),
    };
    Ok(container)
}
