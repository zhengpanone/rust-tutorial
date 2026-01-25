use std::sync::Arc;

use async_trait::async_trait;
use sqlx::{Error as SqlxError, PgPool};

use crate::{
    models::service::Service, repository::base_repository::BaseRepository,
};



#[derive(Clone)]
pub struct ServiceRepository {
    base: BaseRepository<Service>,
}

impl ServiceRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self {
            base: BaseRepository::new(pool),
        }
    }
}

#[async_trait]
impl ServiceRepositoryTrait for ServiceRepository {
    fn create(&self, service: Service) -> Result<Service, RepositoryError> {
        todo!()
    }

    fn find_by_id(&self, id: i32) -> Result<Option<Service>, RepositoryError> {
        todo!()
    }

    fn find_by_service_code(&self, service_code: &str) -> Result<Option<Service>, RepositoryError> {
        todo!()
    }

    fn find_all(&self) -> Result<Vec<Service>, RepositoryError> {
        todo!()
    }

    fn update(&self, service: Service) -> Result<Service, RepositoryError> {
        todo!()
    }

    fn delete(&self, id: i32) -> Result<(), RepositoryError> {
        todo!()
    }

    fn exists_by_service_code(&self, service_code: &str) -> Result<bool, RepositoryError> {
        todo!()
    }
}
