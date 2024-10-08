use axum;

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
