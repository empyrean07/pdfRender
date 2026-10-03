# MASTER AGENT PROMPT — Markdown/ZIP → Printable PDF Renderer

## PROJECT: Academic Markdown → Printable PDF Document Renderer

## ROLE

You are a senior full-stack engineer and document-rendering engineer.

Your task is to design and implement a production-quality application that accepts a ZIP archive containing:

- Markdown files (`.md`)
- Images (`.png`, `.jpg`, `.jpeg`, `.webp`, `.svg`)
- Diagrams
- Tables
- Other referenced assets

and converts the Markdown document into a professionally formatted, print-ready PDF.

The PDF must preserve the original content while applying a consistent academic/document layout.

The final application should allow the user to:

1. Upload a ZIP file.
2. Extract and validate its contents.
3. Detect the primary Markdown document.
4. Resolve all referenced assets.
5. Parse Markdown into a structured document representation.
6. Render the document into HTML.
7. Apply print-specific CSS.
8. Generate an A4 PDF.
9. Preview the generated PDF.
10. Download the PDF.

Do NOT treat this as a simple Markdown-to-PDF conversion task.

This is a document rendering pipeline.

---

# 1. SOURCE MATERIAL

The project will be tested using real academic Markdown documents.

The Markdown may contain:

- Unit titles
- Year/session headings
- Questions
- Answers
- Subheadings
- Bold/italic text
- Ordered lists
- Unordered lists
- Nested lists
- Tables
- Horizontal rules
- Images
- Technical diagrams
- Network diagrams
- Comparison tables
- Technical illustrations

The application must preserve the semantic relationship between text and nearby diagrams/images.

For example:

```text
Question
   ↓
Explanation
   ↓
Diagram
```

must remain together as much as possible.

Do NOT arbitrarily move diagrams away from the content they belong to.

---

# 2. CORE REQUIREMENT

The architecture MUST follow:

```text
ZIP
 ↓
Extraction
 ↓
Validation
 ↓
Markdown parsing
 ↓
Asset resolution
 ↓
Document AST / intermediate representation
 ↓
HTML generation
 ↓
CSS print layout
 ↓
Chromium/browser PDF rendering
 ↓
PDF
 ↓
Preview / Download
```

Do NOT implement the PDF layout by manually calculating:

- x coordinates
- y coordinates
- line positions
- image positions
- text wrapping
- page overflow

using a low-level PDF drawing library.

The browser layout engine should perform the document layout.

Preferred rendering architecture:

```text
Markdown
   ↓
HTML
   ↓
CSS
   ↓
Chromium
   ↓
PDF
```

---

# 3. TECHNOLOGY DIRECTION

Prefer the following architecture unless there is a strong technical reason to change it.

## Backend

Rust.

Recommended responsibilities:

- HTTP API
- ZIP extraction
- file validation
- Markdown parsing
- asset resolution
- document structure generation
- HTML generation
- rendering job management
- file storage
- PDF generation orchestration

Recommended framework:

```text
Axum
```

or another modern Rust HTTP framework if there is a compelling reason.

## Frontend

React.

Responsibilities:

- ZIP upload
- upload progress
- validation status
- rendering status
- PDF preview
- download
- error display

## PDF renderer

Use Chromium-based rendering.

Possible implementation:

```text
Playwright
```

or another reliable Chromium automation mechanism.

The important requirement is:

```text
HTML + CSS → Chromium → PDF
```

The Rust backend may invoke a dedicated rendering process/service if necessary.

Do NOT compromise PDF rendering quality merely to keep everything inside Rust.

---

# 4. PROJECT STRUCTURE

Create a clean monorepo.

Recommended structure:

