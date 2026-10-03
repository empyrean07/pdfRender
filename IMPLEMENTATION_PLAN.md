# IMPLEMENTATION PLAN: Academic Markdown/ZIP to Printable PDF Renderer

## 1. Executive Summary & Architecture Overview

The system is a production-grade, deterministic document rendering pipeline that converts user-uploaded ZIP archives containing Markdown files (`.md`) and referenced assets (images, diagrams, figures) into professionally formatted, print-ready A4 PDF documents.

### High-Level Architecture Flow
```text
┌──────────────┐     ┌──────────────────┐     ┌──────────────────┐
│   ZIP File   │ ──> │ Rust Axum Backend│ ──> │ Markdown Parser  │
│  (User Upload)     │  Ingestion & Storage   │  & AST Construction│
└──────────────┘     └──────────────────┘     └──────────────────┘
                                                       │
                                                       ▼
┌──────────────┐     ┌──────────────────┐     ┌──────────────────┐
│  A4 PDF Out  │ <── │ Chromium Renderer│ <── │ Asset Resolution │
│ (Download/   │     │ (Playwright/Node)│     │ & HTML/CSS Gen   │
│   Preview)   │     └──────────────────┘     └──────────────────┘
└──────────────┘
```

---

## 2. Technology Stack & Dependencies

### Backend (Rust)
- **Framework**: `axum` (v0.7+) for HTTP API
- **Async Runtime**: `tokio` (full features)
- **Serialization**: `serde`, `serde_json`
- **Multipart/Upload Handling**: `axum::extract::Multipart`
- **Archive Extraction**: `zip-rs` with path traversal validation (`zip::ZipArchive`)
- **Markdown Parsing**: `pulldown-cmark` (producing AST / events)
- **Asset / Image Validation**: `image` crate (type/dimension validation) & `mime_guess`
- **Templating**: `askama` or `tera` / dynamic HTML generator for clean semantic markup
- **Tracing & Logging**: `tracing`, `tracing-subscriber`

### Rendering Engine (Chromium Automation)
- **Driver**: Node.js script using `playwright` (or `puppeteer-core`) launched as a renderer worker by Rust backend.
- **Wait Mechanisms**: Ensures `DOMContentLoaded`, all `<img>` `load` events, `@font-face` loading, and network idle before calling `page.pdf()`.

### Frontend (React + Vite + TypeScript)
- **UI Framework**: React 18 + TypeScript + Vite
- **Styling**: Tailwind CSS for clean academic interface
- **Icons**: `lucide-react`
- **PDF Viewer**: Embedded HTML5 object/iframe preview with page controls and fallback view.

---

## 3. Data Flow & Pipeline Stages

1. **Ingestion & Storage**:
   - `POST /api/documents/upload` accepts `multipart/form-data` ZIP archive.
   - Saves archive into `storage/uploads/{document_id}.zip`.
2. **Extraction & Security Validation**:
   - Extract ZIP into `storage/extracted/{document_id}/`.
   - Protect against ZIP slip / path traversal (`../`), enforce maximum extraction size (e.g., 50MB) and maximum file count (e.g., 500 files).
3. **Markdown File Discovery**:
   - Scan extracted directory for `*.md` files.
   - If 1 `.md` file, automatically select as primary document. If multiple, identify primary document or prompt user.
4. **Markdown Parsing & AST Construction**:
   - Parse Markdown into structured AST (`Document`, `Block`, `Inline`).
   - Group Question-Answer blocks (`<section class="question">`) and retain document hierarchy (Unit -> Year/Session -> Question -> Answer -> Subheading -> Figure/Table).
5. **Asset Resolution & Validation**:
   - Extract image references from AST.
   - Resolve relative paths relative to primary Markdown file location.
   - Check filesystem existence and format support (`.png`, `.jpg`, `.jpeg`, `.webp`, `.svg`).
   - Generate validation report (Status: READY / BROKEN_REFERENCES).
6. **HTML & Print CSS Generation**:
   - Render AST into semantic HTML5 document.
   - Convert image paths to absolute file URIs or embedded Data URIs for browser security context.
   - Inject Academic Print CSS (`@page { size: A4; margin: 18mm 15mm 18mm 18mm; }`, page-break rules, typography, headers/footers with dynamic page numbers).
7. **Chromium PDF Generation**:
   - Spawn rendering worker to navigate to generated HTML.
   - Wait for `window.imagesLoaded` and document ready.
   - Execute PDF printing with exact A4 specs.
   - Save resulting PDF in `storage/generated/{document_id}.pdf`.

---

## 4. Module Breakdown & Monorepo Structure

