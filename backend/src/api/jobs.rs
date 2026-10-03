use crate::api::render::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

pub async fn get_job_handler(
    State(state): State<AppState>,
    Path(job_id): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    match state.job_manager.get_job(&job_id) {
        Some(job) => Ok(Json(serde_json::to_value(job).unwrap())),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": format!("Job ID not found: {}", job_id) })),
        )),
    }
}