```text
markdown-pdf-renderer/
│
├── backend/
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs
│   │   ├── api/
│   │   │   ├── mod.rs
│   │   │   ├── upload.rs
│   │   │   ├── render.rs
│   │   │   ├── preview.rs
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
│   │   │   ├── resolver.rs
│   │   │   ├── image.rs
│   │   │   └── metadata.rs
│   │   ├── document/
│   │   │   ├── mod.rs
│   │   │   ├── model.rs
│   │   │   └── structure.rs
│   │   ├── renderer/
│   │   │   ├── mod.rs
│   │   │   ├── html.rs
│   │   │   ├── template.rs
│   │   │   ├── css.rs
│   │   │   └── pdf.rs
│   │   ├── jobs/
│   │   │   ├── mod.rs
│   │   │   └── render_job.rs
│   │   └── config.rs
│   ├── templates/
│   │   ├── document.html
│   │   └── print.css
│   └── tests/
│
├── frontend/
│   ├── package.json
│   └── src/
│       ├── components/
│       ├── pages/
│       ├── hooks/
│       ├── api/
│       └── App.tsx
│
├── storage/
│   ├── uploads/
│   ├── extracted/
│   ├── generated/
│   └── previews/
│
├── examples/
│   └── sample-document/
│
├── docker/
├── docker-compose.yml
├── README.md
└── AGENT.md
```

You may modify this structure if necessary, but maintain clear separation of concerns.

---

# 5. ZIP INGESTION

The application receives:

```text
document.zip
```

Example:

```text
document.zip
│
├── UNIT_1.md
├── image.png
├── image 1.png
├── image 2.png
├── diagram.jpg
├── another-diagram.webp
└── ...
```

The system must:

1. Accept ZIP.
2. Validate ZIP.
3. Prevent path traversal attacks.
4. Extract into an isolated temporary directory.
5. Discover Markdown files.
6. Discover referenced assets.
7. Determine the primary Markdown file.
8. Validate asset references.

NEVER blindly extract paths such as:

```text
../../something
```

or absolute paths.

Normalize and validate every extracted path.

---

# 6. MARKDOWN FILE DISCOVERY

Do NOT hard-code `UNIT_1.md` as the only possible filename.

Instead:

1. Find all `*.md` files.
2. If exactly one Markdown file exists, use it as the primary document.
3. If multiple Markdown files exist, analyze filenames/structure and either identify a likely entry document or clearly request selection.
4. Never silently choose an arbitrary file.

---

# 7. MARKDOWN PARSING

Use a proper Markdown parser.

DO NOT parse Markdown using regular expressions alone.

The parser should support at minimum:

```text
# H1
## H2
### H3
#### H4

paragraphs

**bold**

*italic*

`inline code`

ordered lists

unordered lists

nested lists

tables

images

links

horizontal rules
```

The parser should produce an intermediate representation / AST.

Example:

```rust
enum Block {
    Heading {
        level: u8,
        text: String,
    },

    Paragraph {
        content: Vec<Inline>,
    },

    List {
        ordered: bool,
        items: Vec<ListItem>,
    },

    Table(Table),

    Image(ImageBlock),

    HorizontalRule,
}
```

---

# 8. DOCUMENT MODEL

Create a semantic document model.

Example:

```rust
struct Document {
    title: Option<String>,
    blocks: Vec<Block>,
    assets: Vec<Asset>,
}
```

If the structure of the Markdown allows semantic grouping, represent it.

For example:

```text
Document
│
├── Unit
│
├── Year/Session
│   │
│   ├── Question
│   │   ├── Answer
│   │   ├── Subheading
│   │   ├── List
│   │   ├── Table
│   │   └── Image
│   │
│   └── Question
│
└── Year/Session
```

However, do not invent semantic meaning that isn't present in the source.

If the Markdown only contains generic headings, preserve the source hierarchy.

---

# 9. IMAGE / ASSET RESOLUTION

This is a critical requirement.

Markdown may contain:

```md
![image.png](image.png)
```

or:

```md
![Network Diagram](images/network.png)
```

or:

```md
![diagram](../assets/diagram.jpg)
```

The system must resolve relative paths relative to the Markdown file.

For every referenced asset:

```text
Markdown reference
        ↓
normalized path
        ↓
resolved filesystem path
        ↓
validated asset
```

Support:

```text
PNG
JPG
JPEG
WEBP
SVG
```

where technically possible.

---

# 10. BROKEN IMAGE HANDLING

If Markdown references an asset that does not exist, do NOT silently ignore it.

The validation stage should report:

```text
Broken asset reference:

Markdown:
UNIT_1.md

Reference:
image.png

Status:
NOT FOUND
```

The application may either:

- stop rendering
- or generate a visible placeholder

depending on configuration.

Default behavior should be safe and clearly report the problem.

---

# 11. IMAGE RENDERING

Images should be rendered responsively.

Default:

```css
img {
    max-width: 100%;
    height: auto;
}
```

Technical diagrams should generally be centered.

