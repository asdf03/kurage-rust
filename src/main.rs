use actix_files::NamedFile;
use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use comrak::{markdown_to_html, ComrakOptions};
use std::fs;
use std::path::Path;

async fn load_template(template_name: &str) -> String {
    let template_path = Path::new("static").join("html").join(template_name);
    fs::read_to_string(template_path).expect(&format!("Failed to read {}", template_name))
}

#[get("/")]
async fn root_page() -> impl Responder {
    let markdown_file_path = Path::new("markdown_files").join("root.md");
    let markdown_input = fs::read_to_string(markdown_file_path).expect("Failed to read markdown file");

    let mut options = ComrakOptions::default();
    options.render.unsafe_ = true;

    let html_output = markdown_to_html(&markdown_input, &options);

    let html_header = load_template("header.html").await.replace("{title}", "Hoge");
    let html_footer = load_template("footer.html").await;
    let html_page = format!(
        "{}{}{}",
        html_header,
        html_output,
        html_footer
    );

    HttpResponse::Ok().content_type("text/html; charset=utf-8").body(html_page)
}

#[get("/blog/{file_name}")]
async fn blog_page(path: web::Path<String>) -> impl Responder {
    let file_name = path.into_inner();
    let markdown_file_path = Path::new("markdown_files").join(format!("{}.md", file_name));
    let markdown_input = match fs::read_to_string(markdown_file_path) {
        Ok(content) => content,
        Err(_) => return HttpResponse::NotFound().body("Markdown file not found"),
    };

    let mut options = ComrakOptions::default();
    options.render.unsafe_ = true;

    let mut html_output = markdown_to_html(&markdown_input, &options);

    html_output = html_output.replace("<h2>", "<h2 class=\"custum-header\">");

    HttpResponse::Ok().content_type("text/html; charset=utf-8").body(html_output)
}

#[get("/static/{file_type}/{file_name}")]
async fn static_files(path: actix_web::web::Path<(String, String)>, req: HttpRequest) -> impl Responder {
    let (file_type, file_name) = path.into_inner();
    let file_path = Path::new("static").join(file_type).join(file_name);

    match NamedFile::open(file_path) {
        Ok(file) => file.into_response(&req),
        Err(_) => HttpResponse::NotFound().body("File not found"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(root_page)
            .service(blog_page)
            .service(static_files)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}