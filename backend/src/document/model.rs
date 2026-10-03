use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub title: String,
    pub blocks: Vec<Block>,
    pub metadata: DocumentMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DocumentMetadata {
    pub unit_title: Option<String>,
    pub total_questions: usize,
    pub total_images: usize,
    pub total_tables: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Block {
    Heading {
        level: u8,
        text: String,
        id: String,
    },
    Paragraph {
        inlines: Vec<Inline>,
    },
    List {
        ordered: bool,
        items: Vec<Vec<Block>>,
    },
    Table {
        headers: Vec<Vec<Inline>>,
        rows: Vec<Vec<Vec<Inline>>>,
        alignments: Vec<String>,
    },
    Image {
        src: String,
        alt: String,
        title: Option<String>,
        resolved_path: Option<String>,
        is_missing: bool,
    },
    QuestionGroup {
        title: String,
        blocks: Vec<Block>,
    },
    CodeBlock {
        language: Option<String>,
        code: String,
    },
    HorizontalRule,
    Blockquote {
        blocks: Vec<Block>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum Inline {
    Text { text: String },
    Bold { content: Vec<Inline> },
    Italic { content: Vec<Inline> },
    Code { code: String },
    Link { text: String, url: String },
    Image { src: String, alt: String },
    LineBreak,
}
