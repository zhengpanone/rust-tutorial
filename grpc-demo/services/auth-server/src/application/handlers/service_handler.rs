// src/handlers/service_handler.rs
use axum::{
    Json,
    extract::{Path, Query, State},
};
use common::web::response::ApiResponse;
use uuid::Uuid;
use validator::Validate;

use crate::{AppState, models::service::Service, dto::service_schemas::CreateServiceDTO};

#[utoipa::path(
    post,
    path = "/api/services",
    request_body = CreateServiceDTO,
    responses(
        (status = 201, description = "创建成功", body = Service),
        (status = 400, description = "请求参数错误"),
        (status = 409, description = "服务编码已存在"),
    )
)]
pub async fn create_service(
    State(state): State<AppState>,
    Json(payload): Json<CreateServiceDTO>,
) -> ApiResponse<Service> {
    // 验证请求参数
    if let Err(errors) = payload.validate() {
        return ApiResponse::bad_request(&format!("参数验证失败: {:?}", errors));
    }

    // 检查服务编码是否已存在
    let existing = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM services WHERE service_code = ?",
        payload.service_code
    )
        .fetch_one(&state.pool)
        .await
        .unwrap_or(0);

    if existing > 0 {
        return ApiResponse::error(409, "服务编码已存在");
    }

    let service_id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now();

    match sqlx::query_as!(
        Service,
        r#"
        INSERT INTO services (
            id, service_code, service_name, service_type, service_desc,
            owner_team, base_url, status, health_endpoint, is_internal,
            created_at, updated_at
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        RETURNING *
        "#,
        service_id,
        payload.service_code,
        payload.service_name,
        payload.service_type as i8,
        payload.service_desc,
        payload.owner_team,
        payload.base_url,
        payload.status as i8,
        payload.health_endpoint,
        payload.is_internal,
        now,
        now
    )
        .fetch_one(&state.pool)
        .await
    {
        Ok(service) => ApiResponse::created(service),
        Err(e) => {
            tracing::error!("创建服务失败: {}", e);
            ApiResponse::internal_error("创建服务失败")
        }
    }
}

#[utoipa::path(
    get,
    path = "/api/services/{id}",
    params(
        ("id" = String, Path, description = "服务ID")
    ),
    responses(
        (status = 200, description = "获取成功", body = Service),
        (status = 404, description = "服务不存在"),
    )
)]
pub async fn get_service(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> ApiResponse<Service> {
    match sqlx::query_as!(Service, r#"SELECT * FROM services WHERE id = ?"#, id)
        .fetch_optional(&state.pool)
        .await
    {
        Ok(Some(service)) => ApiResponse::success(service),
        Ok(None) => ApiResponse::not_found("服务不存在"),
        Err(e) => {
            tracing::error!("查询服务失败: {}", e);
            ApiResponse::internal_error("查询服务失败")
        }
    }
}

#[utoipa::path(
    get,
    path = "/api/services",
    params(ServiceQuery),
    responses(
        (status = 200, description = "获取成功", body = PaginatedResponse<Service>),
    )
)]
pub async fn list_services(
    State(state): State<AppState>,
    Query(query): Query<ServiceQuery>,
) -> ApiResponse<PaginatedResponse<Service>> {
    let page = query.page.unwrap_or(1).max(1);
    let page_size = query.page_size.unwrap_or(10).max(1).min(100);
    let offset = (page - 1) * page_size;

    // 构建动态查询
    let mut query_builder = sqlx::query_as::<_, Service>("SELECT * FROM services WHERE 1=1");
    let mut count_builder = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM services WHERE 1=1");

    if let Some(ref service_code) = query.service_code {
        let pattern = format!("%{}%", service_code);
        query_builder = query_builder.bind(&pattern);
        count_builder = count_builder.bind(&pattern);
        // 注意：实际使用需要更复杂的构建方式，这里简化处理
    }

    // 使用安全的方式获取总数
    let count = count_builder.fetch_one(&state.pool).await.unwrap_or(0);

    // 获取分页数据
    let services = query_builder
        .fetch_all(&state.pool)
        .await
        .unwrap_or_else(|_| vec![]);

    let total_pages = (count as f64 / page_size as f64).ceil() as i64;

    ApiResponse::success(PaginatedResponse {
        items: services,
        total: count,
        page,
        page_size,
        total_pages,
    })
}

#[utoipa::path(
    put,
    path = "/api/services/{id}",
    params(
        ("id" = String, Path, description = "服务ID")
    ),
    request_body = UpdateServiceRequest,
    responses(
        (status = 200, description = "更新成功", body = Service),
        (status = 404, description = "服务不存在"),
    )
)]
pub async fn update_service(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateServiceRequest>,
) -> ApiResponse<Service> {
    // 验证请求参数
    if let Err(errors) = payload.validate() {
        return ApiResponse::bad_request(&format!("参数验证失败: {:?}", errors));
    }

    // 使用参数化查询更新服务
    let result = sqlx::query_as!(
        Service,
        r#"
        UPDATE services
        SET service_name = COALESCE($1, service_name),
            service_type = COALESCE($2, service_type),
            service_desc = $3,
            owner_team = $4,
            base_url = $5,
            status = COALESCE($6, status),
            health_endpoint = $7,
            is_internal = COALESCE($8, is_internal),
            updated_at = $9
        WHERE id = $10
        RETURNING *
        "#,
        payload.service_name,
        payload.service_type.map(|t| t as i8),
        payload.service_desc,
        payload.owner_team,
        payload.base_url,
        payload.status.map(|s| s as i8),
        payload.health_endpoint,
        payload.is_internal,
        chrono::Utc::now(),
        id
    )
        .fetch_optional(&state.pool)
        .await;

    match result {
        Ok(Some(service)) => ApiResponse::success(service),
        Ok(None) => ApiResponse::not_found("服务不存在"),
        Err(e) => {
            tracing::error!("更新服务失败: {}", e);
            ApiResponse::internal_error("更新服务失败")
        }
    }
}

#[utoipa::path(
    delete,
    path = "/api/services/{id}",
    params(
        ("id" = String, Path, description = "服务ID")
    ),
    responses(
        (status = 204, description = "删除成功"),
        (status = 404, description = "服务不存在"),
    )
)]
pub async fn delete_service(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> ApiResponse<()> {
    match sqlx::query!("DELETE FROM services WHERE id = ?", id)
        .execute(&state.pool)
        .await
    {
        Ok(result) => {
            if result.rows_affected() > 0 {
                ApiResponse::no_content()
            } else {
                ApiResponse::not_found("服务不存在")
            }
        }
        Err(e) => {
            tracing::error!("删除服务失败: {}", e);
            ApiResponse::internal_error("删除服务失败")
        }
    }
}
