pub mod domain;
pub mod rest;
pub mod infrastructure;
pub mod interface;
pub mod usecase;

use axum::Router;
use crate::rest::blog::create_blog_router;
use crate::rest::static_files::create_static_router;

pub fn create_router() -> Router {
    Router::new()
        .merge(create_blog_router())
        .merge(create_static_router())
}