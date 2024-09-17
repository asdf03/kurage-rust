use std::{
  fs,
  path::Path
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

pub async fn root_page() -> impl IntoResponse {
  let current_dir = std::env::current_dir().expect("Failed to get current_dir path");
  let folder_path = current_dir.join("markdown_files");
  
  match fs::read_dir(folder_path) {
    Ok(entries) => {
      for entry in entries {
        match entry {
          Ok(entry) => {
            let path = entry.path();
            if path.is_file() {
              println!("OK");
            }
          }
          Err(e) => println!("Error reading entry: {}", e),
        }
      }
    }
    Err(e) => println!("Error reading directory: {}", e),

    // let blog_path = blog_page.path();
    // if blog_path.is_file() {
    //   println!("Processing file: {:?}", blog_path);
    //   // ここでファイルに対する処理を実行
    // }

  }

  let html_header = load_template("header.html").await;
  let html_root = load_template("root.html").await;
  let html_footer = load_template("footer.html").await;

  let html_page = format!("{}{}{}", html_header, html_root, html_footer);

  Html(html_page)
}

pub async fn blog_page(extractPath(file_name): extractPath<String>) -> impl IntoResponse {
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


