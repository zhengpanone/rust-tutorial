use crate::entity::user_entity::UserEntity;
use proto::common::Status;
use proto::user::UserResponse;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct UserDTO {
    pub id: String,
    pub name: String,
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
