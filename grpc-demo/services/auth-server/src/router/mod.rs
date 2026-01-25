use crate::handlers::ApiDoc;
use crate::handlers::health_handler::health_check;
use crate::state::AppState;
use axum::Router;
use axum::http::StatusCode;
use axum::routing::get;
use std::sync::Arc;
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_scalar::{Scalar, Servable as ScalarServable};
use utoipa_swagger_ui::SwaggerUi;

mod role_router;
mod user_router;

pub fn create_router(state: Arc<AppState>) -> Router {
    // 生成 OpenAPI 文档实例
    let api = ApiDoc::openapi();

    Router::new()
        // Swagger UI
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api.clone()))
        // Redoc
        .merge(Redoc::with_url("/redoc", api.clone()))
        .merge(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
        .merge(Scalar::with_url("/scalar", api))
        // 业务 API
        .nest("/users", user_router::routers(state.clone()))
        .nest("/roles", role_router::role_router(state.clone()))
        .route("/health", get(health_check))
        .fallback(not_found)
        .with_state(state.clone())
}

async fn not_found() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "NOT Found")
}
