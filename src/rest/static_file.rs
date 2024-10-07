use axum::{ routing::get, Router };
use crate::usecase::static_file::static_file;

pub fn static_file_router() -> Router {
    Router::new().route("/static/:file_type/:file_name", get(static_file))
}
