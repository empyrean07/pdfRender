export interface UploadResponse {
  document_id: string;
  status: string;
  primary_markdown?: string;
  markdown_files_count: number;
  message: string;
}

export interface BrokenAsset {
  markdown_file: string;
  reference: string;
  status: string;
}

export interface ValidationReport {
  document_id: string;
  status: string;
  markdown_files_count: number;
  primary_markdown: string;
  total_images: number;
  total_tables: number;
  total_questions: number;
  broken_references: BrokenAsset[];
  ready_for_rendering: boolean;
}

export interface RenderTriggerResponse {
  job_id: string;
  document_id: string;
  status: string;
  message: string;
}

export interface JobStatusResponse {
  id: string;
  document_id: string;
  status: 'queued' | 'processing' | 'completed' | 'failed';
  progress: number;
  error_message?: string;
  pdf_path?: string;
}

const API_BASE = '/api';

export async function uploadZip(file: File): Promise<UploadResponse> {
  const formData = new FormData();
  formData.append('file', file);

  const res = await fetch(`${API_BASE}/documents/upload`, {
    method: 'POST',
    body: formData,
  });

  if (!res.ok) {
    const errData = await res.json().catch(() => ({ error: 'Upload failed' }));
    throw new Error(errData.error || `Upload failed with status ${res.status}`);
  }

  return res.json();
}

export async function validateDocument(docId: string): Promise<ValidationReport> {
  const res = await fetch(`${API_BASE}/documents/${docId}/validate`, {
    method: 'POST',
  });

  if (!res.ok) {
    const errData = await res.json().catch(() => ({ error: 'Validation failed' }));
    throw new Error(errData.error || `Validation failed with status ${res.status}`);
  }

  return res.json();
}

export async function triggerRender(docId: string): Promise<RenderTriggerResponse> {
  const res = await fetch(`${API_BASE}/documents/${docId}/render`, {
    method: 'POST',
  });

  if (!res.ok) {
    const errData = await res.json().catch(() => ({ error: 'Render trigger failed' }));
    throw new Error(errData.error || `Render trigger failed with status ${res.status}`);
  }

  return res.json();
}

export async function getJobStatus(jobId: string): Promise<JobStatusResponse> {
  const res = await fetch(`${API_BASE}/jobs/${jobId}`);

  if (!res.ok) {
    const errData = await res.json().catch(() => ({ error: 'Status check failed' }));
    throw new Error(errData.error || `Job status check failed with status ${res.status}`);
  }

  return res.json();
}

export function getPdfUrl(docId: string): string {
  return `${API_BASE}/documents/${docId}/pdf`;
}

export function getPreviewHtmlUrl(docId: string): string {
  return `${API_BASE}/documents/${docId}/preview-html`;
}
