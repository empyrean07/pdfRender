# Academic Markdown/ZIP → Printable PDF Renderer

Production-grade document rendering pipeline that converts ZIP archives containing Markdown files (`.md`) and referenced technical diagrams/images into professionally typeset, print-ready A4 PDF documents.

---

## 🌟 Architectural Features

- **Semantic AST Document Model**: Parses Markdown into a structured intermediate AST supporting academic hierarchies (`Unit` -> `Year/Session` -> `Question` -> `Answer` -> `Table/Diagram`).
- **Path Traversal Safe Extraction**: Ingests `.zip` archives with zip-slip protection, path normalization, and resource size caps.
- **Self-Contained HTML Print Engine**: Generates standalone HTML5 documents with base64 embedded assets and specialized academic CSS (`@page { size: A4; margin: 18mm 15mm 18mm 18mm; }`).
- **Smart Pagination & Break Prevention**: Grouping `<section class="question">` and `figure` elements with `break-inside: avoid` to prevent orphaned headings or split diagrams.
- **Headless Chromium Automation**: Uses Playwright Chromium worker waiting for image and font loading before rasterizing A4 PDFs.
- **Interactive React Interface**: Modern Vite + React + Tailwind interface for drag-and-drop upload, live broken-asset detection, rendering progress tracking, and embedded PDF/HTML previewing.

---

## 🏗️ Repository Architecture

```text
markdown-pdf-renderer/
├── backend/                  # Axum Rust HTTP API & Document Engine
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs           # API routing & Axum server
│   │   ├── config.rs         # Storage paths & limits
│   │   ├── archive/          # Path-safe ZIP extraction
│   │   ├── markdown/         # pulldown-cmark parser & AST
│   │   ├── assets/           # Relative asset resolution & base64 embedding
│   │   ├── document/         # Document & Block data models
│   │   ├── renderer/         # HTML generator, print CSS & PDF orchestrator
│   │   ├── jobs/             # Async rendering job state tracker
│   │   └── api/              # HTTP REST handlers
├── renderer-service/         # Node.js Playwright Chromium PDF Worker
│   ├── package.json
│   └── render.js
├── frontend/                 # React 18 + Vite + Tailwind UI
│   ├── package.json
│   └── src/
├── storage/                  # Working directories
│   ├── uploads/
│   ├── extracted/
│   ├── generated/
│   └── previews/
├── examples/                 # Sample academic document fixtures
│   ├── sample-academic-doc/
│   └── UNIT_1.zip
├── IMPLEMENTATION_PLAN.md    # Detailed system specification & phase plan
└── README.md
```

---

## 🚀 Quick Start Guide

### Prerequisites
- **Rust** 1.80+ (`cargo`)
- **Node.js** v20+ (`node`, `npm`)

### 1. Setup Renderer Service (Chromium)
```bash
cd renderer-service
npm install
npx playwright install chromium
```

### 2. Start Backend API Server
```bash
cd backend
cargo run
# Listens on http://127.0.0.1:3001
```

### 3. Start Frontend Interface
```bash
cd frontend
npm install
npm run dev
# Open http://localhost:5173
```

---

## 📡 REST API Reference

| Endpoint | Method | Description |
|---|---|---|
| `/api/documents/upload` | `POST` | Upload multipart `file` (`.zip`). Returns `document_id`. |
| `/api/documents/:id/validate` | `POST` | Validates extracted Markdown & reports broken asset references. |
| `/api/documents/:id/render` | `POST` | Queues async PDF rendering job. Returns `job_id`. |
| `/api/jobs/:id` | `GET` | Polls render job progress (`queued`, `processing`, `completed`, `failed`). |
| `/api/documents/:id/pdf` | `GET` | Downloads / streams generated A4 PDF file. |
| `/api/documents/:id/preview-html` | `GET` | Streams rendered HTML layout for inline browser previewing. |

---

## 📦 Example ZIP File Structure

```text
UNIT_1.zip
│
├── UNIT_1.md                 # Primary Markdown document
└── images/
    ├── network-topology.svg  # Technical diagram
    └── architecture.png      # Referenced figure
```

---

## 🧪 Testing with Sample Fixture

To test the system immediately with the included academic sample document:

1. Open the UI at `http://localhost:5173`.
2. Drag and drop `examples/UNIT_1.zip` into the upload zone.
3. Review the validation metrics (1 Markdown file, 1 diagram, 1 table, 6 questions).
4. Click **Generate A4 Printable PDF**.
5. Preview the typeset PDF directly in the browser or download the PDF file.
