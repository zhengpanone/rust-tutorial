pub mod permission;
pub mod role;
pub mod user;

// 重导出类型，使其在 enums 模块级别可见 UserStatus 可以通过 common::enums::UserStatus 直接访问
// pub use permission::PermissionType;
// pub use role::{RoleStatus, RoleType};
// pub use user::UserStatus;
