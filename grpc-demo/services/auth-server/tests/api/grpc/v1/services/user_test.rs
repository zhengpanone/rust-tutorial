#[cfg(test)]
mod grpc_client_test {
    use proto::common::CommonId;
    use proto::user::CreateUserRequest;
    use proto::user::user_service_grpc_client::UserServiceGrpcClient;
    use sqlx::encode::IsNull::No;
    use tonic::transport::Endpoint;

    #[tokio::test]
    pub async fn test_get_user_by_id() {
        // 1 连接 gRPC 服务
        let mut client = UserServiceGrpcClient::connect("http://127.0.0.1:50051")
            .await
            .expect("Connect UserServiceGrpc failed");

        // let channel = Endpoint::from_static("http://user-service:50051")
        //     .connect()
        //     .await
        //     .unwrap();
        //
        // let user_grpc = UserServiceGrpcClient::new(channel);

        // 2 构造请求
        let request = CommonId {
            id: "ae63c5c2-d672-4e18-b78f-d353f870d44c".to_string(),
        };
        // 3 调用 RPC
        let response = client
            .get_user_by_id(request)
            .await
            .expect("User not found!")
            .into_inner();

        // 4 处理响应
        if let Some(user) = response.user {
            println!("User info:");
            println!("  id       = {}", user.id);
            println!("  username = {}", user.username);
            println!("  email    = {}", user.email);
        } else {
            println!("User not found");
        }
    }

    #[tokio::test]
    pub async fn test_create_user() {
        // 1 连接 gRPC 服务
        let channel = Endpoint::from_static("http://127.0.0.1:50051")
            .connect()
            .await
            .unwrap();

        let mut user_grpc = UserServiceGrpcClient::new(channel);
        let request = CreateUserRequest {
            username: "test".to_string(),
            email: "test@tests.com".to_string(),
            display_name: "".to_string(),
            password: None,
            phone: "".to_string(),
            avatar_url: None,
            roles: vec![],
            permissions: vec![],
            metadata: None,
            send_welcome_email: false,
            timezone: "".to_string(),
            locale: "".to_string(),
            require_email_verification: false,
            require_phone_verification: false,
            source: "".to_string(),
        };
        let response = user_grpc
            .create_user(request)
            .await
            .expect("Create user failed")
            .into_inner();
        println!("{:?}", response);
    }
}
