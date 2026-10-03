use crate::document::model::*;
use base64::Engine;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use tracing::{info, warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrokenAsset {
    pub markdown_file: String,
    pub reference: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetValidationReport {
    pub total_references: usize,
    pub resolved_references: usize,
    pub broken_assets: Vec<BrokenAsset>,
    pub status: String,
}

pub struct AssetResolver;

impl AssetResolver {
    pub fn resolve_and_embed(
        doc: &mut Document,
        extracted_root: &Path,
        primary_md_rel_path: &Path,
    ) -> AssetValidationReport {
        let md_dir = primary_md_rel_path
            .parent()
            .map(|p| extracted_root.join(p))
            .unwrap_or_else(|| extracted_root.to_path_buf());

        let md_filename = primary_md_rel_path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        let mut total_refs = 0;
        let mut resolved_refs = 0;
        let mut broken_assets = Vec::new();

        Self::process_blocks(
            &mut doc.blocks,
            extracted_root,
            &md_dir,
            &md_filename,
            &mut total_refs,
            &mut resolved_refs,
            &mut broken_assets,
        );

        let status = if broken_assets.is_empty() {
            "READY".to_string()
        } else {
            "BROKEN_REFERENCES".to_string()
        };

        info!(
            "Asset resolution completed: {} total, {} resolved, {} broken",
            total_refs, resolved_refs, broken_assets.len()
        );

        AssetValidationReport {
            total_references: total_refs,
            resolved_references: resolved_refs,
            broken_assets,
            status,
        }
    }

    fn process_blocks(
        blocks: &mut [Block],
        extracted_root: &Path,
        md_dir: &Path,
        md_filename: &str,
        total_refs: &mut usize,
        resolved_refs: &mut usize,
        broken_assets: &mut Vec<BrokenAsset>,
    ) {
        for block in blocks {
            match block {
                Block::Image { src, resolved_path, is_missing, .. } => {
                    *total_refs += 1;
                    let (data_uri, found) = Self::resolve_single_image(src, extracted_root, md_dir);
                    if found {
                        *resolved_refs += 1;
                        *resolved_path = Some(data_uri);
                        *is_missing = false;
                    } else {
                        *is_missing = true;
                        broken_assets.push(BrokenAsset {
                            markdown_file: md_filename.to_string(),
                            reference: src.clone(),
                            status: "NOT FOUND".to_string(),
                        });
                    }
                }
                Block::QuestionGroup { blocks: q_blocks, .. } => {
                    Self::process_blocks(
                        q_blocks,
                        extracted_root,
                        md_dir,
                        md_filename,
                        total_refs,
                        resolved_refs,
                        broken_assets,
                    );
                }
                Block::List { items, .. } => {
                    for item_blocks in items {
                        Self::process_blocks(
                            item_blocks,
                            extracted_root,
                            md_dir,
                            md_filename,
                            total_refs,
                            resolved_refs,
                            broken_assets,
                        );
                    }
                }
                Block::Paragraph { inlines } => {
                    Self::process_inlines(
                        inlines,
                        extracted_root,
                        md_dir,
                        md_filename,
                        total_refs,
                        resolved_refs,
                        broken_assets,
                    );
                }
                _ => {}
            }
        }
    }

    fn process_inlines(
        inlines: &mut [Inline],
        extracted_root: &Path,
        md_dir: &Path,
        md_filename: &str,
        total_refs: &mut usize,
        resolved_refs: &mut usize,
        broken_assets: &mut Vec<BrokenAsset>,
    ) {
        for inline in inlines {
            match inline {
                Inline::Image { src, .. } => {
                    *total_refs += 1;
                    let (data_uri, found) = Self::resolve_single_image(src, extracted_root, md_dir);
                    if found {
                        *resolved_refs += 1;
                        *src = data_uri;
                    } else {
                        broken_assets.push(BrokenAsset {
                            markdown_file: md_filename.to_string(),
                            reference: src.clone(),
                            status: "NOT FOUND".to_string(),
                        });
                    }
                }
                Inline::Bold { content } | Inline::Italic { content } => {
                    Self::process_inlines(
                        content,
                        extracted_root,
                        md_dir,
                        md_filename,
                        total_refs,
                        resolved_refs,
                        broken_assets,
                    );
                }
                _ => {}
            }
        }
    }

    fn resolve_single_image(src: &str, extracted_root: &Path, md_dir: &Path) -> (String, bool) {
        if src.starts_with("http://") || src.starts_with("https://") || src.starts_with("data:") {
            return (src.to_string(), true);
        }

        let decoded_ref = urlencoding::decode(src).unwrap_or(std::borrow::Cow::Borrowed(src));
        let ref_path = Path::new(decoded_ref.as_ref());

        let candidate_path = if ref_path.is_absolute() {
            ref_path.to_path_buf()
        } else {
            md_dir.join(ref_path)
        };

        let target_path = match candidate_path.canonicalize() {
            Ok(p) => p,
            Err(_) => {
                let alt_path = extracted_root.join(ref_path);
                match alt_path.canonicalize() {
                    Ok(p) => p,
                    Err(_) => candidate_path,
                }
            }
        };

        if target_path.exists() && target_path.is_file() {
            if let Ok(bytes) = fs::read(&target_path) {
                let mime_type = mime_guess::from_path(&target_path)
                    .first_or_octet_stream()
                    .to_string();
                let b64 = base64::engine::general_purpose::STANDARD.encode(&bytes);
                let data_url = format!("data:{};base64,{}", mime_type, b64);
                return (data_url, true);
            }
        }

        warn!("Asset reference not found: {:?} (attempted: {:?})", src, target_path);
        (src.to_string(), false)
    }
}
