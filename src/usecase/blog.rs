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

use tera::{Tera, Context};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct BlogPost {
  url_path: String,
  title: String,
}


async fn load_template(template_name: &str) -> String {
  let template_path = Path::new("static").join("html").join(template_name);
  fs::read_to_string(template_path).expect(&format!("Failed to read {}", template_name))
}


fn process_markdown_file(path: &Path, blog_posts: &mut Vec<BlogPost>) -> std::io::Result<()> {
  let file_stem = path.file_stem().ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "Invalid file name"))?;
let url_path = format!("/blog/{}", file_stem.to_string_lossy()); 
  let file = File::open(path)?;
  let reader = BufReader::new(file);
  
  for line in reader.lines() {
    let line = line?;
    if line.starts_with("# ") {
      let title = line.trim_start_matches("## ").to_string();
      blog_posts.push(BlogPost { url_path, title });
      break;
    }
  }
  
  Ok(())
}

pub async fn root_page() -> impl IntoResponse {
  let current_dir = std::env::current_dir().expect("Failed to get current_dir path");
  let folder_path = current_dir.join("markdown_files");
  let mut blog_posts = Vec::new();
  
  match fs::read_dir(folder_path) {
    Ok(entries) => {
      for entry in entries {
        match entry {
          Ok(entry) => {
            let path = entry.path();
            if path.is_file() {
              if let Err(e) = process_markdown_file(&path, &mut blog_posts) {
                println!("Error processing file {:?}: {}", path, e);
              }
            }
          }
          Err(e) => println!("Error reading entry: {}", e),
        }
      }
    }
    Err(e) => println!("Error reading directory: {}", e),
  }

  println!("{:#?}", blog_posts);

  let mut tera = Tera::default();
  tera.add_raw_template("root", &load_template("root.html").await).unwrap();

  let mut context = Context::new();
  context.insert("blog_posts", &blog_posts);

  let html_header = load_template("header.html").await;
  let html_root = tera.render("root", &context).unwrap();
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
	let html_header = load_template("blog-header.html").await.replace("{title}", "Hoge");
	let html_footer = load_template("footer.html").await;

	let html_page = format!("{}{}{}", html_header, html_output, html_footer);

	Html(html_page)
}


