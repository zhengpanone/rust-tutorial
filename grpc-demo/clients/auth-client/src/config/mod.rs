use serde::Deserialize;

pub mod grpc_config;

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub grpc_config: grpc_config::GrpcConfig,
}

impl AppConfig {
    pub fn from_env() -> Self {
        todo!()
    }
}
