pub fn get_compact_2col_css() -> &'static str {
    r#"
@page {
    size: A4;
    margin: 8mm 8mm 10mm 8mm;
    @bottom-right {
        content: "Page " counter(page);
        font-family: 'Canva Sans', 'Inter', system-ui, -apple-system, sans-serif;
        font-size: 5.5pt;
        color: #64748b;
    }
}

* {
    box-sizing: border-box;
}

body {
    font-family: 'Canva Sans', 'Inter', system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
    font-size: 6pt;
    line-height: 1.3;
    color: #0f172a;
    background-color: #ffffff;
    margin: 0;
    padding: 0;
}

.document-header {
    border-bottom: 1.5px solid #0f172a;
    padding-bottom: 4px;
    margin-bottom: 8px;
}

.document-title {
    font-family: 'Canva Sans', 'Inter', system-ui, sans-serif;
    font-size: 11pt;
    font-weight: 800;
    color: #0f172a;
    margin: 0 0 2px 0;
    letter-spacing: -0.01em;
}

.document-subtitle {
    font-family: 'Canva Sans', 'Inter', system-ui, sans-serif;
    font-size: 8pt;
    font-weight: 700;
    color: #1e293b;
    text-decoration: underline;
    margin-top: 2px;
}

/* Two-column layout container for main body content */
main {
    column-count: 2;
    column-gap: 6mm;
    column-fill: auto;
}

/* All Generic Headings - Plain text without any background box */
h1, h2, h3, h4, h5, h6 {
    font-family: 'Canva Sans', 'Inter', system-ui, sans-serif;
    color: #0f172a;
    break-after: avoid;
    page-break-after: avoid;
    break-inside: avoid;
    margin-top: 6px;
    margin-bottom: 3px;
    background: transparent !important;
    background-color: transparent !important;
    padding: 0 !important;
}

/* Level 1: Document Level Title */
h1 {
    font-size: 10pt;
    font-weight: 800;
    border-bottom: 1.5px solid #0f172a;
    padding-bottom: 2px;
    margin-top: 10px;
    margin-bottom: 6px;
    text-transform: uppercase;
}

/* Level 2: Major Section / Year / Session (e.g. 2022-Apr-May, Year 2021) - Plain bold text */
h2 {
    font-size: 8pt;
    font-weight: 700;
    color: #1e293b;
    margin-top: 8px;
    margin-bottom: 4px;
    display: block;
}

/* QUESTIONS ONLY - Soft Blue Background Highlight Box */
section.question > h2.question-title,
section.question > h3.question-title,
.question-title {
    margin-top: 8px;
    margin-bottom: 4px;
    background-color: #dbeafe !important;
    color: #1e40af !important;
    font-size: 7.5pt !important;
    font-weight: 700 !important;
    padding: 3px 6px !important;
    border-radius: 3px !important;
    display: block;
    width: 100%;
}

/* Subheadings / Subtopics (e.g. 1. Bus Topology:, Key Features) - Plain bold text */
h3, h4 {
    font-size: 6.5pt;
    font-weight: 700;
    color: #0f172a;
    margin-top: 5px;
    margin-bottom: 2px;
}

/* Level 5 & 6: Minor Subtopics */
h5, h6 {
    font-size: 6pt;
    font-weight: 700;
    color: #334155;
}

p {
    margin-top: 0;
    margin-bottom: 4px;
    text-align: justify;
}

/* Figures & Technical Diagrams */
figure {
    break-inside: avoid;
    page-break-inside: avoid;
    margin: 6px 0;
    text-align: center;
}

figure img {
    max-width: 100%;
    max-height: 120mm;
    height: auto;
    display: block;
    margin: 0 auto;
    border-radius: 2px;
}

figcaption {
    font-size: 5.5pt;
    color: #475569;
    margin-top: 3px;
    font-style: italic;
}

/* Broken Asset Placeholder */
.broken-image-placeholder {
    break-inside: avoid;
    margin: 6px 0;
    padding: 6px 8px;
    background-color: #fff5f5;
    border: 1px dashed #ef4444;
    border-radius: 3px;
    color: #991b1b;
    font-size: 5.5pt;
}

.broken-image-placeholder strong {
    display: block;
    margin-bottom: 2px;
}

/* Tables */
table {
    width: 100%;
    border-collapse: collapse;
    margin: 6px 0;
    font-size: 5.5pt;
    line-height: 1.2;
    break-inside: auto;
}

tr {
    break-inside: avoid;
    page-break-inside: avoid;
}

th, td {
    border: 0.5px solid #cbd5e1;
    padding: 3px 4px;
    text-align: left;
    vertical-align: top;
}

th {
    background-color: #f1f5f9;
    font-weight: 700;
    color: #0f172a;
}

/* Code Blocks */
pre, code {
    font-family: "Courier New", Courier, monospace;
}

pre {
    background-color: #f8fafc;
    border: 0.5px solid #e2e8f0;
    border-radius: 3px;
    padding: 4px 6px;
    font-size: 5.5pt;
    overflow-x: auto;
    break-inside: avoid;
    margin: 4px 0;
}

code {
    background-color: #f1f5f9;
    padding: 1px 3px;
    border-radius: 2px;
    font-size: 90%;
}

/* Lists */
ul, ol {
    margin-top: 0;
    margin-bottom: 4px;
    padding-left: 14px;
}

li {
    margin-bottom: 2px;
    font-size: 6pt;
    line-height: 1.3;
}

hr {
    border: 0;
    height: 0.5px;
    background: #cbd5e1;
    margin: 8px 0;
}
"#
}

pub fn get_academic_print_css() -> &'static str {
    get_compact_2col_css()
}
