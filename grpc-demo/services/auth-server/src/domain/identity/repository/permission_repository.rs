use crate::domain::identity::models::permission::{Permission, PermissionId};
use async_trait::async_trait;
use common::enums::permission::PermissionType;
use common::error::AppResult;
use common::web::response::Pagination;

#[async_trait]
pub trait PermissionRepository: Send + Sync {
    async fn save(&self, permission: Permission) -> AppResult<Permission>;
    async fn find_by_id(&self, id: &PermissionId) -> AppResult<Option<Permission>>;
    async fn find_by_code(&self, code: &str) -> AppResult<Option<Permission>>;
    async fn find_all(
        &self,
        filters: &PermissionFilters,
        pagination: &Pagination,
    ) -> AppResult<(Vec<Permission>, i64)>;
    async fn delete(&self, id: &PermissionId) -> AppResult<bool>;
}

#[derive(Debug, Clone, Default)]
pub struct PermissionFilters {
    pub permission_code: Option<String>,
    pub permission_name: Option<String>,
    pub permission_type: Option<PermissionType>,
    pub resource: Option<String>,
    pub parent_id: Option<PermissionId>,
}
