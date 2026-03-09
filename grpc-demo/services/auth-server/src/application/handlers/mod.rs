use utoipa::OpenApi;
use utoipa::openapi::security::{HttpAuthScheme, SecurityScheme};

pub mod auth_handler;
pub mod health_handler;
pub mod metrics_handler;
pub mod service_handler;
pub mod user_handler;

/// 主 API 文档
#[derive(OpenApi)]
#[openapi(
    // 使用 info 配置 API 基本信息
    info(
        title = "API 文档",
        version = "1.0.0",
        description = "系统 API 文档",
        contact(
            name = "开发团队",
            email = "dev@example.com"
        ),
        license(name = "MIT")
    ),
    // 使用 nest 嵌套子 API
    nest(
        // you can nest sub apis here
        //     (path = "/roles", api = role_handler::RoleApiDoc),
    (path = "/auth", api = auth_handler::AuthApiDoc),
    (path = "/user", api = user_handler::UserApiDoc),
    (path = "/admin/user", api = user_handler::AdminUserApiDoc),
    ),
    // 服务器配置
    servers(
        (url = "/api/v1", description = "API 服务器"),
    ),
    // 全局安全配置
    security(
        ("jwt" = [])
    )
)]
pub struct ApiDoc;

/// 安全配置扩展
pub struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            // 添加 JWT Bearer 认证方案
            components.add_security_scheme(
                "jwt",
                SecurityScheme::Http(utoipa::openapi::security::Http::new(HttpAuthScheme::Bearer)),
            );
            // 也可以添加其他认证方案
            components.add_security_scheme(
                "api_key",
                SecurityScheme::ApiKey(utoipa::openapi::security::ApiKey::Header(
                    utoipa::openapi::security::ApiKeyValue::new("X-API-KEY"),
                )),
            );
        }
    }
}
