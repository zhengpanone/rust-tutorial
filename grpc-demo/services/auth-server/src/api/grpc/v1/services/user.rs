// api/grpc/v1/user_row

use prost_types::Timestamp;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::app::state::AppState;
use crate::application::services::user_service::UserService;
use proto::user::user_service_grpc_server::UserServiceGrpc;
use proto::user::{
    AssignPermissionRequest, AssignRoleRequest, BatchCreateUsersResponse, CreateUserRequest,
    DeleteUserRequest, DeleteUserResponse, DisableUserRequest, EnableUserRequest, GetUserRequest,
    GetUserStatsRequest, GetUsersRequest, GetUsersResponse, ListUserRequest, LockUserRequest,
    RemovePermissionRequest, RemoveRoleRequest, SearchUsersRequest, SearchUsersResponse,
    StreamUsersRequest, UnlockUserRequest, UpdateUserRequest, User, UserOperation, UserResponse,
    UserRole, UserStatsResponse, VerifyEmailRequest, VerifyPhoneRequest,
};

use crate::infrastructure::web::dto::user::request::{
    CreateUserRequest as DomainCreateUserRequest, UpdateUserRequest as DomainUpdateUserRequest,
};
use proto::common::CommonId;
use tonic::codegen::tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status, Streaming};
use tracing::{info};

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
    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<UserResponse>, Status> {
        let remote_addr = request.remote_addr(); // 获取客户端 IP 地址
        let req = request.into_inner();

        info!(target: "user_grpc", "CreateUser request received from {:?}", remote_addr);

        let domain_req: DomainCreateUserRequest = (&req).into();

        let user = self
            .service
            .create_user(domain_req)
            .await
            .map_err(|err| err.to_tonic_status())?;
        let proto_user: User = user.into();
        Ok(Response::new(UserResponse {
            user: Some(proto_user),
            response: None,
            error_message: None,
        }))
    }

    async fn get_user(
        &self,
        request: Request<GetUserRequest>,
    ) -> Result<Response<UserResponse>, Status> {
        let req = request.into_inner();
        Ok(tonic::Response::new(UserResponse {
            user: Some(User {
                id: "1".to_string(),
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
            response: None,
            error_message: None,
        }))
    }

    async fn get_user_by_id(
        &self,
        request: Request<CommonId>,
    ) -> Result<Response<UserResponse>, Status> {
        let user_id = request.into_inner().id;
        info!(target: "user_grpc", "GetUserById request received from {:?}", user_id);
        let user_opt = self
            .service
            .get_user(&user_id)
            .await
            .map_err(|err| err.to_tonic_status())?;

        let response = match user_opt {
            Some(user) => UserResponse {
                user: Some(user.into()),
                response: None,
                error_message: None,
            },
            None => UserResponse {
                user: None,
                response: None,
                error_message: Some(format!("User not found: {}", user_id)),
            },
        };

        Ok(Response::new(response))
    }

    async fn get_users(
        &self,
        request: Request<GetUsersRequest>,
    ) -> Result<Response<GetUsersResponse>, Status> {
        todo!()
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
                    response: None,
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
        let req = request.into_inner();

        let domain_req: DomainUpdateUserRequest = (&req).into();
        let user_id = req.user_id.as_str();

       let user =  self.service
            .update_user(user_id, domain_req)
            .await
            .map_err(|err| err.to_tonic_status())?;

        Ok(Response::new(UserResponse {
            user: None,
            error_message: None,
            response: None,
        }))
    }

    async fn delete_user(
        &self,
        request: Request<DeleteUserRequest>,
    ) -> Result<Response<DeleteUserResponse>, Status> {
        let user_id = request.into_inner().user_id;

        Ok(Response::new(DeleteUserResponse {
            success: false,
            message: "".to_string(),
        }))
    }

    async fn search_users(
        &self,
        request: Request<SearchUsersRequest>,
    ) -> Result<Response<SearchUsersResponse>, Status> {
        todo!()
    }

    async fn get_user_stats(
        &self,
        request: Request<GetUserStatsRequest>,
    ) -> Result<Response<UserStatsResponse>, Status> {
        todo!()
    }

    async fn enable_user(
        &self,
        request: Request<EnableUserRequest>,
    ) -> Result<Response<UserResponse>, Status> {
        todo!()
    }

    async fn disable_user(
        &self,
        request: Request<DisableUserRequest>,
    ) -> Result<Response<UserResponse>, Status> {
        todo!()
    }

    async fn lock_user(
        &self,
        request: Request<LockUserRequest>,
    ) -> Result<Response<UserResponse>, Status> {
        todo!()
    }

    async fn unlock_user(
        &self,
        request: Request<UnlockUserRequest>,
    ) -> Result<Response<UserResponse>, Status> {
        todo!()
    }

    async fn assign_role(
        &self,
        request: Request<AssignRoleRequest>,
    ) -> Result<Response<UserResponse>, Status> {
        todo!()
    }

    async fn remove_role(
        &self,
        request: Request<RemoveRoleRequest>,
    ) -> Result<Response<UserResponse>, Status> {
        todo!()
    }

    async fn assign_permission(
        &self,
        request: Request<AssignPermissionRequest>,
    ) -> Result<Response<UserResponse>, Status> {
        todo!()
    }

    async fn remove_permission(
        &self,
        request: Request<RemovePermissionRequest>,
    ) -> Result<Response<UserResponse>, Status> {
        todo!()
    }

    async fn verify_email(
        &self,
        request: Request<VerifyEmailRequest>,
    ) -> Result<Response<UserResponse>, Status> {
        todo!()
    }

    async fn verify_phone(
        &self,
        request: Request<VerifyPhoneRequest>,
    ) -> Result<Response<UserResponse>, Status> {
        todo!()
    }

    type StreamUsersStream = ReceiverStream<Result<UserResponse, Status>>;

    async fn stream_users(
        &self,
        request: Request<StreamUsersRequest>,
    ) -> Result<Response<Self::StreamUsersStream>, Status> {
        todo!()
    }

    async fn batch_create_users(
        &self,
        request: Request<Streaming<CreateUserRequest>>,
    ) -> Result<Response<BatchCreateUsersResponse>, Status> {
        todo!()
    }

    type BidirectionalStreamUsersStream = ReceiverStream<Result<UserResponse, Status>>;

    async fn bidirectional_stream_users(
        &self,
        request: Request<Streaming<UserOperation>>,
    ) -> Result<Response<Self::BidirectionalStreamUsersStream>, Status> {
        todo!()
    }
}
