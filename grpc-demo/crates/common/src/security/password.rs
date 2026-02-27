use crate::error::{AppError, AppResult};
use argon2::password_hash::rand_core::OsRng;
use argon2::{
    Argon2, PasswordHash,
    password_hash::{PasswordHasher, PasswordVerifier, SaltString},
};
use tracing::error;

/// 哈希密码
pub fn hash_password(password: &str) -> AppResult<String> {
    // 1. 创建 Argon2 实例（默认使用 Argon2id）
    let argon2 = Argon2::default();

    // 2. 随机生成盐
    let salt = SaltString::generate(&mut OsRng);

    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| {
            error!("Failed to hash password: {}", e);
            AppError::Hashing(e.to_string())
        })?
        .to_string();

    Ok(password_hash)
}

/// 验证密码
pub fn verify_password(password: &str, password_hash: &str) -> AppResult<bool> {
    let parsed_hash = PasswordHash::new(password_hash).map_err(|e| {
        error!("Failed to parse password hash: {}", e);
        AppError::Hashing(e.to_string())
    })?;

    let is_valid = Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok();
    Ok(is_valid)
}
#[cfg(test)]
mod tests {
    use crate::security::password::{hash_password, verify_password};

    #[test]
    fn test_password_hash() {
        let password = "admin123";
        let password_hash = hash_password(password).unwrap();
        assert!(verify_password(password, &password_hash).unwrap());
    }
}
