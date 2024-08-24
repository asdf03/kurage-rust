use axum::{
  routing::get,
  Router
};

use crate::usecase::auth_usecase::{
  show_login_form,
  handle_login
};

pub fn create_auth_router() -> Router {
  Router::new()
    .route("/login", get(show_login_form).post(handle_login))
}
