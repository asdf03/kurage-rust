use axum::{routing::get, Router};

pub fn router() -> Router {
  Router::new()
    .route("/", get(root_page))
    .route("/blog/:file_name", get(blog_page))
}

pub async fn root_page() -> &'static str {
  // Todo
  "root_page"
}

pub async fn blog_page() -> &'static str {
  // Todo
  "blog_page"
}