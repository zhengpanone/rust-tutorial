use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateRoleDTO {
    name: String,
    description: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateRoleDTO {
    id: String,
    name: String,
    description: String,
}

#[derive(Serialize, ToSchema)]
pub struct RoleVO {
    id: String,
    name: String,
    description: String,
    created_at: String,
    updated_at: String,
}
