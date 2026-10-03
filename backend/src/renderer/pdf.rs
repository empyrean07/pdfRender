use std::path::{Path, PathBuf};
use std::process::Stdio;
use tokio::process::Command;
use tracing::{error, info};

pub struct PdfRenderer;

impl PdfRenderer {
    pub async fn render_html_to_pdf(html_path: &Path, output_pdf_path: &Path) -> Result<(), String> {
        let script_path = if Path::new("renderer-service/render.js").exists() {
            PathBuf::from("renderer-service/render.js")
        } else if Path::new("../renderer-service/render.js").exists() {
            PathBuf::from("../renderer-service/render.js")
        } else {
            return Err("Renderer script (render.js) not found in renderer-service folder".to_string());
        };

        info!(
            "Orchestrating Chromium PDF rendering: {:?} -> {:?}",
            html_path, output_pdf_path
        );

        let child = Command::new("node")
            .arg(&script_path)
            .arg(html_path)
            .arg(output_pdf_path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to spawn node renderer process: {}", e))?;

        let output = child
            .wait_with_output()
            .await
            .map_err(|e| format!("Error waiting for PDF renderer process: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);
            error!("PDF Renderer process failed. Stderr: {}, Stdout: {}", stderr, stdout);
            return Err(format!("PDF rendering failed: {}", stderr));
        }

        if !output_pdf_path.exists() {
            return Err("PDF renderer completed but generated PDF file is missing".to_string());
        }

        let metadata = std::fs::metadata(output_pdf_path)
            .map_err(|e| format!("Failed to inspect output PDF metadata: {}", e))?;

        if metadata.len() == 0 {
            return Err("Generated PDF file is empty (0 bytes)".to_string());
        }

        info!("Successfully generated PDF ({} bytes) at {:?}", metadata.len(), output_pdf_path);
        Ok(())
    }
}
