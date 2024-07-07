use crate::domain::user::{RegisterForm, LoginForm};
use axum::{Extension, Form, http::StatusCode, response::IntoResponse};

pub async fn register(
    Form(form): Form<RegisterForm>,
    Extension(db): Extension<Arc<Mutex<PgPool>>>
) -> impl IntoResponse {
    let email_hash = hash_password(&form.email);
    let password_hash = hash_password(&form.password);
    let user_id = uuid::Uuid::new_v4().to_String();

    insert_user(&db.lock().await, &user_id, &email_hash, &password_hash).await;

    StatusCode::CREATED
}