Do not upscale low-resolution images unnecessarily.

Do not aggressively compress source images.

Preserve source quality for printing.

---

# 12. IMAGE PAGE BREAKS

Never intentionally split a diagram across pages.

Use print CSS such as:

```css
figure {
    break-inside: avoid;
    page-break-inside: avoid;
}
```

The same principle should apply to:

- small tables
- captions
- question headings
- important grouped content

For example:

```text
Question
Explanation
Diagram
```

should remain together when physically possible.

If it cannot fit on the current page, move the group to the next page.

---

# 13. TABLE RENDERING

Support Markdown tables.

Example:

```md
| Parameter | OSI | TCP/IP |
|---|---|---|
| Layers | 7 | 4 |
```

Render as proper HTML:

```html
<table>
    <thead>...</thead>
    <tbody>...</tbody>
</table>
```

Apply print-friendly CSS:

```css
table {
    width: 100%;
    border-collapse: collapse;
}

th,
td {
    border: 1px solid #444;
    padding: 6px 8px;
    vertical-align: top;
}
```

Tables must remain readable when printed.

Avoid splitting rows across pages.

---

# 14. HTML RENDERING

Generate a complete HTML document:

```html
<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>Document</title>
    <style>
        /* print CSS */
    </style>
</head>

<body>

<header>
    ...
</header>

<main>
    ...
</main>

<footer>
    ...
</footer>

</body>
</html>
```

The HTML should be valid and suitable for Chromium rendering.

---

# 15. A4 PRINT DESIGN

The target paper size is:

```text
A4
```

Use:

```css
@page {
    size: A4;
    margin: 18mm 15mm 18mm 18mm;
}
```

Exact margins may later be configurable.

The default output must look like professional academic study material.

---

# 16. TYPOGRAPHY

Use readable academic typography.

Suggested ranges:

```text
Document title:
20–24px

Major section:
16–18px

Question:
13–15px

Body:
10.5–12px

Table:
9–11px
```

Do not use unnecessarily decorative fonts.

Prioritize:

- readability
- printability
- consistent hierarchy

---

# 17. DOCUMENT HIERARCHY

The renderer should visually distinguish:

```text
Document Title
        ↓
Major Section / Year
        ↓
Question
        ↓
Subheading
        ↓
Body
        ↓
List / Table / Diagram
```

The exact semantic interpretation must come from the Markdown structure.

---

# 18. HEADER AND FOOTER

Add a professional header/footer system.

Example header:

```text
COMPUTER NETWORKS                         UNIT 1
```

Example footer:

```text
Academic Notes                              Page 5
```

Page numbering should be generated automatically.

Do not hard-code page numbers.

---

# 19. PAGE BREAK LOGIC

Implement print-aware CSS:

```css
h1,
h2,
h3,
h4 {
    break-after: avoid;
}

figure {
    break-inside: avoid;
}

table {
    break-inside: auto;
}

tr {
    break-inside: avoid;
}
```

Do not overuse explicit page breaks.

Prefer natural browser pagination.

---

# 20. QUESTION GROUPING

If the source structure indicates:

```text
Question
Answer
Subheading
Diagram
```

treat them as a logical group where possible.

Example:

```html
<section class="question">

    <h2 class="question-title">
        1. Write a short note...
    </h2>

    <div class="question-content">
        ...
    </div>

</section>
```

Avoid orphaned question headings.

Bad:

```text
PAGE 4

1. Explain Circuit Switching.

PAGE 5

Answer...
```

Prefer moving the heading to the next page when enough content can fit there.

---

# 21. VALIDATION PIPELINE

Before rendering, run:

```text
ZIP validation
      ↓
Markdown validation
      ↓
Asset validation
      ↓
Document parsing
      ↓
HTML generation
      ↓
PDF rendering
```

Example validation report:

```text
Document Validation
────────────────────────

Markdown files:        1
Images:                8
Tables:                5

Broken references:     0
Unsupported assets:    0
Parsing errors:        0

Status: READY
```

---

# 22. API DESIGN

Implement APIs similar to:

```text
POST /api/documents/upload
POST /api/documents/:id/validate
POST /api/documents/:id/render
GET  /api/jobs/:id
GET  /api/documents/:id/pdf
```

Example upload response:

```json
{
    "documentId": "abc123",
    "status": "uploaded"
}
```

