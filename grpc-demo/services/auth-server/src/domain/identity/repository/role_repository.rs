use crate::domain::identity::entity::permission::{Permission, PermissionId};
use crate::domain::identity::entity::role::SysRole;
use async_trait::async_trait;
use common::enums::role::RoleType;
use common::error::AppResult;
use common::web::response::Pagination;

#[async_trait]
pub trait RoleRepository: Send + Sync {
    async fn save(&self, role: SysRole) -> AppResult<SysRole>;

    async fn find_by_id(&self, id: &str) -> AppResult<SysRole>;

    async fn find_by_code(&self, code: &str) -> AppResult<SysRole>;

    async fn find_all(
        &self,
        filters: &RoleFilters,
        pagination: &Pagination,
    ) -> AppResult<(Vec<SysRole>, i64)>;
    async fn delete_by_id(&self, id: &str) -> AppResult<bool>;

    async fn assign_permission(&self, role_id: &str, permission_id: &PermissionId)
    -> AppResult<()>;
    async fn remove_permission(&self, role_id: &str, permission_id: &PermissionId)
    -> AppResult<()>;
    async fn get_role_permissions(&self, role_id: &str) -> AppResult<Vec<Permission>>;
}

#[derive(Debug, Clone, Default)]
pub struct RoleFilters {
    pub role_code: Option<String>,
    pub role_name: Option<String>,
    pub role_type: Option<RoleType>,
}
