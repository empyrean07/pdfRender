use crate::api::render::AppState;
use axum::{
    body::Body,
    extract::{Path, State},
    http::{header, HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use tokio_util::io::ReaderStream;

pub async fn preview_html_handler(
    State(state): State<AppState>,
    Path(doc_id): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let preview_path = state.config.previews_dir.join(format!("{}.html", doc_id));

    if !preview_path.exists() {
        return Err((
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": format!("HTML preview file not found for ID: {}. Please render document first.", doc_id) })),
        ));
    }

    let file = tokio::fs::File::open(&preview_path).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": format!("Failed to open HTML preview file: {}", e) })),
        )
    })?;

    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);

    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "text/html; charset=utf-8".parse().unwrap());

    Ok((headers, body))
}
