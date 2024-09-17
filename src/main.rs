use kurage_rust::rest::blog::create_blog_router;
use kurage_rust::rest::static_files::create_static_router;


use axum::{
	extract::Path as AxumPath, response::Html, routing::{get, post}, Extension, Router
};
use comrak::{markdown_to_html, ComrakOptions};
use std::{fs, path::Path, sync::{Arc, Mutex}};

#[tokio::main]
async fn main() {

	let app = Router::new()
		.merge(create_blog_router())
		.merge(create_static_router());
	
	let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
		.await
		.unwrap();

	axum::serve(listener, app)
		.await
		.unwrap();
}
