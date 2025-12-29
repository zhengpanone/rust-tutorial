use sqlx::PgPool;
use tracing::log::info;
use crate::config::Config;
use crate::db::postgres::create_pg_pool;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub config: Config,
}

impl AppState {
    pub async fn new(config:Config) -> Result<Self, tonic::transport::Error> {
        let db_pool = create_pg_pool(&config.database)
            .await
            .expect("Failed to connect to DB");
        info!("Database connection established");

        // 运行数据库迁移
        // sqlx::migrate!("./migrations").run(&db_pool).await?;
        Ok(Self {
            db: db_pool,
            config,
        })
    }
}
