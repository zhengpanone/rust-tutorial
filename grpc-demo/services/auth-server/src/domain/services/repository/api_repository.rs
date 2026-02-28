// src/domain/services/repository/api_repository.rs
use async_trait::async_trait;

use super::super::{
    models::api::{Api, ApiId},
    models::module::ModuleId,
    value_objects::{ApiMethod, ApiStatus},
};

#[async_trait]
pub trait ApiRepository: Send + Sync {
    async fn save(&self, api: Api) -> Result<Api, crate::shared::error::Error>;
    async fn find_by_id(&self, id: &ApiId) -> Result<Option<Api>, crate::shared::error::Error>;
    async fn find_by_module_id(
        &self,
        module_id: &ModuleId,
        pagination: &Pagination,
    ) -> Result<(Vec<Api>, i64), crate::shared::error::Error>;
    async fn delete(&self, id: &ApiId) -> Result<bool, crate::shared::error::Error>;
    async fn exists_by_path(
        &self,
        module_id: &ModuleId,
        method: ApiMethod,
        path: &str,
    ) -> Result<bool, crate::shared::error::Error>;
    async fn count_by_module(
        &self,
        module_id: &ModuleId,
    ) -> Result<i64, crate::shared::error::Error>;
    async fn find_by_status(
        &self,
        status: ApiStatus,
    ) -> Result<Vec<Api>, crate::shared::error::Error>;
}
