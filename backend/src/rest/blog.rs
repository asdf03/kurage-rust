use axum::{ routing::get, Router, response::IntoResponse };
use serde_json::json;
use std::net::SocketAddr;

use crate::usecase::blog::blog_page;

pub fn blog_router() -> Router {
    Router::new()
        .route("/blog/:file_name", get(blog_page))
        .route("/blog/test", get(test))
}

#[derive(serde::Serialize)]
struct SimpleResponse {
    name: String,
    age: u32,
    city: String,
}

pub async fn test() -> impl IntoResponse {
    let response = SimpleResponse {
        name: String::from("test"),
        age: 30,
        city: String::from("tokyo")
    };

    serde_json::to_string(&response).unwrap()
}
