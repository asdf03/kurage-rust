pub mod domain;
pub mod rest;
pub mod infrastructure;
pub mod interface;
pub mod usecase;

use axum::Router;
use crate::rest::blog::blog_router;
use crate::rest::static_file::static_file_router;

pub fn create_router() -> Router {
    Router::new()
        .merge(blog_router())
        .merge(static_file_router())
}