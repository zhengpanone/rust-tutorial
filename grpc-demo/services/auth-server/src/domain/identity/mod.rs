pub mod error;
pub mod entity;
pub mod repository;
pub mod services;
pub mod value_objects;

// 重导出常用类型
pub use value_objects::{Email, Username};
pub use error::IdentityError;
