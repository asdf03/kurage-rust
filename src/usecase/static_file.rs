use axum::{
  extract::Path as AxumPath,
  http::{
    Response,
    StatusCode,
    header
  },
  body::Body
};

use std::{
  path::Path as StdPath
};

pub async fn static_file(
  AxumPath((file_type, file_name)): AxumPath<(String, String)>
) -> Response<Body> {
  let file_path = StdPath::new("static").join(file_type).join(file_name);
  if let Ok(file_content) = tokio::fs::read(&file_path).await {
    Response::builder()
      .status(StatusCode::OK)
      .header(header::CONTENT_TYPE, mime_guess::from_path(&file_path).first_or_octet_stream().as_ref())
      .body(Body::from(file_content))
      .expect("Failed to construct response")
  } else {
    Response::builder()
      .status(StatusCode::NOT_FOUND)
      .body(Body::empty())
      .expect("Failed to construct response")
  }
}