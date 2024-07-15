use crate::domain::user::{RegisterForm, LoginForm};
use axum::{Extension, Form, http::StatusCode, response::IntoResponse};
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::Mutex;

fn hash_password(password: &str) -> String {
    format!("{:x}", md5::compute(password))
}

pub async fn register(
    Form(form): Form<RegisterForm>,
    Extension(db): Extension<Arc<Mutex<PgPool>>>
) -> impl IntoResponse {
    let email_hash = hash_password(&form.email);
    let password_hash = hash_password(&form.password);
    let user_id = uuid::Uuid::new_v4().to_String();

    // match insert_user(&db.lock().await, &user_id, &email_hash, &password_hash).await {
    //     Ok(_) => StatusCode::CREATED,
    //     Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    // }
}
