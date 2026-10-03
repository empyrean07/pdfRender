use crate::api::render::AppState;
use axum::{
    body::Body,
    extract::{Path, State},
    http::{header, HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use tokio_util::io::ReaderStream;

pub async fn download_pdf_handler(
    State(state): State<AppState>,
    Path(doc_id): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let pdf_path = state.config.generated_dir.join(format!("{}.pdf", doc_id));

    if !pdf_path.exists() {
        return Err((
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": format!("PDF document not found for ID: {}. Please generate the PDF first.", doc_id) })),
        ));
    }

    let file = tokio::fs::File::open(&pdf_path).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": format!("Failed to open generated PDF file: {}", e) })),
        )
    })?;

    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);

    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "application/pdf".parse().unwrap());
    headers.insert(
        header::CONTENT_DISPOSITION,
        format!("inline; filename=\"document-{}.pdf\"", doc_id)
            .parse()
            .unwrap(),
    );

    Ok((headers, body))
}
