import React from 'react';
import { ValidationReport as ReportType } from '../api/client';
import { CheckCircle2, AlertTriangle, FileText, Image, Table, HelpCircle, ArrowRight } from 'lucide-react';

interface ValidationReportProps {
  report: ReportType;
  onGenerate: () => void;
  isRendering: boolean;
}

export const ValidationReport: React.FC<ValidationReportProps> = ({ report, onGenerate, isRendering }) => {
  const isReady = report.status === 'READY';

  return (
    <div className="bg-white rounded-xl shadow-sm border border-slate-200 p-6 max-w-3xl mx-auto mb-8">
      <div className="flex items-center justify-between border-b border-slate-100 pb-4 mb-6">
        <div>
          <h2 className="text-xl font-bold text-slate-800">Document Validation Report</h2>
          <p className="text-xs text-slate-500 font-mono mt-0.5">
            Primary Document: <span className="text-blue-600 font-semibold">{report.primary_markdown}</span>
          </p>
        </div>

        <div className={`flex items-center space-x-1.5 px-3 py-1 rounded-full text-xs font-semibold ${
          isReady ? 'bg-emerald-100 text-emerald-800' : 'bg-amber-100 text-amber-800'
        }`}>
          {isReady ? (
            <>
              <CheckCircle2 className="w-4 h-4 text-emerald-600" />
              <span>Status: READY</span>
            </>
          ) : (
            <>
              <AlertTriangle className="w-4 h-4 text-amber-600" />
              <span>Status: BROKEN REFERENCES</span>
            </>
          )}
        </div>
      </div>

      {/* Metrics Grid */}
      <div className="grid grid-cols-4 gap-4 mb-6">
        <div className="bg-slate-50 p-4 rounded-lg border border-slate-100 text-center">
          <FileText className="w-5 h-5 text-blue-600 mx-auto mb-1.5" />
          <div className="text-2xl font-bold text-slate-800">{report.markdown_files_count}</div>
          <div className="text-xs text-slate-500 font-medium">Markdown File</div>
        </div>

        <div className="bg-slate-50 p-4 rounded-lg border border-slate-100 text-center">
          <Image className="w-5 h-5 text-emerald-600 mx-auto mb-1.5" />
          <div className="text-2xl font-bold text-slate-800">{report.total_images}</div>
          <div className="text-xs text-slate-500 font-medium">Image / Diagram</div>
        </div>

        <div className="bg-slate-50 p-4 rounded-lg border border-slate-100 text-center">
          <Table className="w-5 h-5 text-purple-600 mx-auto mb-1.5" />
          <div className="text-2xl font-bold text-slate-800">{report.total_tables}</div>
          <div className="text-xs text-slate-500 font-medium">Tables</div>
        </div>

        <div className="bg-slate-50 p-4 rounded-lg border border-slate-100 text-center">
          <HelpCircle className="w-5 h-5 text-amber-600 mx-auto mb-1.5" />
          <div className="text-2xl font-bold text-slate-800">{report.total_questions}</div>
          <div className="text-xs text-slate-500 font-medium">Questions</div>
        </div>
      </div>

      {/* Broken References Warnings */}
      {report.broken_references.length > 0 && (
        <div className="mb-6 bg-amber-50 border border-amber-200 rounded-lg p-4">
          <div className="flex items-center space-x-2 text-amber-800 font-semibold text-sm mb-2">
            <AlertTriangle className="w-4 h-4 text-amber-600" />
            <span>Missing Asset References ({report.broken_references.length})</span>
          </div>
          <p className="text-xs text-amber-700 mb-3">
            The following image references inside <code className="font-mono bg-amber-100 px-1 rounded">{report.primary_markdown}</code> could not be found inside the ZIP file. Visible placeholders will be generated in the output PDF.
          </p>
          <div className="max-h-36 overflow-y-auto space-y-1.5">
            {report.broken_references.map((item, idx) => (
              <div key={idx} className="text-xs bg-white p-2 rounded border border-amber-200 flex justify-between items-center font-mono">
                <span className="text-slate-700">{item.reference}</span>
                <span className="text-red-600 font-semibold px-2 py-0.5 bg-red-50 rounded">NOT FOUND</span>
              </div>
            ))}
          </div>
        </div>
      )}

      {/* Action Button */}
      <div className="flex justify-end">
        <button
          onClick={onGenerate}
          disabled={isRendering}
          className="inline-flex items-center space-x-2 bg-blue-600 hover:bg-blue-700 disabled:bg-blue-300 text-white font-semibold px-6 py-3 rounded-lg shadow-md transition-all hover:shadow-lg"
        >
          <span>Generate A4 Printable PDF</span>
          <ArrowRight className="w-4 h-4" />
        </button>
      </div>
    </div>
  );
};
