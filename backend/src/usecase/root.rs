use axum::{ response::{ Html, IntoResponse } };
use tera::{ Tera, Context };
use serde::Serialize;
use std::{ path::Path, fs::{ self, File }, io::{ BufRead, BufReader }, env::current_dir };

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
    let file_stem = path
        .file_stem()
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "Invalid file name"))?;
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

pub async fn root_page() -> Result<impl IntoResponse, ()> {
    let current_dir = current_dir().expect("Failed to get current_dir path");
    let folder_path = current_dir.join("markdown_files");
    let mut blog_posts = Vec::new();

    if let Err(e) = fs::read_dir(&folder_path) {
        println!("Error reading directory: {}", e);
        return Err(());
    }

    for entry in fs::read_dir(folder_path).unwrap() {
        let entry = match entry {
            Ok(entry) => entry,
            Err(e) => {
                println!("Error reading entry: {}", e);
                continue;
            }
        };

        let path = entry.path();
        if path.is_file() {
            if let Err(e) = process_markdown_file(&path, &mut blog_posts) {
                println!("Error processing file {:?}: {}", path, e);
            }
        }
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

    Ok(Html(html_page))
}
