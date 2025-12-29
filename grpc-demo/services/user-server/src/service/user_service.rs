use crate::entity::user_entity::UserEntity;
use crate::repository::user_repository::UserRepository;
use crate::schemas::user::RegisterDTO;
use crate::state::AppState;
use anyhow::anyhow;
use std::sync::Arc;

#[derive(Clone)]
pub struct UserService {
    state: Arc<AppState>,
    repo: UserRepository,
}

impl UserService {
    pub fn new(state: Arc<AppState>) -> Self {
        let repository = UserRepository::new(state.db.clone());
        Self {
            state,
            repo: repository,
        }
    }

    pub async fn get_user_by_id(&self, id: &str) -> anyhow::Result<Option<UserEntity>> {
        let user = self.repo.find_by_id(id).await?;
        Ok(user)
    }

    pub async fn create_user(&self, register: &RegisterDTO) -> anyhow::Result<UserEntity> {
        let user_entity: UserEntity = register
            .try_into()
            .map_err(|_| anyhow!("Failed to convert DTO to entity"))?;
        self.repo
            .create_user(user_entity)
            .await
            .map_err(|e| anyhow!("Failed to create user: {}", e))
    }
}
