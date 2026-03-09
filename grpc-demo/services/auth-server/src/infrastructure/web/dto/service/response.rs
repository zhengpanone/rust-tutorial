use crate::domain::services::value_objects::{ServiceStatus, ServiceType};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct ServiceVO {
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
