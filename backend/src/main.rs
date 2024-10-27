use axum::http::{HeaderValue, Method};
// use http::{Method, HeaderValue};

use kurage_rust::create_router;
use tower_http::cors::{Any, CorsLayer, AllowMethods, AllowOrigin};
use tower::ServiceBuilder;

#[tokio::main]
async fn main() {

	let cors = CorsLayer::new()
		.allow_methods(AllowMethods::list([Method::GET]))
		.allow_origin(AllowOrigin::list(vec!["http://localhost:3000".parse().unwrap()]));

	let app = create_router().layer(cors);
	
	let listener = tokio::net::TcpListener::bind("0.0.0.0:8000")
		.await
		.unwrap();

	axum::serve(listener, app)
		.await
		.unwrap();
}
