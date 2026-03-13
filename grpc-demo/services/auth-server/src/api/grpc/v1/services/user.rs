// api/grpc/v1/user_row

use super::interceptor::GrpcContext;
use crate::app::state::AppState;
use crate::application::services::user_service::UserService;
use metrics::{counter, histogram};
use prost_types::Timestamp;
use proto::user::user_service_grpc_server::UserServiceGrpc;
use proto::user::{
    AssignPermissionRequest, AssignRoleRequest, BatchCreateUsersResponse, CreateUserRequest,
    DeleteUserRequest, DeleteUserResponse, DisableUserRequest, EnableUserRequest, GetUserRequest,
    GetUserStatsRequest, GetUsersRequest, GetUsersResponse, ListUserRequest, LockUserRequest,
    RemovePermissionRequest, RemoveRoleRequest, SearchUsersRequest as ProtoSearchUsersRequest,
    SearchUsersRequest, SearchUsersResponse as ProtoSearchUsersResponse, StreamUsersRequest,
    UnlockUserRequest, UpdateUserRequest, User, UserOperation, UserResponse, UserRole,
    UserStatsResponse, VerifyEmailRequest, VerifyPhoneRequest,
};
use std::sync::Arc;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use crate::api::grpc::v1::converter::user_converter::UserConverter;
use crate::infrastructure::web::dto::user::request::{
    CreateUserRequest as DomainCreateUserRequest, UpdateUserRequest as DomainUpdateUserRequest,
    UserFilter,
};
use common::web::pagination::{PaginationInfo, PaginationParams};
use proto::common::{
    CommonId, PageRequest as ProtoPageRequest, PageResponse as ProtoPageResponse, PageResponse,
};
use tonic::codegen::tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status, Streaming};
use tracing::{debug, info, instrument};

fn now_timestamp() -> Timestamp {
    let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    Timestamp {
        seconds: duration.as_secs() as i64,
        nanos: duration.subsec_nanos() as i32,
    }
}

pub fn grpc_user_service(state: Arc<AppState>) -> UserGrpcService {
    UserGrpcService::new(state.user_service.clone())
}

// 用户gRPC服务 gRPC adapter
pub struct UserGrpcService {
    user_service: Arc<dyn UserService>,
}

impl UserGrpcService {
    pub fn new(user_service: Arc<dyn UserService>) -> Self {
        Self { user_service }
    }

    /// 记录指标
    fn record_metrics(&self, method: &str, success: bool, duration_ms: f64) {
        let success_str = if success {
            "true".to_string()
        } else {
            "false".to_string()
        };
        counter!("grpc_requests_total","method" => method.to_string(), "success" => success_str)
            .increment(1);
        histogram!("grpc_request_duration_ms", "method" => method.to_string()).record(duration_ms);

        if !success {
            counter!("grpc_errors_total", "method" => method.to_string()).increment(1);
        }
    }

    /// 转换为领域分页
    fn convert_pagination(&self, request: &ProtoPageRequest) -> PaginationParams {
        PaginationParams {
            page: request.page,
            page_size: request.page_size,
            sort_by: None,
            sort_order: None,
            search: None,
            filters: None,
        }
    }

    /// 获取gRPC上下文
    fn get_grpc_context<T>(&self, request: &Request<T>) -> Result<GrpcContext, Status> {
        request
            .extensions()
            .get::<GrpcContext>()
            .cloned()
            .ok_or_else(|| Status::unauthenticated("Missing authentication context"))
    }
}

#[tonic::async_trait]
impl UserServiceGrpc for UserGrpcService {
    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<UserResponse>, Status> {
        let remote_addr = request.remote_addr(); // 获取客户端 IP 地址
        let req = request.into_inner();

        info!(target: "user_grpc", "CreateUser request received from {:?}", remote_addr);

        let domain_req: DomainCreateUserRequest = (&req).into();

        let user = self
            .user_service
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
        let _req = request.into_inner();
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
            .user_service
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

        let _user = self
            .user_service
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
        let _user_id = request.into_inner().user_id;

        Ok(Response::new(DeleteUserResponse {
            success: false,
            message: "".to_string(),
        }))
    }

    #[instrument(name = "grpc_search_users", skip_all)]
    async fn search_users(
        &self,
        request: Request<ProtoSearchUsersRequest>,
    ) -> Result<Response<ProtoSearchUsersResponse>, Status> {
        let start_time = Instant::now();
        debug!(
            "Searching users via gRPC, request received from {:?}",
            request
        );
        let ctx = self.get_grpc_context(&request);
        let request_data = request.into_inner();
        // 转换分页
        let pagination = if let Some(req_page) = request_data.pagination {
            self.convert_pagination(&req_page)
        } else {
            PaginationParams::default()
        };
        // 构建 UserFilter
        let filter = UserFilter {
            status: None,
            role: None,
            email_verified: None,
            phone_verified: None,
            created_after: None,
            created_before: None,
            search: Some(request_data.query),
        };

        // 调用应用服务
        let user_service = self.user_service.clone();
        let result = user_service.list_users(filter, pagination).await;
        // 转换为gRPC响应
        let result = result.map_err(|err| err.to_tonic_status())?;
        let response = ProtoSearchUsersResponse {
            users: result
                .items
                .into_iter()
                .map(|user| UserConverter::to_proto(&user))
                .collect(),
            pagination: Some(ProtoPageResponse {
                page: result.page,
                page_size: result.page_size,
                total: result.total,
                total_pages: result.total_pages,
                has_next: result.has_next,
                has_previous: result.has_previous,
            }),
            total_hits: 0,
            response: None,
        };

        Ok(Response::new(response))
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
