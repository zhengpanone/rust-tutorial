use crate::entity::user_entity::UserEntity;
use crate::errors::AppError;
use chrono::Utc;
use proto::common::Status;
use proto::user::UserResponse;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct UserDTO {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterDTO {
    username: String,
    password: String,
    email: String,
}

impl TryFrom<&RegisterDTO> for UserEntity {
    type Error = AppError;

    fn try_from(request: &RegisterDTO) -> Result<Self, Self::Error> {
        let now = Utc::now();
        Ok(Self {
            id: Uuid::new_v4().to_string(),
            username: request.username.clone(),
            password_hash: request.password.clone(),
            email: request.email.clone(),
            full_name: "".to_string(),
            phone_number: "".to_string(),
            role: 0,
            created_at: now,
            updated_at: now,
            last_login_at: None,
        })
    }
}

#[derive(Serialize)]
pub struct UserVO {
    pub id: String,
    pub name: String,
}

impl TryFrom<UserEntity> for UserVO {
    type Error = String;

    fn try_from(entity: UserEntity) -> Result<Self, Self::Error> {
        Ok(UserVO {
            id: entity.id,
            name: entity.username,
        })
    }
}

impl TryFrom<UserResponse> for UserVO {
    type Error = String;
    fn try_from(resp: UserResponse) -> Result<Self, Self::Error> {
        if resp.status != Status::Success as i32 {
            return Err(resp.error_message.unwrap_or("unknown error".to_string()));
        }
        let user = resp.user.ok_or("user not found")?;
        Ok(UserVO {
            id: user.id,
            name: user.username,
        })
    }
}
