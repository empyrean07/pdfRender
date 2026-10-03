use std::fs::{self, File};
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use tracing::{info, warn};
use walkdir::WalkDir;
use zip::ZipArchive;

#[derive(Debug)]
pub struct ExtractionResult {
    pub extracted_path: PathBuf,
    pub markdown_files: Vec<PathBuf>,
    pub primary_markdown: Option<PathBuf>,
    pub total_files: usize,
    pub total_bytes: u64,
}

pub struct Extractor;

impl Extractor {
    pub fn extract_zip(
        zip_path: &Path,
        dest_dir: &Path,
        max_bytes: u64,
        max_files: usize,
    ) -> Result<ExtractionResult, String> {
        let file = File::open(zip_path).map_err(|e| format!("Failed to open ZIP file: {}", e))?;
        let mut archive = ZipArchive::new(file).map_err(|e| format!("Invalid ZIP archive: {}", e))?;

        fs::create_dir_all(dest_dir)
            .map_err(|e| format!("Failed to create destination directory: {}", e))?;

        let mut total_files = 0;
        let mut total_bytes = 0u64;

        for i in 0..archive.len() {
            let mut zip_file = archive
                .by_index(i)
                .map_err(|e| format!("Error reading ZIP index {}: {}", i, e))?;
            
            let name_str = zip_file.name();

            // Strict Zip Slip & path traversal check
            if zip_file.enclosed_name().is_none() || name_str.contains("..") || name_str.starts_with('/') || name_str.starts_with('\\') {
                warn!("Blocked Zip Slip path traversal attempt: {}", name_str);
                return Err(format!("Security alert: Zip Slip path traversal attempt detected in entry: {}", name_str));
            }

            let enclosed_name = zip_file.enclosed_name().unwrap();

            let out_path = dest_dir.join(&enclosed_name);

            if zip_file.name().ends_with('/') || zip_file.is_dir() {
                fs::create_dir_all(&out_path)
                    .map_err(|e| format!("Failed to create extracted directory {:?}: {}", out_path, e))?;
            } else {
                if let Some(parent) = out_path.parent() {
                    if !parent.exists() {
                        fs::create_dir_all(parent)
                            .map_err(|e| format!("Failed to create parent directory {:?}: {}", parent, e))?;
                    }
                }

                total_files += 1;
                if total_files > max_files {
                    return Err(format!("Exceeded maximum file count limit ({})", max_files));
                }

                let mut outfile = File::create(&out_path)
                    .map_err(|e| format!("Failed to create output file {:?}: {}", out_path, e))?;

                let mut buffer = [0u8; 8192];
                loop {
                    let n = zip_file
                        .read(&mut buffer)
                        .map_err(|e| format!("Error reading ZIP stream: {}", e))?;
                    if n == 0 {
                        break;
                    }
                    total_bytes += n as u64;
                    if total_bytes > max_bytes {
                        return Err(format!("Exceeded maximum extracted size limit ({} bytes)", max_bytes));
                    }
                    io::Write::write_all(&mut outfile, &buffer[..n])
                        .map_err(|e| format!("Failed to write extracted file: {}", e))?;
                }
            }
        }

        info!("Extracted {} files ({} bytes) to {:?}", total_files, total_bytes, dest_dir);

        // Discover Markdown files
        let mut markdown_files = Vec::new();
        for entry in WalkDir::new(dest_dir).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                if let Some(ext) = entry.path().extension() {
                    if ext.to_string_lossy().to_lowercase() == "md" {
                        if let Ok(rel) = entry.path().strip_prefix(dest_dir) {
                            markdown_files.push(rel.to_path_buf());
                        }
                    }
                }
            }
        }

        markdown_files.sort();

        let primary_markdown = if markdown_files.len() == 1 {
            Some(markdown_files[0].clone())
        } else if markdown_files.len() > 1 {
            let primary = markdown_files.iter().find(|p| {
                let stem = p.file_stem().unwrap_or_default().to_string_lossy().to_lowercase();
                stem == "readme" || stem == "main" || stem == "index" || stem.contains("unit")
            }).cloned().unwrap_or_else(|| markdown_files[0].clone());
            Some(primary)
        } else {
            None
        };

        Ok(ExtractionResult {
            extracted_path: dest_dir.to_path_buf(),
            markdown_files,
            primary_markdown,
            total_files,
            total_bytes,
        })
    }
}
