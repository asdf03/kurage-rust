use axum::{
    routing::get,
    Router
};

use crate::usecase::root::root_page;

pub fn root_router() -> Router {
    Router::new()
        .route("/", get(root_page))
}