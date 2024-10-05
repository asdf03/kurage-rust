use axum::{
	extract::Path as AxumPath, response::Html, routing::{get, post}, Extension, Router
};
use comrak::{markdown_to_html, ComrakOptions};
use std::{fs, path::Path, sync::{Arc, Mutex}};

use kurage_rust::create_router;

#[tokio::main]
async fn main() {

	let app = create_router();
	
	let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
		.await
		.unwrap();

	axum::serve(listener, app)
		.await
		.unwrap();
}