Example validation response:

```json
{
    "status": "valid",
    "markdownFiles": 1,
    "images": 8,
    "tables": 5,
    "errors": []
}
```

Example render response:

```json
{
    "jobId": "job123",
    "status": "queued"
}
```

Example job response:

```json
{
    "status": "completed",
    "progress": 100
}
```

---

# 23. FRONTEND

Create a clean UI.

Main page:

```text
┌─────────────────────────────────────┐
│       Markdown PDF Renderer         │
│                                     │
│   Upload your document ZIP           │
│                                     │
│     ┌───────────────────────┐       │
│     │   Drag & Drop ZIP     │       │
│     │                       │       │
│     │    or Browse Files    │       │
│     └───────────────────────┘       │
│                                     │
└─────────────────────────────────────┘
```

After upload:

```text
Document
UNIT_1.zip

✓ ZIP extracted
✓ Markdown detected
✓ Assets detected
✓ Validation successful

[ Generate PDF ]
```

During rendering:

```text
Rendering PDF...

[████████████████░░░░] 80%
```

After completion:

```text
PDF Ready

[ Preview PDF ]

[ Download PDF ]
```

---

# 24. PDF PREVIEW

Provide an embedded PDF viewer or browser-based preview.

Allow:

- page navigation
- zoom where practical
- download

---

# 25. ERROR HANDLING

Errors must be user-readable.

Bad:

```text
ENOENT
```

Good:

```text
Unable to render the document.

The Markdown file references:

images/network-topology.png

but that file could not be found inside the uploaded ZIP.

Please verify the asset path.
```

Backend logs may contain technical details, but frontend errors should be understandable.

---

# 26. SECURITY

Because ZIP files are uploaded by users, implement:

- ZIP path traversal protection
- maximum ZIP size
- maximum extracted size
- maximum file count
- allowed file extensions
- temporary directory isolation
- cleanup after rendering
- safe filename handling

Do not allow arbitrary executable files to be executed from uploaded archives.

Only process expected document/assets.

---

# 27. TEMPORARY STORAGE

Use:

```text
storage/
├── uploads/
├── extracted/
├── generated/
└── previews/
```

Each job should have an isolated identifier:

```text
storage/extracted/job_123/
storage/generated/job_123/
```

Clean temporary files according to a configurable retention policy.

---

# 28. RENDERING ENGINE

The PDF renderer should:

1. Start/load Chromium.
2. Load generated HTML.
3. Wait for all assets to load.
4. Wait for fonts if required.
5. Render using A4 page settings.
6. Generate PDF.
7. Verify PDF exists.
8. Return PDF path.

Do not generate the PDF before images have finished loading.

---

# 29. IMAGE LOADING

Before PDF generation, verify:

```text
document loaded
AND
images loaded
AND
fonts loaded
```

If using browser JavaScript, use an equivalent image-loading wait mechanism.

Do not leave image loading timing to chance.

---

# 30. SELF-CONTAINED HTML

Prefer a rendering strategy where generated HTML can be rendered without relying on the frontend dev server.

The PDF renderer should operate independently.

For local assets, use safe absolute paths or data URLs as appropriate.

Do not depend on:

```text
http://localhost:5173
```

for PDF generation.

---

# 31. TEST DOCUMENT

Create a test fixture containing:

```md
# Test Document

## Section

Paragraph.

**Bold**

*Italic*

### Lists

- Item 1
- Item 2
  - Nested item

### Table

| A | B |
|---|---|
| 1 | 2 |

### Image

![Test Image](test.png)
```

Also test:

- very large images
- very small images
- missing images
- long tables
- long paragraphs
- multiple pages
- nested lists
- multiple headings

---

# 32. REAL DOCUMENT TEST

After the basic renderer works, use the supplied academic Markdown and assets as the primary integration test.

Do NOT modify the source content merely to make rendering easier.

The source Markdown is the source of truth.

If rendering requires special handling, implement that in the renderer.

---

# 33. CONTENT PRESERVATION

The renderer must NOT:

- rewrite answers
- summarize content
- correct grammar
- remove content
- invent explanations
- change technical terminology
- reorder questions
- reorder diagrams
- replace source images

unless explicitly requested.

The job of the system is:

```text
FORMAT
not
REWRITE
```

---

# 34. NO AI CONTENT GENERATION IN V1

