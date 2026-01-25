use crate::config::database::DatabaseConfig;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

pub async fn create_pg_pool(db_config: &DatabaseConfig) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(db_config.max_connections)
        .connect(&db_config.url)
        .await
}
