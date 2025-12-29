use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserClientError {
    #[error("endpoint is missing")]
    MissingEndpoint,

    #[error("gRPC transport error: {0}")]
    Transport(#[from] tonic::transport::Error),

    #[error("gRPC status error: {0}")]
    Status(#[from] tonic::Status),
}
