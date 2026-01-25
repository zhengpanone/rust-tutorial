use crate::interceptor::client_interceptor::ClientInterceptor;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct GrpcConfig {
    pub endpoints: GrpcEndpointsConfig,
    pub auth: GrpcAuth,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GrpcEndpointsConfig {
    pub user_service: String,
    pub order_service: String,
    pub product_service: String,
}
#[derive(Debug, Clone, Deserialize)]
pub struct GrpcAuth {
    pub token: String,
    pub enable_tracing: bool,
}

impl GrpcConfig {
    pub fn create_interceptor(&self) -> ClientInterceptor {
        if self.auth.enable_tracing {
            ClientInterceptor::with_token_and_tracing(&self.auth.token)
        } else {
            ClientInterceptor::with_token(&self.auth.token)
        }
    }
}
