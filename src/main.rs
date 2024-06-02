mod domain;
mod usecase;
mod infrastructure;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    infrastructure::web::start_server().await
}