import React, { useState, useEffect } from 'react';
import { Header } from './components/Header';
import { FileUpload } from './components/FileUpload';
import { ValidationReport } from './components/ValidationReport';
import { ProgressTracker } from './components/ProgressTracker';
import { PdfPreviewer } from './components/PdfPreviewer';
import {
  uploadZip,
  validateDocument,
  triggerRender,
  getJobStatus,
  ValidationReport as ReportType,
} from './api/client';

type Step = 'upload' | 'validated' | 'rendering' | 'completed';

export const App: React.FC = () => {
  const [step, setStep] = useState<Step>('upload');
  const [documentId, setDocumentId] = useState<string | null>(null);
  const [jobId, setJobId] = useState<string | null>(null);
  const [validationReport, setValidationReport] = useState<ReportType | null>(null);

  const [isUploading, setIsUploading] = useState(false);
  const [uploadError, setUploadError] = useState<string | null>(null);

  const [renderProgress, setRenderProgress] = useState(0);
  const [renderStatus, setRenderStatus] = useState<'queued' | 'processing' | 'completed' | 'failed'>('queued');
  const [renderError, setRenderError] = useState<string | null>(null);

  const handleUpload = async (file: File) => {
    setIsUploading(true);
    setUploadError(null);

    try {
      const uploadRes = await uploadZip(file);
      const docId = uploadRes.document_id;
      setDocumentId(docId);

      // Validate extracted contents
      const report = await validateDocument(docId);
      setValidationReport(report);
      setStep('validated');
    } catch (err: any) {
      setUploadError(err.message || 'Failed to upload and validate ZIP archive.');
    } finally {
      setIsUploading(false);
    }
  };

  const handleStartRender = async () => {
    if (!documentId) return;

    setRenderError(null);
    setRenderProgress(10);
    setRenderStatus('queued');
    setStep('rendering');

    try {
      const renderRes = await triggerRender(documentId);
      setJobId(renderRes.job_id);
    } catch (err: any) {
      setRenderError(err.message || 'Failed to trigger PDF rendering process.');
      setRenderStatus('failed');
    }
  };

  // Job status polling effect
  useEffect(() => {
    if (step !== 'rendering' || !jobId) return;

    const interval = setInterval(async () => {
      try {
        const job = await getJobStatus(jobId);
        setRenderProgress(job.progress);
        setRenderStatus(job.status);

        if (job.status === 'completed') {
          clearInterval(interval);
          setStep('completed');
        } else if (job.status === 'failed') {
          clearInterval(interval);
          setRenderError(job.error_message || 'Rendering failed.');
        }
      } catch (err: any) {
        clearInterval(interval);
        setRenderStatus('failed');
        setRenderError(err.message || 'Failed to poll rendering status.');
      }
    }, 1000);

    return () => clearInterval(interval);
  }, [step, jobId]);

  const handleReset = () => {
    setStep('upload');
    setDocumentId(null);
    setJobId(null);
    setValidationReport(null);
    setUploadError(null);
    setRenderError(null);
    setRenderProgress(0);
  };

  return (
    <div className="min-h-screen bg-slate-50 flex flex-col font-sans">
      <Header />

      <main className="flex-1 max-w-6xl w-full mx-auto p-6 md:p-8">
        {step === 'upload' && (
          <div className="mt-8">
            <FileUpload
              onUpload={handleUpload}
              isLoading={isUploading}
              error={uploadError}
            />
          </div>
        )}

        {step === 'validated' && validationReport && (
          <div className="mt-4">
            <ValidationReport
              report={validationReport}
              onGenerate={handleStartRender}
              isRendering={isUploading}
            />
          </div>
        )}

        {step === 'rendering' && (
          <div className="mt-8">
            <ProgressTracker
              progress={renderProgress}
              status={renderStatus}
              error={renderError}
            />
          </div>
        )}

        {step === 'completed' && documentId && (
          <div className="mt-4">
            <PdfPreviewer documentId={documentId} onReset={handleReset} />
          </div>
        )}
      </main>

      <footer className="py-6 text-center text-xs text-slate-400 border-t border-slate-200 bg-white mt-auto">
        Academic Markdown to Printable PDF Renderer &bull; A4 Chromium Engine &bull; Antigravity Built
      </footer>
    </div>
  );
};
