// src/app/bootstrap/services.rs

use crate::app::bootstrap::server::jwt::JwtService;
use crate::app::bootstrap::{InfrastructureServices, server};
use crate::app::config::config::AppConfig;
use crate::app::state::AppState;
use common::error::AppError;
use std::sync::Arc;
use tracing::info;

// 应用服务容器
pub struct AppServiceContainer {
    pub jwt_service: Arc<JwtService>,
    // pub service_app: Arc<ServiceApp>,
    // pub module_app: Arc<ModuleApp>,
    // pub api_app: Arc<dyn ApiApp>,
    // pub auth_app: Arc<dyn AuthApp>,
    // pub user_app: Arc<dyn UserApp>,
    // pub role_app: Arc<dyn RoleApp>,
}

/// 初始化应用状态
pub async fn init_app_state(
    config: Arc<AppConfig>,
    services: &InfrastructureServices,
) -> Result<Arc<AppState>, AppError> {
    info!("🎯 Initializing application state...");

    let service_container = init_app_services(config.clone(), services.clone()).await;
    todo!()
}

pub async fn init_app_services(
    config: Arc<AppConfig>,
    services: InfrastructureServices,
) -> Result<AppServiceContainer, AppError> {
    info!("🏗️  Initializing application services...");

    // 初始化JWT服务
    let jwt_service = server::jwt::init_jwt_service(&config.security, &services).await?;
    todo!()
}
