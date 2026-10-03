import React, { useState } from 'react';
import { Download, Eye, FileCode, RefreshCw } from 'lucide-react';
import { getPdfUrl, getPreviewHtmlUrl } from '../api/client';

interface PdfPreviewerProps {
  documentId: string;
  onReset: () => void;
}

export const PdfPreviewer: React.FC<PdfPreviewerProps> = ({ documentId, onReset }) => {
  const [activeTab, setActiveTab] = useState<'pdf' | 'html'>('pdf');
  const pdfUrl = getPdfUrl(documentId);
  const htmlUrl = getPreviewHtmlUrl(documentId);

  return (
    <div className="bg-white rounded-xl shadow-lg border border-slate-200 overflow-hidden max-w-5xl mx-auto">
      {/* Action Header Toolbar */}
      <div className="bg-slate-900 text-white px-6 py-4 flex flex-wrap items-center justify-between gap-4 border-b border-slate-800">
        <div className="flex items-center space-x-3">
          <button
            onClick={() => setActiveTab('pdf')}
            className={`flex items-center space-x-1.5 px-3 py-1.5 rounded-lg text-xs font-semibold transition-colors ${
              activeTab === 'pdf'
                ? 'bg-blue-600 text-white shadow'
                : 'bg-slate-800 text-slate-300 hover:bg-slate-700'
            }`}
          >
            <Eye className="w-4 h-4" />
            <span>PDF View</span>
          </button>

          <button
            onClick={() => setActiveTab('html')}
            className={`flex items-center space-x-1.5 px-3 py-1.5 rounded-lg text-xs font-semibold transition-colors ${
              activeTab === 'html'
                ? 'bg-blue-600 text-white shadow'
                : 'bg-slate-800 text-slate-300 hover:bg-slate-700'
            }`}
          >
            <FileCode className="w-4 h-4" />
            <span>HTML Layout View</span>
          </button>
        </div>

        <div className="flex items-center space-x-3">
          <button
            onClick={onReset}
            className="flex items-center space-x-1.5 px-3 py-1.5 bg-slate-800 hover:bg-slate-700 text-slate-300 rounded-lg text-xs font-medium transition-colors"
          >
            <RefreshCw className="w-3.5 h-3.5" />
            <span>Upload New Document</span>
          </button>

          <a
            href={pdfUrl}
            download={`academic-document-${documentId}.pdf`}
            target="_blank"
            rel="noopener noreferrer"
            className="flex items-center space-x-2 bg-emerald-600 hover:bg-emerald-700 text-white px-4 py-2 rounded-lg text-xs font-bold transition-all shadow hover:shadow-lg"
          >
            <Download className="w-4 h-4" />
            <span>Download PDF</span>
          </a>
        </div>
      </div>

      {/* Frame Container */}
      <div className="w-full h-[750px] bg-slate-100">
        {activeTab === 'pdf' ? (
          <iframe
            src={pdfUrl}
            title="Generated PDF Preview"
            className="w-full h-full border-none shadow-inner"
          />
        ) : (
          <iframe
            src={htmlUrl}
            title="HTML Print Layout Preview"
            className="w-full h-full border-none bg-white p-4 shadow-inner"
          />
        )}
      </div>
    </div>
  );
};
