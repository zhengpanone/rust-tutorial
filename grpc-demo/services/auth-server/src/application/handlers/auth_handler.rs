use crate::app::state::AppState;
use crate::infrastructure::web::dto::auth::request::LoginRequest;
use axum::Json;
use axum::extract::State;
use std::sync::Arc;

pub async fn login(State(state): State<Arc<AppState>>, Json(payload): Json<LoginRequest>) {
    todo!()
}

pub async fn register() {
    todo!()
}

pub async fn refresh_token() {
    todo!()
}

pub async fn forgot_password() {
    todo!()
}

pub async fn reset_password() {
    todo!()
}

pub async fn verify_email() {
    todo!()
}

pub async fn resend_verification() {}

pub async fn logout() {
    todo!()
}

pub async fn get_current_user() {
    todo!()
}

pub async fn change_password() {
    todo!()
}

pub async fn get_user_sessions() {
    todo!()
}

pub async fn revoke_user_session() {
    todo!()
}

pub async fn update_current_user() {
    todo!()
}
