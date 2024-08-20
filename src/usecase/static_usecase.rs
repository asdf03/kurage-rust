use axum::{
  extract::Path as AxumPath
};

use std::{
  path::Path as StdPath
};

pub async fn static_files(
  AxumPath((file_type, file_name)): AxumPath<(String, String)>
) -> axum::http::Response<axum::body::Body> {
  let file_path = StdPath::new("static").join(file_type).join(file_name);
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