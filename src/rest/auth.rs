use std::sync::{Arc, Mutex};

use axum::{response::IntoResponse, Extension, Form};
use sqlx::PgPool;

pub struct RegisterForm {
    pub email: String,
    pub password: String,
}

pub async fn auth_register(
    Form(form): Form<RegisterForm>,
) -> impl IntoResponse {
    // let email_hash = hash_password(&form.email);
    // let password_hash = hash_password(&form.password);
    // let user_id = uuid::Uuid::new_v4().to_String();

    // match insert_user(&db.lock().await, &user_id, &email_hash, &password_hash).await {
    //     Ok(_) => StatusCode::CREATED,
    //     Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    // }
    String::from("hoge")
}