```text
markdown-pdf-renderer/
├── backend/
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs
│   │   ├── api/
│   │   │   ├── mod.rs
│   │   │   ├── upload.rs
│   │   │   ├── validate.rs
│   │   │   ├── render.rs
│   │   │   ├── jobs.rs
│   │   │   └── download.rs
│   │   ├── archive/
│   │   │   ├── mod.rs
│   │   │   └── extractor.rs
│   │   ├── markdown/
│   │   │   ├── mod.rs
│   │   │   ├── parser.rs
│   │   │   ├── ast.rs
│   │   │   └── resolver.rs
│   │   ├── assets/
│   │   │   ├── mod.rs
│   │   │   └── resolver.rs
│   │   ├── document/
│   │   │   ├── mod.rs
│   │   │   └── model.rs
│   │   ├── renderer/
│   │   │   ├── mod.rs
│   │   │   ├── html.rs
│   │   │   ├── css.rs
│   │   │   └── pdf.rs
│   │   ├── jobs/
│   │   │   ├── mod.rs
│   │   │   └── manager.rs
│   │   └── config.rs
│   └── templates/
│       ├── document.html
│       └── print.css
├── renderer-service/
│   ├── package.json
│   └── render.js
├── frontend/
│   ├── package.json
│   ├── vite.config.ts
│   └── src/
│       ├── components/
│       │   ├── FileUpload.tsx
│       │   ├── ValidationReport.tsx
│       │   ├── ProgressTracker.tsx
│       │   ├── PdfPreviewer.tsx
│       │   └── Header.tsx
│       ├── api/
│       │   └── client.ts
│       └── App.tsx
├── storage/
│   ├── uploads/
│   ├── extracted/
│   ├── generated/
│   └── previews/
├── examples/
│   └── sample-academic-doc/
│       ├── UNIT_1.md
│       └── images/
├── docker/
│   ├── Dockerfile.backend
│   └── Dockerfile.frontend
├── docker-compose.yml
├── README.md
└── IMPLEMENTATION_PLAN.md
```

---

## 5. Security Considerations

1. **Path Traversal / ZIP Slip Protection**:
   - Every file entry in ZIP is sanitized: reject any filename containing `..` or absolute paths leading outside the target directory.
2. **Resource Limits**:
   - Max Upload Size: 25 MB
   - Max Extracted Size: 100 MB
   - Max File Count: 500 files
   - Process Execution Timeout: 60s for rendering jobs
3. **Isolated Temporary Storage**:
   - Unique UUID subfolders per document job.
   - Automated storage cleanup task for expired temporary jobs.

---

## 6. Implementation Phases & Milestones

### Phase 1: Project Skeleton & Directory Setup
- Create `backend/`, `frontend/`, `renderer-service/`, `storage/`, `examples/` workspace structures.
- Setup Cargo workspace/package and Node dependencies.

### Phase 2: ZIP Extraction & Validation Engine
- Implement `extractor.rs` with ZipSlip protection and size checks.
- Add Markdown file discovery logic.

### Phase 3: Markdown AST Parsing & Academic Hierarchy Structuring
- Implement `parser.rs` and `ast.rs` using `pulldown-cmark`.
- Group headings, paragraphs, lists, tables, code blocks, and images into structured `Block` elements.

### Phase 4: Asset Resolver & Broken Reference Checker
- Implement `assets/resolver.rs`.
- Resolve Markdown relative paths (e.g. `images/diag.png`, `../assets/fig.jpg`).
- Return comprehensive JSON validation report.

### Phase 5: Semantic HTML & Print CSS Layout Engine
- Build dynamic HTML generator and `templates/print.css`.
- Implement A4 pagination CSS (`@page`, `break-inside: avoid` on `<figure>`, `section.question`, `tr`).
- Format headers & footers with page numbers.

### Phase 6: Chromium PDF Rendering Orchestrator
- Create Node/Playwright sidecar `renderer-service/render.js`.
- Invoke from Rust `pdf.rs` with exact A4 print parameters.

### Phase 7: REST API & Background Job Manager
- Axum endpoints for upload, validate, render job status polling, preview, and download.

### Phase 8: React Frontend Implementation
- Modern UI for uploading ZIP, viewing live validation metrics, progress tracking, embedded PDF preview, and download.

### Phase 9: Test Fixtures & Automated Integration Testing
- Create sample academic document ZIP fixture (`examples/sample-academic-doc/`).
- Unit tests for AST, ZIP extraction, asset resolution, and PDF generation.

---

## 7. Acceptance Criteria Verification

- Upload `sample-academic-doc.zip` -> Successfully extracts & validates without error.
- Validates missing assets and reports broken links if any exist.
- Renders print-ready A4 PDF with academic styling, centered figures, page numbers, proper table formatting, and zero orphaned section titles.
- Download and PDF preview work flawlessly in the React web frontend.
