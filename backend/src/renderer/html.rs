use crate::document::model::*;
use crate::renderer::css;
use v_htmlescape::escape;

pub struct HtmlRenderer;

impl HtmlRenderer {
    pub fn render_document(doc: &Document) -> String {
        let mut html = String::new();

        html.push_str("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n");
        html.push_str("<meta charset=\"UTF-8\">\n");
        html.push_str("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n");
        html.push_str(&format!("<title>{}</title>\n", escape(&doc.title)));
        html.push_str("<link rel=\"preconnect\" href=\"https://fonts.googleapis.com\">\n");
        html.push_str("<link rel=\"preconnect\" href=\"https://fonts.gstatic.com\" crossorigin>\n");
        html.push_str("<link href=\"https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700;800&display=swap\" rel=\"stylesheet\">\n");
        html.push_str("<style>\n");
        html.push_str(css::get_compact_2col_css());
        html.push_str("\n</style>\n</head>\n<body>\n");

        // Header section
        html.push_str("<header class=\"document-header\">\n");
        html.push_str(&format!("<h1 class=\"document-title\">{}</h1>\n", escape(&doc.title)));
        if let Some(ref unit) = doc.metadata.unit_title {
            if unit != &doc.title {
                html.push_str(&format!("<div class=\"document-subtitle\">{}</div>\n", escape(unit)));
            }
        }
        html.push_str("</header>\n<main>\n");

        // Render document blocks
        for block in &doc.blocks {
            html.push_str(&Self::render_block(block));
        }

        html.push_str("</main>\n</body>\n</html>");
        html
    }

    fn render_block(block: &Block) -> String {
        match block {
            Block::Heading { level, text, id } => {
                format!("<h{} id=\"{}\">{}</h{}>\n", level, escape(id), escape(text), level)
            }
            Block::Paragraph { inlines } => {
                let content = Self::render_inlines(inlines);
                format!("<p>{}</p>\n", content)
            }
            Block::QuestionGroup { title, blocks } => {
                let mut out = String::new();
                out.push_str("<section class=\"question\">\n");
                out.push_str(&format!("<h2 class=\"question-title\">{}</h2>\n", escape(title)));
                out.push_str("<div class=\"question-content\">\n");
                for b in blocks {
                    if let Block::Heading { text, .. } = b {
                        if text == title {
                            continue;
                        }
                    }
                    out.push_str(&Self::render_block(b));
                }
                out.push_str("</div>\n</section>\n");
                out
            }
            Block::Image { src, alt, resolved_path, is_missing, .. } => {
                if *is_missing {
                    format!(
                        "<div class=\"broken-image-placeholder\">\n  <strong>⚠️ Missing Reference</strong>\n  <div>File: <code>{}</code></div>\n</div>\n",
                        escape(src)
                    )
                } else {
                    let img_src = resolved_path.as_deref().unwrap_or(src);
                    format!(
                        "<figure>\n  <img src=\"{}\" alt=\"{}\">\n</figure>\n",
                        img_src,
                        escape(alt)
                    )
                }
            }
            Block::List { ordered, items } => {
                let tag = if *ordered { "ol" } else { "ul" };
                let mut out = format!("<{}>\n", tag);
                for item in items {
                    out.push_str("  <li>\n");
                    for b in item {
                        out.push_str(&Self::render_block(b));
                    }
                    out.push_str("  </li>\n");
                }
                out.push_str(&format!("</{}>\n", tag));
                out
            }
            Block::Table { headers, rows, alignments } => {
                let mut out = String::new();
                out.push_str("<table>\n");

                if !headers.is_empty() {
                    out.push_str("<thead>\n  <tr>\n");
                    for (i, cell) in headers.iter().enumerate() {
                        let align = alignments.get(i).map(|s| s.as_str()).unwrap_or("left");
                        out.push_str(&format!(
                            "    <th style=\"text-align: {};\">{}</th>\n",
                            align,
                            Self::render_inlines(cell)
                        ));
                    }
                    out.push_str("  </tr>\n</thead>\n");
                }

                out.push_str("<tbody>\n");
                for row in rows {
                    out.push_str("  <tr>\n");
                    for (i, cell) in row.iter().enumerate() {
                        let align = alignments.get(i).map(|s| s.as_str()).unwrap_or("left");
                        out.push_str(&format!(
                            "    <td style=\"text-align: {};\">{}</td>\n",
                            align,
                            Self::render_inlines(cell)
                        ));
                    }
                    out.push_str("  </tr>\n");
                }
                out.push_str("</tbody>\n</table>\n");
                out
            }
            Block::CodeBlock { language, code } => {
                let lang_attr = language
                    .as_ref()
                    .map(|l| format!(" class=\"language-\"{}\"", escape(l)))
                    .unwrap_or_default();
                format!("<pre><code{}>{}</code></pre>\n", lang_attr, escape(code))
            }
            Block::HorizontalRule => "<hr>\n".to_string(),
            Block::Blockquote { blocks } => {
                let mut out = String::new();
                out.push_str("<blockquote>\n");
                for b in blocks {
                    out.push_str(&Self::render_block(b));
                }
                out.push_str("</blockquote>\n");
                out
            }
        }
    }

    fn render_inlines(inlines: &[Inline]) -> String {
        let mut out = String::new();
        for inline in inlines {
            match inline {
                Inline::Text { text } => out.push_str(&escape(text).to_string()),
                Inline::Bold { content } => {
                    out.push_str("<strong>");
                    out.push_str(&Self::render_inlines(content));
                    out.push_str("</strong>");
                }
                Inline::Italic { content } => {
                    out.push_str("<em>");
                    out.push_str(&Self::render_inlines(content));
                    out.push_str("</em>");
                }
                Inline::Code { code } => {
                    out.push_str("<code>");
                    out.push_str(&escape(code).to_string());
                    out.push_str("</code>");
                }
                Inline::Link { text, url } => {
                    out.push_str(&format!("<a href=\"{}\">{}</a>", escape(url), escape(text)));
                }
                Inline::Image { src, alt } => {
                    out.push_str(&format!("<img src=\"{}\" alt=\"{}\">", src, escape(alt)));
                }
                Inline::LineBreak => out.push_str("<br>"),
            }
        }
        out
    }
}
