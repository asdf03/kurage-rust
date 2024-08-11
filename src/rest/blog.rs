use axum::{routing::get, Router};
use crate::usecase::blog_usecase;

pub fn router() -> Router {
  Router::new()
    .route("/", get(blog_usecase::root_page))
    .route("/:file_name", get(blog_usecase::blog_page))
}