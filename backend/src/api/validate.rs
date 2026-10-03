use crate::api::render::AppState;
use crate::archive::extractor::Extractor;
use crate::assets::resolver::{AssetResolver, BrokenAsset};
use crate::markdown::parser::MarkdownParser;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Serialize;
use std::fs;

#[derive(Serialize)]
pub struct ValidateResponse {
    pub document_id: String,
    pub status: String,
    pub markdown_files_count: usize,
    pub primary_markdown: String,
    pub total_images: usize,
    pub total_tables: usize,
    pub total_questions: usize,
    pub broken_references: Vec<BrokenAsset>,
    pub ready_for_rendering: bool,
}

pub async fn validate_handler(
    State(state): State<AppState>,
    Path(doc_id): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let config = &state.config;
    let extract_dir = config.extracted_dir.join(&doc_id);
    if !extract_dir.exists() {
        return Err((
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": format!("Document workspace not found for ID: {}", doc_id) })),
        ));
    }

    let zip_file_path = config.uploads_dir.join(format!("{}.zip", doc_id));

    // Discover Markdown files
    let extraction = Extractor::extract_zip(
        &zip_file_path,
        &extract_dir,
        config.max_extracted_size,
        config.max_file_count,
    )
    .map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": format!("Failed to inspect document workspace: {}", e) })),
        )
    })?;

    let primary_md_path = match extraction.primary_markdown {
        Some(p) => p,
        None => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({ "error": "No Markdown (.md) file found inside the uploaded ZIP" })),
            ));
        }
    };

    let full_md_path = extract_dir.join(&primary_md_path);
    let md_content = fs::read_to_string(&full_md_path).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": format!("Failed to read primary Markdown file: {}", e) })),
        )
    })?;

    let mut doc = MarkdownParser::parse(&md_content);

    // Resolve asset references
    let asset_report = AssetResolver::resolve_and_embed(&mut doc, &extract_dir, &primary_md_path);

    Ok(Json(ValidateResponse {
        document_id: doc_id,
        status: asset_report.status.clone(),
        markdown_files_count: extraction.markdown_files.len(),
        primary_markdown: primary_md_path.to_string_lossy().to_string(),
        total_images: doc.metadata.total_images,
        total_tables: doc.metadata.total_tables,
        total_questions: doc.metadata.total_questions,
        broken_references: asset_report.broken_assets,
        ready_for_rendering: true,
    }))
}
