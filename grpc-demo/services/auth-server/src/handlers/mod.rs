use utoipa::OpenApi;

pub mod health_handler;
pub mod role_handler;
pub mod service_handler;
pub mod user_handler;

// OpenAPI 文档定义
// 合并多个 OpenApi 文档
#[derive(OpenApi)]
#[openapi(
        nest(
            // you can nest sub apis here
            // (path = "/users", api = user_handler::UserApiDoc),
            (path = "/roles", api = role_handler::RoleApiDoc),
        )
    )]
pub struct ApiDoc;
