// src/domain/services/repositories/module_repository.rs
use async_trait::async_trait;

use super::super::models::{
    module::{Module, ModuleId},
    service::ServiceId,
};

#[async_trait]
pub trait ModuleRepository: Send + Sync {
    async fn save(&self, module: Module) -> Result<Module, crate::shared::error::Error>;
    async fn find_by_id(
        &self,
        id: &ModuleId,
    ) -> Result<Option<Module>, crate::shared::error::Error>;
    async fn find_by_service_id(
        &self,
        service_id: &ServiceId,
        pagination: &Pagination,
    ) -> Result<(Vec<Module>, i64), crate::shared::error::Error>;
    async fn delete(&self, id: &ModuleId) -> Result<bool, crate::shared::error::Error>;
    async fn exists_by_code(
        &self,
        service_id: &ServiceId,
        code: &str,
    ) -> Result<bool, crate::shared::error::Error>;
    async fn count_by_service(
        &self,
        service_id: &ServiceId,
    ) -> Result<i64, crate::shared::error::Error>;
}
