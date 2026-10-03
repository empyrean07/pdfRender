mod api;
mod archive;
mod assets;
mod config;
mod document;
mod jobs;
mod markdown;
mod renderer;

use api::download::download_pdf_handler;
use api::jobs::get_job_handler;
use api::preview::preview_html_handler;
use api::render::{render_handler, AppState};
use api::upload::upload_handler;
use api::validate::validate_handler;
use config::Config;
use jobs::manager::JobManager;

use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "backend=info,tower_http=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::default();

    // Ensure storage subdirectories exist
    let _ = std::fs::create_dir_all(&config.uploads_dir);
    let _ = std::fs::create_dir_all(&config.extracted_dir);
    let _ = std::fs::create_dir_all(&config.generated_dir);
    let _ = std::fs::create_dir_all(&config.previews_dir);

    let state = AppState {
        config: Arc::new(config.clone()),
        job_manager: JobManager::new(),
    };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/documents/upload", post(upload_handler))
        .route("/api/documents/:id/validate", post(validate_handler))
        .route("/api/documents/:id/render", post(render_handler))
        .route("/api/jobs/:id", get(get_job_handler))
        .route("/api/documents/:id/pdf", get(download_pdf_handler))
        .route("/api/documents/:id/preview-html", get(preview_html_handler))
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .with_state(state);

    // Bind to 0.0.0.0 to accept connections across Docker container network
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    info!("Starting Academic Markdown PDF Renderer backend on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind TCP listener port");
    
    axum::serve(listener, app)
        .await
        .expect("Backend server crashed");
}
