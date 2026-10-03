use crate::document::model::*;
use pulldown_cmark::{Alignment, Event, Options, Parser as CmarkParser, Tag, TagEnd};

pub struct MarkdownParser;

impl MarkdownParser {
    pub fn parse(content: &str) -> Document {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_FOOTNOTES);
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TASKLISTS);

        let parser = CmarkParser::new_ext(content, options);

        let mut blocks: Vec<Block> = Vec::new();
        let mut metadata = DocumentMetadata::default();
        let mut document_title = String::from("Academic Document");

        let events: Vec<Event> = parser.collect();
        let mut idx = 0;

        while idx < events.len() {
            if let Some((block, next_idx)) = Self::parse_block(&events, idx, &mut metadata) {
                if let Block::Heading { level: 1, ref text, .. } = block {
                    if metadata.unit_title.is_none() {
                        metadata.unit_title = Some(text.clone());
                        document_title = text.clone();
                    }
                }
                blocks.push(block);
                idx = next_idx;
            } else {
                idx += 1;
            }
        }

        // Post-process to group question sections based on Heading 3 (###) Notion standard
        let structured_blocks = Self::group_questions(blocks, &mut metadata);

        Document {
            title: document_title,
            blocks: structured_blocks,
            metadata,
        }
    }

    fn parse_block(events: &[Event], start: usize, meta: &mut DocumentMetadata) -> Option<(Block, usize)> {
        if start >= events.len() {
            return None;
        }

        match &events[start] {
            Event::Start(Tag::Heading { level, .. }) => {
                let lvl = *level as u8;
                let mut inlines = Vec::new();
                let mut curr = start + 1;
                while curr < events.len() {
                    if matches!(&events[curr], Event::End(TagEnd::Heading(_))) {
                        curr += 1;
                        break;
                    }
                    if let Some((inline, next_c)) = Self::parse_inline(events, curr) {
                        inlines.push(inline);
                        curr = next_c;
                    } else {
                        curr += 1;
                    }
                }
                let text = Self::inlines_to_plain_text(&inlines);
                let id = text.to_lowercase().replace(|c: char| !c.is_alphanumeric(), "-");
                Some((Block::Heading { level: lvl, text, id }, curr))
            }
            Event::Start(Tag::Paragraph) => {
                let mut inlines = Vec::new();
                let mut curr = start + 1;
                while curr < events.len() {
                    if matches!(&events[curr], Event::End(TagEnd::Paragraph)) {
                        curr += 1;
                        break;
                    }
                    if let Event::Start(Tag::Image { dest_url, title, .. }) = &events[curr] {
                        let src = dest_url.to_string();
                        let title_str = if title.is_empty() { None } else { Some(title.to_string()) };
                        let mut alt_inlines = Vec::new();
                        let mut img_curr = curr + 1;
                        while img_curr < events.len() {
                            if matches!(&events[img_curr], Event::End(TagEnd::Image)) {
                                img_curr += 1;
                                break;
                            }
                            if let Some((inline, nxt)) = Self::parse_inline(events, img_curr) {
                                alt_inlines.push(inline);
                                img_curr = nxt;
                            } else {
                                img_curr += 1;
                            }
                        }
                        let alt = Self::inlines_to_plain_text(&alt_inlines);
                        meta.total_images += 1;
                        
                        if img_curr < events.len() && matches!(&events[img_curr], Event::End(TagEnd::Paragraph)) {
                            img_curr += 1;
                        }

                        return Some((
                            Block::Image {
                                src,
                                alt,
                                title: title_str,
                                resolved_path: None,
                                is_missing: false,
                            },
                            img_curr,
                        ));
                    }

                    if let Some((inline, next_c)) = Self::parse_inline(events, curr) {
                        inlines.push(inline);
                        curr = next_c;
                    } else {
                        curr += 1;
                    }
                }
                Some((Block::Paragraph { inlines }, curr))
            }
            Event::Start(Tag::List(first_opt)) => {
                let is_ordered = first_opt.is_some();
                let mut items = Vec::new();
                let mut curr = start + 1;

                while curr < events.len() {
                    if matches!(&events[curr], Event::End(TagEnd::List(_))) {
                        curr += 1;
                        break;
                    }
                    if matches!(&events[curr], Event::Start(Tag::Item)) {
                        curr += 1;
                        let mut item_blocks = Vec::new();
                        let mut pending_inlines = Vec::new();

                        while curr < events.len() {
                            if matches!(&events[curr], Event::End(TagEnd::Item)) {
                                curr += 1;
                                break;
                            }

                            if matches!(
                                &events[curr],
                                Event::Start(Tag::Paragraph)
                                    | Event::Start(Tag::List(_))
                                    | Event::Start(Tag::Table(_))
                                    | Event::Start(Tag::CodeBlock(_))
                                    | Event::Start(Tag::Heading { .. })
                            ) {
                                if !pending_inlines.is_empty() {
                                    item_blocks.push(Block::Paragraph {
                                        inlines: std::mem::take(&mut pending_inlines),
                                    });
                                }
                                if let Some((blk, next_c)) = Self::parse_block(events, curr, meta) {
                                    item_blocks.push(blk);
                                    curr = next_c;
                                } else {
                                    curr += 1;
                                }
                            } else if let Some((inline, next_c)) = Self::parse_inline(events, curr) {
                                pending_inlines.push(inline);
                                curr = next_c;
                            } else {
                                curr += 1;
                            }
                        }

                        if !pending_inlines.is_empty() {
                            item_blocks.push(Block::Paragraph {
                                inlines: pending_inlines,
                            });
                        }

                        items.push(item_blocks);
                    } else {
                        curr += 1;
                    }
                }
                Some((Block::List { ordered: is_ordered, items }, curr))
            }
            Event::Start(Tag::Table(alignments)) => {
                meta.total_tables += 1;
                let align_strs = alignments
                    .iter()
                    .map(|a| match a {
                        Alignment::Left => "left",
                        Alignment::Center => "center",
                        Alignment::Right => "right",
                        Alignment::None => "left",
                    }.to_string())
                    .collect();

                let mut headers = Vec::new();
                let mut rows = Vec::new();
                let mut curr = start + 1;

                while curr < events.len() {
                    if matches!(&events[curr], Event::End(TagEnd::Table)) {
                        curr += 1;
                        break;
                    }
                    if matches!(&events[curr], Event::Start(Tag::TableHead)) {
                        curr += 1;
                        while curr < events.len() {
                            if matches!(&events[curr], Event::End(TagEnd::TableHead)) {
                                curr += 1;
                                break;
                            }
                            if matches!(&events[curr], Event::Start(Tag::TableCell)) {
                                curr += 1;
                                let mut cell_inlines = Vec::new();
                                while curr < events.len() && !matches!(&events[curr], Event::End(TagEnd::TableCell)) {
                                    if let Some((inl, nxt)) = Self::parse_inline(events, curr) {
                                        cell_inlines.push(inl);
                                        curr = nxt;
                                    } else {
                                        curr += 1;
                                    }
                                }
                                if curr < events.len() { curr += 1; }
                                headers.push(cell_inlines);
                            } else {
                                curr += 1;
                            }
                        }
                    } else if matches!(&events[curr], Event::Start(Tag::TableRow)) {
                        curr += 1;
                        let mut row_cells = Vec::new();
                        while curr < events.len() {
                            if matches!(&events[curr], Event::End(TagEnd::TableRow)) {
                                curr += 1;
                                break;
                            }
                            if matches!(&events[curr], Event::Start(Tag::TableCell)) {
                                curr += 1;
                                let mut cell_inlines = Vec::new();
                                while curr < events.len() && !matches!(&events[curr], Event::End(TagEnd::TableCell)) {
                                    if let Some((inl, nxt)) = Self::parse_inline(events, curr) {
                                        cell_inlines.push(inl);
                                        curr = nxt;
                                    } else {
                                        curr += 1;
                                    }
                                }
                                if curr < events.len() { curr += 1; }
                                row_cells.push(cell_inlines);
                            } else {
                                curr += 1;
                            }
                        }
                        rows.push(row_cells);
                    } else {
                        curr += 1;
                    }
                }

                Some((
                    Block::Table {
                        headers,
                        rows,
                        alignments: align_strs,
                    },
                    curr,
                ))
            }
            Event::Start(Tag::CodeBlock(kind)) => {
                let lang = match kind {
                    pulldown_cmark::CodeBlockKind::Fenced(lang) => {
                        let l = lang.trim();
                        if l.is_empty() { None } else { Some(l.to_string()) }
                    }
                    pulldown_cmark::CodeBlockKind::Indented => None,
                };
                let mut code = String::new();
                let mut curr = start + 1;
                while curr < events.len() {
                    if matches!(&events[curr], Event::End(TagEnd::CodeBlock)) {
                        curr += 1;
                        break;
                    }
                    if let Event::Text(txt) = &events[curr] {
                        code.push_str(txt);
                    }
                    curr += 1;
                }
                Some((Block::CodeBlock { language: lang, code }, curr))
            }
            Event::Rule => Some((Block::HorizontalRule, start + 1)),
            _ => None,
        }
    }

    fn parse_inline(events: &[Event], start: usize) -> Option<(Inline, usize)> {
        if start >= events.len() {
            return None;
        }

        match &events[start] {
            Event::Text(txt) => Some((Inline::Text { text: txt.to_string() }, start + 1)),
            Event::Code(code) => Some((Inline::Code { code: code.to_string() }, start + 1)),
            Event::SoftBreak | Event::HardBreak => Some((Inline::LineBreak, start + 1)),
            Event::Start(Tag::Emphasis) => {
                let mut content = Vec::new();
                let mut curr = start + 1;
                while curr < events.len() {
                    if matches!(&events[curr], Event::End(TagEnd::Emphasis)) {
                        curr += 1;
                        break;
                    }
                    if let Some((inl, nxt)) = Self::parse_inline(events, curr) {
                        content.push(inl);
                        curr = nxt;
                    } else {
                        curr += 1;
                    }
                }
                Some((Inline::Italic { content }, curr))
            }
            Event::Start(Tag::Strong) => {
                let mut content = Vec::new();
                let mut curr = start + 1;
                while curr < events.len() {
                    if matches!(&events[curr], Event::End(TagEnd::Strong)) {
                        curr += 1;
                        break;
                    }
                    if let Some((inl, nxt)) = Self::parse_inline(events, curr) {
                        content.push(inl);
                        curr = nxt;
                    } else {
                        curr += 1;
                    }
                }
                Some((Inline::Bold { content }, curr))
            }
            Event::Start(Tag::Link { dest_url, .. }) => {
                let url = dest_url.to_string();
                let mut text_inlines = Vec::new();
                let mut curr = start + 1;
                while curr < events.len() {
                    if matches!(&events[curr], Event::End(TagEnd::Link)) {
                        curr += 1;
                        break;
                    }
                    if let Some((inl, nxt)) = Self::parse_inline(events, curr) {
                        text_inlines.push(inl);
                        curr = nxt;
                    } else {
                        curr += 1;
                    }
                }
                let text = Self::inlines_to_plain_text(&text_inlines);
                Some((Inline::Link { text, url }, curr))
            }
            _ => None,
        }
    }

    fn inlines_to_plain_text(inlines: &[Inline]) -> String {
        let mut out = String::new();
        for inl in inlines {
            match inl {
                Inline::Text { text } => out.push_str(text),
                Inline::Bold { content } => out.push_str(&Self::inlines_to_plain_text(content)),
                Inline::Italic { content } => out.push_str(&Self::inlines_to_plain_text(content)),
                Inline::Code { code } => out.push_str(code),
                Inline::Link { text, .. } => out.push_str(text),
                Inline::Image { alt, .. } => out.push_str(alt),
                Inline::LineBreak => out.push(' '),
            }
        }
        out
    }

    fn group_questions(blocks: Vec<Block>, meta: &mut DocumentMetadata) -> Vec<Block> {
        let mut result = Vec::new();
        let mut current_question: Option<(String, Vec<Block>)> = None;

        for block in blocks {
            // Strictly classify Heading 3 (###) as the Question Level per Notion export convention
            let is_main_q = if let Block::Heading { level, .. } = &block {
                *level == 3
            } else {
                false
            };

            if is_main_q {
                if let Some((q_title, q_blocks)) = current_question.take() {
                    meta.total_questions += 1;
                    result.push(Block::QuestionGroup { title: q_title, blocks: q_blocks });
                }
                if let Block::Heading { text, .. } = &block {
                    current_question = Some((text.clone(), vec![block]));
                }
            } else if let Some((_, ref mut q_blocks)) = current_question {
                q_blocks.push(block);
            } else {
                result.push(block);
            }
        }

        if let Some((q_title, q_blocks)) = current_question {
            meta.total_questions += 1;
            result.push(Block::QuestionGroup { title: q_title, blocks: q_blocks });
        }

        result
    }
}
