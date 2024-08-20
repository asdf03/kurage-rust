mod domain;
mod rest;
mod infrastructure;
mod interface;
mod usecase;

// use crate::infrastructure::db::create_db_pool;
use crate::rest::auth::auth_register;

use crate::rest::blog::create_router;


use axum::{
    extract::Path as AxumPath, response::Html, routing::{get, post}, Extension, Router
};
use comrak::{markdown_to_html, ComrakOptions};
use std::{fs, path::Path, sync::{Arc, Mutex}};


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

async fn blog_page(
        AxumPath((file_name,)): AxumPath<(String,)>
    ) -> Html<String> {
    let markdown_file_path = Path::new("markdown_files").join(format!("{}.md", file_name));
    let markdown_input = fs::read_to_string(markdown_file_path).expect("Failed to read markdown file");
    
    let mut options = ComrakOptions::default();
    options.render.unsafe_ = true;

    let html_output = markdown_to_html(&markdown_input, &options);

    let html_header = load_template("header.html").await.replace("{title}", "Hoge");
    let html_footer = load_template("footer.html").await;
    let html_page = format!("{}{}{}", html_header, html_output, html_footer);

    Html(html_page)

}

async fn static_files(
    AxumPath((file_type, file_name)): AxumPath<(String, String)>
) -> axum::http::Response<axum::body::Body> {
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
    // let db_pool = create_db_pool().await;
    // let db_pool = Arc::new(Mutex::new(db_pool));

    let app = create_router();
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .unwrap();

    axum::serve(listener, app)
        .await
        .unwrap();
}
