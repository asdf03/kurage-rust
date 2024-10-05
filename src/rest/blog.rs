use axum::{
  routing::get,
  Router
};

use crate::usecase::blog::{
  root_page,
  blog_page
};

pub fn blog_router() -> Router {
  Router::new()
    .route("/", get(root_page))
    .route("/blog/:file_name", get(blog_page))
}
