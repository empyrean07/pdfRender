use crate::api::render::AppState;
use crate::archive::extractor::Extractor;
use axum::{
    extract::{Multipart, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Serialize;
use std::fs::File;
use std::io::Write;

#[derive(Serialize)]
pub struct UploadResponse {
    pub document_id: String,
    pub status: String,
    pub primary_markdown: Option<String>,
    pub markdown_files_count: usize,
    pub message: String,
}

pub async fn upload_handler(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let config = &state.config;
    let mut zip_bytes = Vec::new();

    while let Ok(Some(field)) = multipart.next_field().await {
        if field.name() == Some("file") {
            let data = field.bytes().await.map_err(|e| {
                (
                    StatusCode::BAD_REQUEST,
                    Json(serde_json::json!({ "error": format!("Failed to read file payload: {}", e) })),
                )
            })?;
            zip_bytes = data.to_vec();
            break;
        }
    }

    if zip_bytes.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": "No ZIP file uploaded in request field 'file'" })),
        ));
    }

    if zip_bytes.len() > config.max_upload_size {
        return Err((
            StatusCode::PAYLOAD_TOO_LARGE,
            Json(serde_json::json!({ "error": format!("Uploaded ZIP file exceeds maximum size of {} MB", config.max_upload_size / (1024 * 1024)) })),
        ));
    }

    let doc_id = uuid::Uuid::new_v4().to_string();
    let zip_save_path = config.uploads_dir.join(format!("{}.zip", doc_id));
    let extract_save_dir = config.extracted_dir.join(&doc_id);

    // Save uploaded ZIP
    let mut file = File::create(&zip_save_path).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": format!("Failed to save ZIP to upload storage: {}", e) })),
        )
    })?;

    file.write_all(&zip_bytes).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": format!("Failed to write ZIP file: {}", e) })),
        )
    })?;

    // Extract ZIP archive
    let extraction = Extractor::extract_zip(
        &zip_save_path,
        &extract_save_dir,
        config.max_extracted_size,
        config.max_file_count,
    )
    .map_err(|err_msg| {
        (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": format!("ZIP extraction failed: {}", err_msg) })),
        )
    })?;

    let primary_md_str = extraction
        .primary_markdown
        .as_ref()
        .map(|p| p.to_string_lossy().to_string());

    Ok(Json(UploadResponse {
        document_id: doc_id,
        status: "uploaded".to_string(),
        primary_markdown: primary_md_str,
        markdown_files_count: extraction.markdown_files.len(),
        message: "ZIP successfully extracted and validated".to_string(),
    }))
}