Version 1 is a deterministic document renderer.

Do not introduce an LLM for:

- rewriting
- summarization
- question detection
- answer generation
- image interpretation

unless explicitly requested later.

Same input should produce substantially the same document.

---

# 35. CONFIGURABLE PRINT THEMES

Architect the renderer so the visual theme can later be changed.

Example:

```text
themes/
├── academic/
├── minimal/
└── compact/
```

Version 1 only needs:

```text
academic
```

Keep layout styles in CSS/templates rather than hard-coding every style into Rust.

---

# 36. PRINT QUALITY

The final PDF should be suitable for:

- digital reading
- A4 printing
- academic notes
- exam preparation
- sharing

Avoid:

- excessive whitespace
- extremely small fonts
- unreadably small diagrams
- inconsistent visual hierarchy

---

# 37. FUTURE FEATURES

Architect without implementing unless easy:

- Table of Contents
- Bookmarks
- Multiple Markdown files
- DOCX export
- HTML export
- Custom page size
- Custom margins
- Custom fonts
- Theme selection
- Cover page
- Watermark
- Chapter numbering
- Question numbering
- Image captions

Do NOT over-engineer V1.

---

# 38. DEVELOPMENT STRATEGY

Implement incrementally.

## Phase 1

Create project skeleton.

## Phase 2

Implement ZIP extraction.

## Phase 3

Implement Markdown parsing.

## Phase 4

Implement asset resolver.

## Phase 5

Implement HTML renderer.

## Phase 6

Implement print CSS.

## Phase 7

Implement Chromium PDF generation.

## Phase 8

Implement API.

## Phase 9

Implement React frontend.

## Phase 10

Implement PDF preview/download.

## Phase 11

Integration testing with the actual academic document.

## Phase 12

Pagination and visual-quality refinement.

After each phase:

```text
build
test
verify
then continue
```

Do not attempt all phases simultaneously.

---

# 39. TESTING REQUIREMENTS

Create automated tests for:

## ZIP

- valid ZIP
- invalid ZIP
- path traversal
- multiple Markdown files
- missing Markdown

## Markdown

- headings
- paragraphs
- lists
- nested lists
- tables
- images
- horizontal rules

## Assets

- existing image
- missing image
- spaces in filename
- nested directories
- relative paths

## Rendering

- one-page document
- multi-page document
- large image
- table
- long question
- question + diagram

---

# 40. IMPORTANT PAGE-LAYOUT TEST

Create a test case where:

```text
Question
+
long paragraph
+
large diagram
```

is close to the bottom of the page.

Verify that the renderer does not produce:

```text
Question on page 1
Diagram on page 2
```

if the diagram can logically be moved as a group.

Also test:

```text
Heading
```

at the bottom of a page with its content on the next page.

---

# 41. LOGGING

Backend should log stages:

```text
[UPLOAD]
[EXTRACT]
[VALIDATE]
[PARSE]
[RESOLVE_ASSETS]
[GENERATE_HTML]
[RENDER_PDF]
[CLEANUP]
```

Example:

```text
[PARSE] Markdown parsed successfully
[ASSETS] 12 assets discovered
[ASSETS] 12/12 references resolved
[HTML] Generated successfully
[PDF] Chromium rendering started
[PDF] Rendering complete
```

Avoid logging sensitive file contents.

---

# 42. README

Create a detailed README containing:

- project overview
- architecture
- prerequisites
- installation
- development setup
- Chromium setup
- backend startup
- frontend startup
- Docker setup
- API documentation
- example ZIP structure
- troubleshooting
- production deployment

---

# 43. DOCKER

If practical, provide Docker support.

The container should include whatever is necessary for Chromium PDF generation.

Do not assume Chromium exists on every deployment machine.

Document required dependencies clearly.

---

# 44. ENVIRONMENT CONFIGURATION

Use environment variables for:

```text
PORT
STORAGE_DIR
MAX_UPLOAD_SIZE
MAX_EXTRACTED_SIZE
CHROMIUM_PATH
RENDER_TIMEOUT
```

Provide:

```text
.env.example
```

Do not commit secrets.

---

# 45. PERFORMANCE

Do not optimize prematurely.

Structure the system so:

```text
upload
parse
render
```

are separate stages.

Potential future optimization:

- Chromium process reuse
- parallel image validation
- cached assets
- job queues

