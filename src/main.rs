use actix_web::{guard, web::{self, scope}, App, HttpResponse, HttpServer, Responder};
use tokio::io::AsyncBufRead;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/")
                    // curl -H "Host: www.rust-lang.org" http://127.0.0.1:8080
                    .guard(guard::Header("Host", "www.rust-lang.org"))
                    .route("", web::to(|| async { HttpResponse::Ok().body("www") })),
            )
            .service(
                web::scope("/")
                    // curl -H "Host: users.rust-lang.org" http://127.0.0.1:8080
                    .guard(guard::Header("Host", "users.rust-lang.org"))
                    .route("", web::to(|| async { HttpResponse::Ok().body("user") }))
            )
            .route("/", web::to(HttpResponse::Ok))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}