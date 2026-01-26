// src/app/bootstrap/database.rs

use crate::app::config::config::DatabaseConfig;
use crate::app::state::AppState;
use common::error::{AppError, AppResult};
use futures::TryFutureExt;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use tracing::{error, info};

pub async fn init_database(config: &DatabaseConfig) -> AppResult<PgPool> {
    info!("Initializing database connection pool...");

    let pool = PgPoolOptions::new()
        .max_connections(config.max_connections)
        .connect(&config.url)
        .await
        .map_err(|e| {
            error!("Failed to connect to database: {}", e);
            e
        })?;

    // 测试连接
    sqlx::query("SELECT 1").execute(&pool).await.map_err(|e| {
        error!("Failed to execute query: {}", e);
        e
    })?;

    // 运行数据库迁移
    if config.run_migrations {
        run_migrations(&pool).await?
    }
    info!("Database connection pool initialized successfully");
    Ok(pool)
}

/// 运行数据库迁移
async fn run_migrations(pool: &PgPool) -> AppResult<()> {
    info!("Running database migrations...");
    sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .map_err(|e| {
            error!("Failed to run migrations: {}", e);
            e
        })?;
    info!("Database migrations completed successfully");
    Ok(())
}
