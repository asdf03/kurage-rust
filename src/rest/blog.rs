use axum::{
  routing::get,
  Router
};

use crate::usecase::blog::blog_page;

pub fn blog_router() -> Router {
  Router::new()
    .route("/blog/:file_name", get(blog_page))
}
