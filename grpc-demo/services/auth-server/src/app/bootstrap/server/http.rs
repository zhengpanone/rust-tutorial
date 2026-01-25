use axum::Router;

// src/app/bootstrap/server/http.rs
pub async fn start_http_server() {}

pub async fn build_router() {
    let mut router = Router::new();
    // 生成 OpenAPI 文档实例
    // let api = ApiDoc::openapi();


    let public_routes = Router::new()
     // Swagger UI
    // .route(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api.clone()))
        // Redoc
        // .route(Redoc::with_url("/redoc", api.clone()))
        // .route(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
        // .route(Scalar::with_url("/scalar", api))
        // .route("/api/v1/auth/login", axum::routing::post(crate::application::handlers::auth_handler::login))
        // .route("/api/v1/auth/register", axum::routing::post(crate::application::handlers::auth_handler::register))
        ;
    //   let  protected_routes=
       
        // 业务 API
        // .nest("/users", user_router::routers(state.clone()))
        // .nest("/roles", role_router::role_router(state.clone()))
        // .route("/health", get(health_check))
        // .fallback(not_found)
        // .with_state(state.clone())
}
