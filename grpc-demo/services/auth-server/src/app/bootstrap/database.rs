// src/app/bootstrap/database.rs

use sqlx::PgPool;
use crate::app::config::config::DatabaseConfig;

pub async fn init_database(config: &DatabaseConfig)->AppResult<PgPool> {
    let pool = create_pool(config.url)?;
    AppState::set_database_pool(pool);
    Ok(())
}