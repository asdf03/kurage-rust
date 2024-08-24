use axum::{
  routing::get,
  Router
};

use crate::usecase::blog_usecase::{
  root_page,
  blog_page
};

pub fn create_blog_router() -> Router {
  Router::new()
    .route("/", get(root_page))
    .route("/blog/:file_name", get( ))
}
