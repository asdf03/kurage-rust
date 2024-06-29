use axum::{
    routing::get,
    Router,
    response::Html,
    extract::Path as AxumPath,
};
use comrak::{markdown_to_html, ComrakOptions};
use std::{fs, path::Path};
use tower::ServiceBuilder;


async fn load_template(template_name: &str) -> String {
    let template_path = Path::new("static").join("html").join(template_name);
    fs::read_to_string(template_path).expect(&format!("Failed to read {}", template_name))
}

async fn root_page() -> Html<String> {
    let markdown_file_path = Path::new("markdown_files").join("root.md");
    let markdown_input = fs::read_to_string(markdown_file_path).expect("Failed to read markdown file");

    let mut options = ComrakOptions::default();
    options.render.unsafe_ = true;

    let html_output = markdown_to_html(&markdown_input, &options);

    let html_header = load_template("header.html").await.replace("{title}", "Hoge");
    let html_footer = load_template("footer.html").await;
    let html_page = format!("{}{}{}", html_header, html_output, html_footer);

    Html(html_page)
}

async fn static_files(AxumPath((file_type, file_name)): AxumPath<(String, String)>) -> axum::http::Response<axum::body::Body> {
    let file_path = Path::new("static").join(file_type).join(file_name);
    if let Ok(file_content) = tokio::fs::read(&file_path).await {
        axum::http::Response::builder()
            .status(axum::http::StatusCode::OK)
            .header(axum::http::header::CONTENT_TYPE, mime_guess::from_path(&file_path).first_or_octet_stream().as_ref())
            .body(axum::body::Body::from(file_content))
            .expect("Failed to construct response")
    } else {
        axum::http::Response::builder()
            .status(axum::http::StatusCode::NOT_FOUND)
            .body(axum::body::Body::empty())
            .expect("Failed to construct response")
    }
}


#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root_page))
        .route("/static/:file_type/:file_name", get(static_files));
        // .route("/blog/:file_name", get(static_files))
    
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

// use comrak::{markdown_to_html, ComrakOptions};
// use std::fs;
// use std::path::Path;

// async fn load_template(template_name: &str) -> String {
//     let template_path = Path::new("static").join("html").join(template_name);
//     fs::read_to_string(template_path).expect(&format!("Failed to read {}", template_name))
// }

// #[get("/")]
// async fn root_page() -> impl Responder {
//     let markdown_file_path = Path::new("markdown_files").join("root.md");
//     let markdown_input = fs::read_to_string(markdown_file_path).expect("Failed to read markdown file");

//     let mut options = ComrakOptions::default();
//     options.render.unsafe_ = true;

//     let html_output = markdown_to_html(&markdown_input, &options);

//     let html_header = load_template("header.html").await.replace("{title}", "Hoge");
//     let html_footer = load_template("footer.html").await;
//     let html_page = format!(
//         "{}{}{}",
//         html_header,
//         html_output,
//         html_footer
//     );

//     HttpResponse::Ok().content_type("text/html; charset=utf-8").body(html_page)
// }

// #[get("/blog/{file_name}")]
// async fn blog_page(path: web::Path<String>) -> impl Responder {
//     let file_name = path.into_inner();
//     let markdown_file_path = Path::new("markdown_files").join(format!("{}.md", file_name));
//     let markdown_input = match fs::read_to_string(markdown_file_path) {
//         Ok(content) => content,
//         Err(_) => return HttpResponse::NotFound().body("Markdown file not found"),
//     };

//     let mut options = ComrakOptions::default();
//     options.render.unsafe_ = true;

//     let html_output = markdown_to_html(&markdown_input, &options);

//     let html_header = load_template("header.html").await.replace("{title}", "Hoge");
//     let html_footer = load_template("footer.html").await;
//     let html_page = format!(
//         "{}{}{}",
//         html_header,
//         html_output,
//         html_footer
//     );

//     HttpResponse::Ok().content_type("text/html; charset=utf-8").body(html_page)
// }

// #[get("/static/{file_type}/{file_name}")]
// async fn static_files(path: actix_web::web::Path<(String, String)>, req: HttpRequest) -> impl Responder {
//     let (file_type, file_name) = path.into_inner();
//     let file_path = Path::new("static").join(file_type).join(file_name);

//     match NamedFile::open(file_path) {
//         Ok(file) => file.into_response(&req),
//         Err(_) => HttpResponse::NotFound().body("File not found"),
//     }
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .service(root_page)
//             .service(blog_page)
//             .service(static_files)
//     })
//     .bind(("127.0.0.1", 8080))?
//     .run()
//     .await
// }