use std::str::FromStr;
use tonic::metadata::MetadataValue;
use tonic::service::Interceptor;
use tonic::{Request, Status};
use tracing::info;

#[derive(Clone)]
pub struct ClientInterceptor {
    token: Option<String>,
}

impl ClientInterceptor {
    pub fn new() -> Self {
        Self { token: None }
    }
    pub fn with_token(token: impl Into<String>) -> Self {
        Self {
            token: Some(token.into()),
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
