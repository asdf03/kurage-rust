use std::{
  fs::{self, File},
  path::Path,
  io::{BufRead, BufReader}
};

use axum::{
    response::{
        Html,
        IntoResponse
    },
    extract::Path as extractPath
};

use comrak::{markdown_to_html, ComrakOptions};

async fn load_template(template_name: &str) -> String {
    let template_path = Path::new("static").join("html").join(template_name);
    fs::read_to_string(template_path).expect(&format!("Failed to read {}", template_name))
}

pub async fn blog_page(extractPath(file_name): extractPath<String>) -> impl IntoResponse {
    let markdown_file_path = Path::new("markdown_files").join(format!("{}.md", file_name));
    let markdown_input = fs::read_to_string(markdown_file_path).expect("Failed to read markdown file");
    
    let mut options = ComrakOptions::default();
    options.render.unsafe_ = true;

    let html_output = markdown_to_html(&markdown_input, &options);
    let html_header = load_template("blog-header.html").await.replace("{title}", "Hoge");
    let html_footer = load_template("footer.html").await;

    let html_page = format!("{}{}{}", html_header, html_output, html_footer);

    Html(html_page)
}


