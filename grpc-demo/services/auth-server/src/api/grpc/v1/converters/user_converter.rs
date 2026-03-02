// src/api/grpc/v1/converters/user_converter.rs

use crate::domain::identity::entity::user::User as DomainUser;
use crate::infrastructure::web::dto::user::request::{
    CreateUserRequest as DomainCreateUserRequest, UpdateUserRequest as DomainUpdateUserRequest,
};
use proto::user;
use proto::user::{User as ProtoUser, UserRole as ProtoUserRole};

impl From<DomainUser> for ProtoUser {
    fn from(user: DomainUser) -> Self {
        ProtoUser {
            id: user.id.to_string(),
            username: user.username,
            email: user.email,
            full_name: user.display_name,
            phone_number: user.phone.unwrap_or_default(),
            role: user::UserRole::User as i32, // 根据实际角色映射
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
impl From<&user::CreateUserRequest> for DomainCreateUserRequest {
    fn from(req: &user::CreateUserRequest) -> Self {
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
impl From<&user::UpdateUserRequest> for DomainUpdateUserRequest {
    fn from(req: &user::UpdateUserRequest) -> Self {
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
