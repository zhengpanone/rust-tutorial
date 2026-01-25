use chrono::Utc;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::{
    enums::service_enums::{ServiceStatus, ServiceType},
    models::service::Service,
};

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateServiceDTO {
    #[validate(length(min = 1, max = 50, message = "服务编码长度必须在1-50之间"))]
    pub service_code: String,

    #[validate(length(min = 1, max = 100, message = "服务名称长度必须在1-100之间"))]
    pub service_name: String,

    pub service_type: ServiceType,

    #[validate(length(max = 500, message = "服务描述长度不能超过500"))]
    pub service_desc: Option<String>,

    #[validate(length(max = 100, message = "负责团队长度不能超过100"))]
    pub owner_team: Option<String>,

    #[validate(url(message = "基础URL格式不正确"))]
    #[validate(length(max = 200, message = "基础URL长度不能超过200"))]
    pub base_url: Option<String>,

    pub status: ServiceStatus,

    #[validate(length(max = 200, message = "健康检查端点长度不能超过200"))]
    pub health_endpoint: Option<String>,

    pub is_internal: bool,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct UpdateServiceDTO {
    #[validate(length(min = 1, max = 100, message = "服务名称长度必须在1-100之间"))]
    pub service_name: Option<String>,

    pub service_type: Option<ServiceType>,

    #[validate(length(max = 500, message = "服务描述长度不能超过500"))]
    pub service_desc: Option<String>,

    #[validate(length(max = 100, message = "负责团队长度不能超过100"))]
    pub owner_team: Option<String>,

    #[validate(url(message = "基础URL格式不正确"))]
    #[validate(length(max = 200, message = "基础URL长度不能超过200"))]
    pub base_url: Option<String>,

    pub status: Option<ServiceStatus>,

    #[validate(length(max = 200, message = "健康检查端点长度不能超过200"))]
    pub health_endpoint: Option<String>,

    pub is_internal: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, utoipa::IntoParams)]
pub struct ServiceQueryDto {
    pub service_code: Option<String>,
    pub service_name: Option<String>,
    pub service_type: Option<ServiceType>,
    pub status: Option<ServiceStatus>,
    pub is_internal: Option<bool>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

// From trait 实现：DTO -> Model
impl From<CreateServiceDTO> for Service {
    fn from(request: CreateServiceDTO) -> Self {
        let now = Utc::now();
        let id = Uuid::new_v4().to_string();

        Service {
            id,
            service_code: request.service_code,
            service_name: request.service_name,
            service_type: request.service_type,
            service_desc: request.service_desc,
            owner_team: request.owner_team,
            base_url: request.base_url,
            status: request.status,
            health_endpoint: request.health_endpoint,
            is_internal: request.is_internal,
            created_at: now,
            updated_at: now,
        }
    }
}
