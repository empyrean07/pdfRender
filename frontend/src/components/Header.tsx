import React from 'react';
import { FileText, Printer, ShieldCheck } from 'lucide-react';

export const Header: React.FC = () => {
  return (
    <header className="bg-slate-900 border-b border-slate-800 text-white py-4 px-6 shadow-md">
      <div className="max-w-6xl mx-auto flex items-center justify-between">
        <div className="flex items-center space-x-3">
          <div className="bg-blue-600 p-2 rounded-lg text-white">
            <FileText className="w-6 h-6" />
          </div>
          <div>
            <h1 className="text-xl font-bold tracking-tight">Academic Markdown Renderer</h1>
            <p className="text-xs text-slate-400">Print-Ready A4 Document Pipeline</p>
          </div>
        </div>

        <div className="flex items-center space-x-4 text-xs font-medium text-slate-300">
          <div className="flex items-center space-x-1.5 bg-slate-800 px-3 py-1.5 rounded-md border border-slate-700">
            <ShieldCheck className="w-4 h-4 text-emerald-400" />
            <span>Path-Safe ZIP Ingestion</span>
          </div>
          <div className="flex items-center space-x-1.5 bg-slate-800 px-3 py-1.5 rounded-md border border-slate-700">
            <Printer className="w-4 h-4 text-blue-400" />
            <span>Chromium A4 Engine</span>
          </div>
        </div>
      </div>
    </header>
  );
};
