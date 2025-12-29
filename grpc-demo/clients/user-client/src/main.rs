mod error;
mod client;
mod interceptor;

use proto::user::GetUserRequest;
use proto::user::user_service_grpc_client::UserServiceGrpcClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    let mut client = UserServiceGrpcClient::connect("http://127.0.0.1:50051").await?;

    let request = tonic::Request::new(GetUserRequest {
        id: "1".to_string(),
    });
    let response = client.get_user(request).await?;

    println!("RESPONSE={:?}", response);
    Ok(())
}
