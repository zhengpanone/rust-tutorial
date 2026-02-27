// src/application/init/mod.rs
//! 应用初始化服务
//! 负责在应用启动时执行初始化任务

mod account_init;

#[derive(Debug, Clone)]
pub struct InitConfig {
    pub enabled: bool,
    /// 超级管理员配置
    pub super_admin: SuperAdminConfig,
    /// 三权账号配置
    pub three_auth: ThreeAuthConfig,
}
/// 超级管理员配置
#[derive(Debug, Clone)]
pub struct SuperAdminConfig {
    pub username: String,
    pub password: String,
}

/// 三权账号配置
#[derive(Debug, Clone)]
pub struct ThreeAuthConfig {
    pub username: String,
    pub password: String,
}

// TODO
