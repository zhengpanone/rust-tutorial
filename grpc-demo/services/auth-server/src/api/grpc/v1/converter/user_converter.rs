// src/api/grpc/v1/converter/user_converter.rs

use crate::domain::identity::entity::user::User as DomainUser;
use common::enums::user::UserStatus as DomainUserStatus;

use crate::infrastructure::web::dto::user::request::{
    CreateUserRequest as DomainCreateUserRequest, UpdateUserRequest as DomainUpdateUserRequest,
};

use crate::api::grpc::v1::converter::timestamp_converter::TimestampConverter;
use proto::user::{
    CreateUserRequest as ProtoCreateUserRequest, UpdateUserRequest as ProtoUpdateUserRequest,
    User as ProtoUser, UserRole as ProtoUserRole, UserStatus as ProtoUserStatus, UserStatus,
};

/// 用户转换器
pub struct UserConverter;

impl UserConverter {
    /// 转换领域用户到gRPC用户
    pub fn to_proto(user: &DomainUser) -> ProtoUser {
        let metadata: std::collections::HashMap<String, String> = user
            .metadata
            .as_ref()
            .and_then(|v| serde_json::from_value(v.clone()).ok())
            .unwrap_or_default();
        ProtoUser {
            id: user.id.to_string(),
            username: user.username.to_string(),
            email: user.email.to_string(),
            full_name: "".to_string(),
            phone_number: "".to_string(),
            created_at: Some(TimestampConverter::to_proto(user.created_at)),
            updated_at: Some(TimestampConverter::to_proto(user.updated_at)),
            metadata,
            addresses: vec![],
            role: 0,
            tags: vec![],
        }
    }

    /// 转换gRPC用户状态到领域用户状态
    pub fn to_domain_status(status: ProtoUserStatus) -> DomainUserStatus {
        match status {
            ProtoUserStatus::Active => DomainUserStatus::Activate,
            ProtoUserStatus::Inactive => DomainUserStatus::Deactivate,
            ProtoUserStatus::Suspended => DomainUserStatus::Suspended,
            ProtoUserStatus::Deleted => DomainUserStatus::Deleted,
            ProtoUserStatus::Locked => DomainUserStatus::Locked,
            _ => DomainUserStatus::Activate,
        }
    }
}

impl From<DomainUser> for ProtoUser {
    fn from(user: DomainUser) -> Self {
        ProtoUser {
            id: user.id.to_string(),
            username: user.username,
            email: user.email,
            full_name: user.display_name,
            phone_number: user.phone.unwrap_or_default(),
            role: ProtoUserRole::User as i32, // 根据实际角色映射
            metadata: std::collections::HashMap::new(), // 或从 user.metadata 转换
            addresses: vec![],
            tags: vec![],
            created_at: Some(prost_types::Timestamp {
                seconds: user.created_at.timestamp(),
                nanos: user.created_at.timestamp_subsec_nanos() as i32,
            }),
            updated_at: Some(prost_types::Timestamp {
                seconds: user.updated_at.timestamp(),
                nanos: user.updated_at.timestamp_subsec_nanos() as i32,
            }),
        }
    }
}

// From trait 实现：Proto CreateUserRequest -> Domain CreateUserRequest
impl From<&ProtoCreateUserRequest> for DomainCreateUserRequest {
    fn from(req: &ProtoCreateUserRequest) -> Self {
        Self {
            username: req.username.clone(),
            email: req.email.clone(),
            phone: None,
            password: req.password.clone(),
            display_name: req.display_name.clone(),
            avatar_url: req.avatar_url.clone(),
            roles: req.roles.clone(),
            status: Default::default(),
            metadata: None,
        }
    }
}

// From trait 实现：Proto UpdateUserRequest -> Domain UpdateUserRequest
impl From<&ProtoUpdateUserRequest> for DomainUpdateUserRequest {
    fn from(req: &ProtoUpdateUserRequest) -> Self {
        Self {
            username: Some(req.username.clone()),
            email: req.email.clone(),
            phone: None,
            display_name: Some(req.display_name.clone()),
            avatar_url: Some(req.avatar_url.clone()),
            roles: None,
            email_verified: None,
            phone_verified: None,
            status: None,
            metadata: None,
        }
    }
}
