use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use crate::usecase::fetch_data::convert_domain;

//////////////////////////////////
// apiを叩いてJSONが来るという想定
//////////////////////////////////
async fn fetch_api() -> String {
    r#"
    {
        "id": "Hoge_id"
    }
    "#.to_string()
}

//////////////////////////////////
// apiを叩く関数
//////////////////////////////////
async fn get_hoge_id() -> impl Responder {
    let json_str = fetch_api().await;

    match convert_domain(json_str) {
        Ok(json_domain) => HttpResponse::Ok().body(json_domain.value),
        Err(_) =>  HttpResponse::InternalServerError().body("Failed to convert to domain"),
    }
}

//////////////////////////////////
// サーバーを建てる関数
//////////////////////////////////
pub async fn start_server() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_hoge_id))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}