use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum JobState {
    Queued,
    Processing,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderJob {
    pub id: String,
    pub document_id: String,
    pub status: JobState,
    pub progress: u8,
    pub error_message: Option<String>,
    pub pdf_path: Option<String>,
}

#[derive(Clone, Default)]
pub struct JobManager {
    jobs: Arc<Mutex<HashMap<String, RenderJob>>>,
}

impl JobManager {
    pub fn new() -> Self {
        Self {
            jobs: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn create_job(&self, document_id: &str) -> RenderJob {
        let job_id = Uuid::new_v4().to_string();
        let job = RenderJob {
            id: job_id.clone(),
            document_id: document_id.to_string(),
            status: JobState::Queued,
            progress: 0,
            error_message: None,
            pdf_path: None,
        };
        let mut map = self.jobs.lock().unwrap();
        map.insert(job_id, job.clone());
        job
    }

    pub fn update_progress(&self, job_id: &str, progress: u8, status: JobState) {
        let mut map = self.jobs.lock().unwrap();
        if let Some(job) = map.get_mut(job_id) {
            job.progress = progress;
            job.status = status;
        }
    }

    pub fn complete_job(&self, job_id: &str, pdf_path: &str) {
        let mut map = self.jobs.lock().unwrap();
        if let Some(job) = map.get_mut(job_id) {
            job.progress = 100;
            job.status = JobState::Completed;
            job.pdf_path = Some(pdf_path.to_string());
        }
    }

    pub fn fail_job(&self, job_id: &str, error: &str) {
        let mut map = self.jobs.lock().unwrap();
        if let Some(job) = map.get_mut(job_id) {
            job.status = JobState::Failed;
            job.error_message = Some(error.to_string());
        }
    }

    pub fn get_job(&self, job_id: &str) -> Option<RenderJob> {
        let map = self.jobs.lock().unwrap();
        map.get(job_id).cloned()
    }
}
