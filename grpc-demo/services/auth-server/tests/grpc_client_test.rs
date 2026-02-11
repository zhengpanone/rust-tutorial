#[cfg(test)]
mod grpc_client_test {
    use proto::user::GetUserRequest;
    use proto::user::user_service_grpc_client::UserServiceGrpcClient;
    use tonic::transport::Endpoint;

    #[tokio::test]
    pub async fn user_client_get_user_by_id() {
        // 1 连接 gRPC 服务
        let mut client = UserServiceGrpcClient::connect("http://127.0.0.1:50051")
            .await
            .expect("Connect UserServiceGrpc failed");

        // let channel = Endpoint::from_static("http://user-service:50051")
        //     .connect()
        //     .await?;
        //
        // let user_grpc = UserServiceGrpcClient::new(channel);
        // 2 构造请求
        // let request = GetUserRequest {
        //     id: "1".to_string(),
        // };
        // // 3 调用 RPC
        // let response = client
        //     .get_user(request)
        //     .await
        //     .expect("User not found!")
        //     .into_inner();
        //
        // // 4 处理响应
        // if let Some(user) = response.user {
        //     println!("User info:");
        //     println!("  id       = {}", user.id);
        //     println!("  username = {}", user.username);
        //     println!("  email    = {}", user.email);
        // } else {
        //     println!("User not found");
        // }
    }
}
