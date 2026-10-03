use crate::archive::extractor::Extractor;
use crate::assets::resolver::AssetResolver;
use crate::config::Config;
use crate::jobs::manager::{JobManager, JobState};
use crate::markdown::parser::MarkdownParser;
use crate::renderer::html::HtmlRenderer;
use crate::renderer::pdf::PdfRenderer;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Serialize;
use std::fs;
use std::sync::Arc;
use tracing::{error, info};

#[derive(Serialize)]
pub struct RenderTriggerResponse {
    pub job_id: String,
    pub document_id: String,
    pub status: String,
    pub message: String,
}

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub job_manager: JobManager,
}

pub async fn render_handler(
    State(state): State<AppState>,
    Path(doc_id): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let extract_dir = state.config.extracted_dir.join(&doc_id);
    if !extract_dir.exists() {
        return Err((
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": format!("Document workspace not found for ID: {}", doc_id) })),
        ));
    }

    let job = state.job_manager.create_job(&doc_id);
    let job_id = job.id.clone();
    let job_id_response = job_id.clone();
    let job_manager = state.job_manager.clone();
    let config = state.config.clone();
    let doc_id_clone = doc_id.clone();

    tokio::spawn(async move {
        job_manager.update_progress(&job_id, 10, JobState::Processing);
        info!("Starting PDF rendering job {} for document {}", job_id, doc_id_clone);

        let zip_file_path = config.uploads_dir.join(format!("{}.zip", doc_id_clone));

        // 1. Inspect extraction & primary markdown
        let extraction = match Extractor::extract_zip(
            &zip_file_path,
            &extract_dir,
            config.max_extracted_size,
            config.max_file_count,
        ) {
            Ok(res) => res,
            Err(e) => {
                job_manager.fail_job(&job_id, &format!("Failed to locate Markdown: {}", e));
                return;
            }
        };

        let primary_md_path = match extraction.primary_markdown {
            Some(p) => p,
            None => {
                job_manager.fail_job(&job_id, "No Markdown file found in upload archive");
                return;
            }
        };

        job_manager.update_progress(&job_id, 30, JobState::Processing);

        // 2. Parse Markdown
        let full_md_path = extract_dir.join(&primary_md_path);
        let md_content = match fs::read_to_string(&full_md_path) {
            Ok(c) => c,
            Err(e) => {
                job_manager.fail_job(&job_id, &format!("Failed to read Markdown content: {}", e));
                return;
            }
        };

        let mut doc = MarkdownParser::parse(&md_content);

        // 3. Resolve & embed assets
        job_manager.update_progress(&job_id, 50, JobState::Processing);
        AssetResolver::resolve_and_embed(&mut doc, &extract_dir, &primary_md_path);

        // 4. Generate HTML
        job_manager.update_progress(&job_id, 70, JobState::Processing);
        let html_string = HtmlRenderer::render_document(&doc);

        let html_preview_path = config.previews_dir.join(format!("{}.html", doc_id_clone));
        if let Err(e) = fs::write(&html_preview_path, &html_string) {
            job_manager.fail_job(&job_id, &format!("Failed to save HTML preview file: {}", e));
            return;
        }

        // 5. Render PDF using Chromium Playwright
        job_manager.update_progress(&job_id, 85, JobState::Processing);
        let pdf_output_path = config.generated_dir.join(format!("{}.pdf", doc_id_clone));

        match PdfRenderer::render_html_to_pdf(&html_preview_path, &pdf_output_path).await {
            Ok(_) => {
                let pdf_path_str = pdf_output_path.to_string_lossy().to_string();
                job_manager.complete_job(&job_id, &pdf_path_str);
                info!("Completed rendering job {} -> {}", job_id, pdf_path_str);
            }
            Err(err_msg) => {
                error!("Render job {} failed: {}", job_id, err_msg);
                job_manager.fail_job(&job_id, &err_msg);
            }
        }
    });

    Ok(Json(RenderTriggerResponse {
        job_id: job_id_response,
        document_id: doc_id,
        status: "queued".to_string(),
        message: "PDF rendering process queued successfully".to_string(),
    }))
}
