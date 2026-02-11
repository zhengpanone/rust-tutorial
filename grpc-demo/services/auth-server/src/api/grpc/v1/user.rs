use std::sync::Arc;
// grpc/user_handler
use prost_types::Timestamp;
use std::time::{SystemTime, UNIX_EPOCH};


use proto::common;
use proto::user::user_service_grpc_server::UserServiceGrpc;
use proto::user::{
    CreateUserRequest, DeleteUserRequest, DeleteUserResponse, GetUserRequest, ListUserRequest,
    LoginRequest, LoginResponse, RegisterRequest, RegisterResponse, UpdateUserRequest, User,
    UserResponse, UserRole, VerifyTokenRequest, VerifyTokenResponse,
};
use tonic::codegen::tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};
use crate::app::state::AppState;
use crate::application::services::user_service::UserService;

fn now_timestamp() -> Timestamp {
    let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    Timestamp {
        seconds: duration.as_secs() as i64,
        nanos: duration.subsec_nanos() as i32,
    }
}


pub fn grpc_user_service(state: Arc<AppState>) -> GrpcUserService {
    GrpcUserService::new(state.user_service.clone())
}

// gRPC adapter
pub struct GrpcUserService {
    service: Arc<dyn UserService>,
}
impl GrpcUserService {
    pub fn new(service: Arc<dyn UserService>) -> Self {
        Self { service }
    }
}

#[tonic::async_trait]
impl UserServiceGrpc for GrpcUserService {
    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        let _req = request.into_inner();
        Ok(Response::new(RegisterResponse {
            username: "demo".into(),
            email: "demo@tests.com".to_string(),
            full_name: "Demo User".to_string(),
            phone_number: "15527300572".to_string(),
            access_token: "access token".to_string(),
            refresh_token: "refresh token".to_string(),
            expires_at: 0,
        }))
    }

    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        let _req = request.into_inner();
        Ok(Response::new(LoginResponse::default()))
    }

    async fn verify_token(
        &self,
        request: Request<VerifyTokenRequest>,
    ) -> Result<Response<VerifyTokenResponse>, Status> {
        let _req = request.into_inner();
        Ok(Response::new(VerifyTokenResponse::default()))
    }

    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<UserResponse>, Status> {
        let _req = request.into_inner();
        Ok(Response::new(UserResponse::default()))
    }

    async fn get_user(
        &self,
        request: Request<GetUserRequest>,
    ) -> Result<Response<UserResponse>, Status> {
        let id = request.into_inner().id;
        Ok(tonic::Response::new(UserResponse {
            user: Some(User {
                id,
                username: "demo".to_string(),
                email: "demo@tests.com".to_string(),
                full_name: "Demo User".to_string(),
                phone_number: "15527300572".to_string(),
                role: 0,
                metadata: Default::default(),
                addresses: vec![],
                tags: vec![],
                created_at: Some(now_timestamp()),
                updated_at: Some(now_timestamp()),
            }),
            status: 0,
            error_message: None,
        }))
    }

    type ListPageUserStream = ReceiverStream<Result<UserResponse, Status>>;

    async fn list_page_user(
        &self,
        request: Request<ListUserRequest>,
    ) -> Result<Response<Self::ListPageUserStream>, Status> {
        let req = request.into_inner();
        // 这里你可以读取分页参数
        let (page, page_size) = if let Some(ref p) = req.page {
            (p.page, p.page_size)
        } else {
            (1, 10)
        };
        // let page = req.page.as_ref().map(|p| p.page).unwrap_or(1);
        // let page_size = req.page.as_ref().map(|p| p.page_size).unwrap_or(10);

        // channel：buffer 大小根据业务调整
        let (tx, rx) = tokio::sync::mpsc::channel(16);

        // 异步生产数据
        tokio::spawn(async move {
            for i in 0..page_size {
                let user = User {
                    id: format!("{}_{}", page, i),
                    username: format!("user_{}", i),
                    email: format!("user{}@tests.com", i),
                    full_name: format!("User {}", i),
                    phone_number: "".into(),
                    role: UserRole::User as i32,
                    metadata: Default::default(),
                    addresses: vec![],
                    tags: vec!["demo".into()],
                    created_at: Some(now_timestamp()),
                    updated_at: Some(now_timestamp()),
                };

                let resp = UserResponse {
                    user: Some(user),
                    status: common::Status::Success as i32,
                    error_message: None,
                };

                // 如果客户端断开，send 会失败
                if tx.send(Ok(resp)).await.is_err() {
                    break;
                }
            }
        });
        Ok(Response::new(ReceiverStream::new(rx)))
    }

    async fn update_user(
        &self,
        request: Request<UpdateUserRequest>,
    ) -> Result<Response<UserResponse>, Status> {
        let _update_user = request.into_inner();
        Ok(Response::new(UserResponse {
            user: None,
            status: 0,
            error_message: None,
        }))
    }

    async fn delete_user(
        &self,
        request: Request<DeleteUserRequest>,
    ) -> Result<Response<DeleteUserResponse>, Status> {
        let _id = request.into_inner().id;

        Ok(Response::new(DeleteUserResponse {
            success: false,
            message: "".to_string(),
        }))
    }
}
