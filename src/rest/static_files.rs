use axum::{
  routing::get,
  Router
};
use crate::usecase::static_usecase::static_files;

pub fn create_static_router() -> Router {
  Router::new()
    .route("/static/:file_type/:file_name", get(static_files))
}