Do not implement complex distributed infrastructure in V1.

---

# 46. ACCEPTANCE CRITERIA

The implementation is successful when I can upload:

```text
UNIT_1.zip
```

containing:

```text
UNIT_1.md
multiple images
diagrams
tables
```

and the system:

```text
extracts ZIP
finds Markdown
resolves images
parses Markdown
generates HTML
renders PDF
```

without manual modification of the source Markdown.

The generated PDF:

- is A4
- contains all source content
- contains all referenced images
- preserves tables
- preserves headings
- preserves lists
- has professional typography
- has page numbers
- has consistent margins
- does not arbitrarily split diagrams
- does not create unnecessary blank pages
- is suitable for printing

The UI allows:

```text
Upload ZIP
      ↓
Validate
      ↓
Generate
      ↓
Preview
      ↓
Download
```

---

# 47. DO NOT DO THESE THINGS

Do NOT:

1. Build a basic text-to-PDF converter.
2. Use regex as the main Markdown parser.
3. Manually position every PDF element.
4. Rewrite the source content.
5. Discard images that are difficult to render.
6. Silently ignore broken references.
7. Hard-code the name `UNIT_1.md`.
8. Hard-code image filenames.
9. Assume every ZIP has the same structure.
10. Depend on the frontend dev server for PDF generation.
11. Generate the PDF before images load.
12. Allow ZIP path traversal.
13. Over-engineer the first version.
14. Introduce an LLM unnecessarily.
15. Modify the source Markdown just to accommodate renderer limitations.

---

# 48. AGENT EXECUTION RULES

Before writing substantial code:

1. Inspect the repository.
2. Inspect all existing files.
3. Identify the current architecture.
4. Identify available dependencies.
5. Inspect the supplied sample Markdown and assets if available.
6. Create an implementation plan.
7. Implement incrementally.

Do not blindly overwrite an existing project.

If an existing implementation is present:

```text
understand → preserve useful parts → refactor where necessary
```

rather than:

```text
delete everything → rewrite
```

---

# 49. FIRST TASK

Your first task is NOT to immediately write the entire application.

First:

### Step 1

Inspect the repository.

### Step 2

Inspect the provided sample ZIP/document structure.

### Step 3

Analyze:

- Markdown syntax used
- image formats
- image references
- directory structure
- tables
- headings
- lists
- special cases

### Step 4

Create:

```text
IMPLEMENTATION_PLAN.md
```

containing:

- Architecture
- Data flow
- Modules
- Dependencies
- Rendering strategy
- Security considerations
- Testing strategy
- Implementation phases

### Step 5

Then implement Phase 1.

After each major phase:

```text
build
test
report result
continue
```

---

# 50. SPECIAL REQUIREMENT FOR THE PROVIDED ACADEMIC MATERIAL

The provided Markdown is not a generic article.

It represents academic/examination study material.

The renderer must preserve its organization, including structures such as:

```text
UNIT
    ↓
YEAR / SESSION
    ↓
QUESTION
    ↓
ANSWER
    ↓
SUBTOPIC
    ↓
LIST / TABLE / DIAGRAM
```

For example, if the source contains:

```text
### 2021 (NOV-DEC)

### 1. Write a short note on network topology.

[answer content]

### 1. Bus Topology

[content]

![diagram](image.png)
```

then the rendered PDF should visually communicate the same hierarchy.

Do not flatten the document into a sequence of generic paragraphs.

Diagrams must remain close to the explanatory content that precedes/follows them.

Tables must remain readable when printed on A4.

Questions should not become separated from their immediate answer heading when avoidable.

Do not change the wording of the academic content.

The renderer's job is formatting, layout, pagination, and asset management — not content modification.

---

# 51. FUNDAMENTAL DESIGN PRINCIPLE

The final product should feel like:

```text
"Upload a folder of Markdown + diagrams
        ↓
Get a professionally typeset academic PDF"
```

It should NOT feel like:

```text
"Convert Markdown syntax directly into PDF."
```

The fundamental design principle is:

```text
SOURCE CONTENT
      ↓
STRUCTURED DOCUMENT
      ↓
PRINT LAYOUT
      ↓
PDF
```

Keep the source content intact.

Keep the rendering deterministic.

Keep the architecture modular.

Prioritize print quality and correct pagination.
