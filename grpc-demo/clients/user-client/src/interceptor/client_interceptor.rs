use std::str::FromStr;
use tonic::metadata::MetadataValue;
use tonic::service::Interceptor;
use tonic::{Request, Status};
use tracing::info;

#[derive(Clone)]
pub struct ClientInterceptor {
    token: Option<String>,
    enable_tracing: bool,
}

impl ClientInterceptor {
    pub fn new() -> Self {
        Self { token: None ,enable_tracing: false,}
    }
    pub fn with_token(token: impl Into<String>) -> Self {
        Self {
            token: Some(token.into()),
            enable_tracing: false,
        }
    }

    /// ⭐ 带 Tracing（你要的 with_tracing）
    pub fn with_tracing() -> Self {
        Self {
            token: None,
            enable_tracing: true,
        }
    }

    /// Token + Tracing
    pub fn with_token_and_tracing(token: impl Into<String>) -> Self {
        Self {
            token: Some(token.into()),
            enable_tracing: true,
        }
    }
}

impl Interceptor for ClientInterceptor {
    fn call(&mut self, mut request: Request<()>) -> Result<Request<()>, Status> {
        let request_id = uuid::Uuid::new_v4().to_string();
        request.metadata_mut().insert(
            "x-request-id",
            MetadataValue::from_str(&request_id).unwrap(),
        );
        if let Some(token) = &self.token {
            request.metadata_mut().insert(
                "authorization",
                format!("Bearer {}", token).parse().unwrap(),
            );
        }
        info!(%request_id, "grpc request sent");
        Ok(request)
    }
}
