import React from 'react';
import { Loader2, Printer, CheckCircle } from 'lucide-react';

interface ProgressTrackerProps {
  progress: number;
  status: 'queued' | 'processing' | 'completed' | 'failed';
  error?: string | null;
}

export const ProgressTracker: React.FC<ProgressTrackerProps> = ({ progress, status, error }) => {
  return (
    <div className="bg-white rounded-xl shadow-sm border border-slate-200 p-8 max-w-xl mx-auto text-center mb-8">
      {status === 'failed' ? (
        <div className="text-red-600">
          <div className="w-12 h-12 bg-red-100 rounded-full flex items-center justify-center mx-auto mb-3">
            <span className="text-xl">⚠️</span>
          </div>
          <h3 className="text-lg font-bold text-slate-800">Rendering Failed</h3>
          <p className="text-sm text-red-600 mt-2 bg-red-50 p-3 rounded-lg border border-red-200 font-mono text-left">
            {error || 'An unexpected rendering error occurred.'}
          </p>
        </div>
      ) : status === 'completed' ? (
        <div className="text-emerald-600">
          <CheckCircle className="w-12 h-12 mx-auto mb-3 animate-bounce" />
          <h3 className="text-lg font-bold text-slate-800">Rendering Complete!</h3>
          <p className="text-xs text-slate-500 mt-1">PDF ready for preview and download below.</p>
        </div>
      ) : (
        <div>
          <div className="w-12 h-12 bg-blue-100 text-blue-600 rounded-full flex items-center justify-center mx-auto mb-4">
            <Printer className="w-6 h-6 animate-pulse" />
          </div>
          <h3 className="text-lg font-bold text-slate-800 mb-1">Typesetting & Rendering PDF...</h3>
          <p className="text-xs text-slate-500 mb-6">
            Loading HTML layout into Chromium engine, resolving print CSS break rules, and rasterizing A4 pages.
          </p>

          {/* Progress Bar */}
          <div className="w-full bg-slate-100 rounded-full h-3 mb-2 overflow-hidden border border-slate-200">
            <div
              className="bg-blue-600 h-3 rounded-full transition-all duration-300"
              style={{ width: `${Math.max(5, progress)}%` }}
            />
          </div>

          <div className="flex justify-between items-center text-xs text-slate-500 font-medium px-1">
            <span className="capitalize">Status: {status}</span>
            <span>{progress}%</span>
          </div>
        </div>
      )}
    </div>
  );
};
