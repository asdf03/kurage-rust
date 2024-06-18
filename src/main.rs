use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use comrak::{markdown_to_html, ComrakOptions};
use std::fs;
use std::path::Path;

#[get("/")]
async fn root_page() -> impl Responder {
    let markdown_file_path = Path::new("markdown_files").join("root.md");
    let markdown_input = fs::read_to_string(markdown_file_path).expect("Failed to read markdown file");

    let mut options = ComrakOptions::default();
    options.render.unsafe_ = true;

    let mut html_output = markdown_to_html(&markdown_input, &options);

    html_output = html_output.replace("<h2>", "<h2 class=\"custum-header\">");

    HttpResponse::Ok().content_type("text/html; charset=utf-8").body(html_output)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(root_page)
            // .service(static_fies)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}