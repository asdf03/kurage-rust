use actix_web::{web, App, HttpResponse, HttpServer};

fn scope_config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/test")
        .route(web::get().to(|| async { HttpResponse::Ok().body("test") }))
        .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}

fn config(cfg: &mut web::ServiceConfig) {
    cfg.service( web::resource("/app")
        .route(web::get().to(|| async { HttpResponse::Ok().body("app") }))
        .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(config)
            .service(web::scope("/api").configure(scope_config))
            .route(
                "/",
                web::get().to(|| async { HttpResponse::Ok().body("/") }),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await

    // Path with `http://localhost:8080/`
    // Path with `http://localhost:8080/app`
    // Path with `http://localhost:8080/api/test`
}