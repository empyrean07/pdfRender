import React, { useState, useRef } from 'react';
import { UploadCloud, FileArchive, AlertCircle, Loader2 } from 'lucide-react';

interface FileUploadProps {
  onUpload: (file: File) => void;
  isLoading: boolean;
  error?: string | null;
}

export const FileUpload: React.FC<FileUploadProps> = ({ onUpload, isLoading, error }) => {
  const [isDragOver, setIsDragOver] = useState(false);
  const fileInputRef = useRef<HTMLInputElement>(null);

  const handleDragOver = (e: React.DragEvent) => {
    e.preventDefault();
    setIsDragOver(true);
  };

  const handleDragLeave = (e: React.DragEvent) => {
    e.preventDefault();
    setIsDragOver(false);
  };

  const handleDrop = (e: React.DragEvent) => {
    e.preventDefault();
    setIsDragOver(false);
    if (e.dataTransfer.files && e.dataTransfer.files.length > 0) {
      const file = e.dataTransfer.files[0];
      if (file.name.endsWith('.zip')) {
        onUpload(file);
      } else {
        alert('Please select a valid ZIP archive (.zip)');
      }
    }
  };

  const handleFileSelect = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (e.target.files && e.target.files.length > 0) {
      const file = e.target.files[0];
      onUpload(file);
    }
  };

  return (
    <div className="bg-white rounded-xl shadow-sm border border-slate-200 p-8 max-w-2xl mx-auto">
      <div className="text-center mb-6">
        <h2 className="text-2xl font-bold text-slate-800">Upload Document Archive</h2>
        <p className="text-slate-500 text-sm mt-1">
          Upload a ZIP package containing your Markdown document (<code className="bg-slate-100 px-1.5 py-0.5 rounded text-blue-600 font-mono">.md</code>) and referenced images or diagrams.
        </p>
      </div>

      <div
        onDragOver={handleDragOver}
        onDragLeave={handleDragLeave}
        onDrop={handleDrop}
        onClick={() => fileInputRef.current?.click()}
        className={`border-2 border-dashed rounded-xl p-10 text-center cursor-pointer transition-all ${
          isDragOver
            ? 'border-blue-500 bg-blue-50/50 scale-[1.01]'
            : 'border-slate-300 hover:border-slate-400 bg-slate-50/50'
        }`}
      >
        <input
          type="file"
          ref={fileInputRef}
          onChange={handleFileSelect}
          accept=".zip"
          className="hidden"
        />

        {isLoading ? (
          <div className="flex flex-col items-center justify-center py-4">
            <Loader2 className="w-12 h-12 text-blue-600 animate-spin mb-3" />
            <p className="text-slate-700 font-medium">Extracting and validating ZIP archive...</p>
            <p className="text-xs text-slate-400 mt-1">Checking path safety & image references</p>
          </div>
        ) : (
          <div className="flex flex-col items-center justify-center py-4">
            <div className="w-16 h-16 bg-blue-100 rounded-full flex items-center justify-center text-blue-600 mb-4">
              <UploadCloud className="w-8 h-8" />
            </div>
            <p className="text-slate-700 font-semibold text-base mb-1">
              Drag & drop your document ZIP file here
            </p>
            <p className="text-slate-400 text-xs mb-4">
              Supports .zip containing Markdown files, PNG, JPG, WEBP, SVG
            </p>
            <button
              type="button"
              className="inline-flex items-center space-x-2 bg-blue-600 hover:bg-blue-700 text-white font-medium px-4 py-2 rounded-lg text-sm transition-colors shadow-sm"
            >
              <FileArchive className="w-4 h-4" />
              <span>Browse ZIP File</span>
            </button>
          </div>
        )}
      </div>

      {error && (
        <div className="mt-4 p-4 bg-red-50 border border-red-200 rounded-lg flex items-start space-x-3 text-red-700 text-sm">
          <AlertCircle className="w-5 h-5 text-red-500 flex-shrink-0 mt-0.5" />
          <div>
            <p className="font-semibold">Upload Error</p>
            <p className="mt-0.5 text-xs text-red-600">{error}</p>
          </div>
        </div>
      )}
    </div>
  );
};
