use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use utoipa::OpenApi;

use crate::{
    schemas::{
        common::IdsDTO,
        role_schemas::{CreateRoleDTO, RoleVO, UpdateRoleDTO},
    },
    state::AppState,
};

const TAG_NAME: &str = "Role API";
/// 创建角色
#[utoipa::path(
    post,
    path = "/create",
    tag = TAG_NAME,
    request_body = CreateRoleDTO,
    // responses(
    //     (status = 201, description = "创建角色", body = ApiResponse<RoleResponse>),
    //     (status = 400, description = "请求参数错误", body = ApiError),
    //     (status = 422, description = "验证失败", body = ApiError),
    //     (status = 500, description = "服务器错误", body = ApiError)
    // )
)]
pub async fn create_role(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateRoleDTO>,
) -> Result<Json<RoleVO>, StatusCode> {
    todo!()
}

/// 删除角色 （204 无内容）
#[utoipa::path(
    delete,
    path = "/delete",
    tag = TAG_NAME,
    request_body = IdsDTO,
    // responses(
    //     (status = 204, description = "角色删除成功"),
    //     (status = 404, description = "角色不存在", body = ApiError),
    //     (status = 500, description = "内部服务器错误", body = ApiError)
    // )
)]
pub async fn delete_role(
    State(state): State<Arc<AppState>>,
    Json(ids): Json<IdsDTO>,
) -> Result<(), StatusCode> {
    todo!()
}

/// 更新角色
#[utoipa::path(put,
    path = "/update",
    tag = TAG_NAME,
    request_body = UpdateRoleDTO,
    // responses(
    //     (status = 200, description = "角色更新成功", body = ApiResponse<RoleResponse>),
    //     (status = 400, description = "请求参数错误", body = ApiError),
    //     (status = 404, description = "角色不存在", body = ApiError),
    //     (status = 422, description = "验证失败", body = ApiError),
    //     (status = 500, description = "服务器错误", body = ApiError)
    // )

)]
pub async fn update_role(
    State(state): State<Arc<AppState>>,
    Json(request): Json<UpdateRoleDTO>,
) -> Result<Json<RoleVO>, StatusCode> {
    todo!()
}

pub async fn get_role_detail(
    State(state): State<Arc<AppState>>,
    Path(role_id): Path<String>,
) -> Result<Json<RoleVO>, StatusCode> {
    todo!()
}

#[derive(OpenApi)]
#[openapi(
    paths(create_role, delete_role, update_role),

tags((name = "Role API", description = "Role management"))
)]
pub struct RoleApiDoc;
