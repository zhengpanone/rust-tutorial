// use std::sync::Arc;
// use lapin::Connection;
// use common::error::AppResult;
// use crate::app::config::config::RabbitmqConfig;
//
// /// 初始化RabbitMQ连接
// pub async fn init_rabbitmq(config: &Option<RabbitmqConfig>) -> AppResult<Option<Arc<Connection>>> {
//     match config {
//         Some(cfg) => {
//             tracing::info!("Initializing RabbitMQ connection...");
//
//             let options = lapin::ConnectionProperties::default()
//                 .with_executor(tokio::runtime::Handle::current())
//                 .with_connection_name(cfg.connection_name.clone().into());
//
//             let connection = lapin::Connection::connect(&cfg.url, options)
//                 .await
//                 .map_err(|e| {
//                     tracing::error!("Failed to connect to RabbitMQ: {}", e);
//                     e
//                 })?;
//
//             tracing::info!("RabbitMQ connection initialized successfully");
//             Ok(Some(Arc::new(connection)))
//         }
//         None => {
//             tracing::warn!("RabbitMQ is not configured, skipping initialization");
//             Ok(None)
//         }
//     }
// }