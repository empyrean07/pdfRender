#[cfg(test)]
mod tests {
    use backend::archive::extractor::Extractor;
    use backend::assets::resolver::AssetResolver;
    use backend::markdown::parser::MarkdownParser;
    use backend::renderer::html::HtmlRenderer;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;
    use zip::write::FileOptions;

    #[test]
    fn test_markdown_parser_and_list_items() {
        let md = r#"
# UNIT TEST DOCUMENT

## 2022-Apr-May

### 1. Explain Bus Topology.

In a bus topology, all devices connect to a central backbone.

#### Advantages:
- Easy to connect devices.
- Requires less cable length.

#### Disadvantages:
- Entire network fails if backbone breaks.
"#;

        let doc = MarkdownParser::parse(md);
        assert_eq!(doc.title, "UNIT TEST DOCUMENT");
        assert_eq!(doc.metadata.total_questions, 1);

        let html = HtmlRenderer::render_document(&doc);
        assert!(html.contains("UNIT TEST DOCUMENT"));
        assert!(html.contains("Easy to connect devices."));
        assert!(html.contains("Requires less cable length."));
        assert!(html.contains("Entire network fails if backbone breaks."));
        assert!(html.contains("class=\"question-title\""));
    }

    #[test]
    fn test_zip_path_traversal_protection() {
        let tmp = tempdir().unwrap();
        let zip_path = tmp.path().join("malicious.zip");
        let extract_dir = tmp.path().join("extracted");

        let file = File::create(&zip_path).unwrap();
        let mut zip = zip::ZipWriter::new(file);

        let options: FileOptions<'_, ()> = FileOptions::default().compression_method(zip::CompressionMethod::Stored);
        let _ = zip.start_file("../outside.txt", options);
        let _ = zip.write_all(b"malicious payload");
        zip.finish().unwrap();

        let result = Extractor::extract_zip(&zip_path, &extract_dir, 10 * 1024 * 1024, 100);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("Zip Slip") || err.contains("Path traversal"));
    }

    #[test]
    fn test_asset_resolver_broken_references() {
        let tmp = tempdir().unwrap();
        let md_content = r#"
# Document Title

![Missing Image](images/missing.png)
"#;
        let mut doc = MarkdownParser::parse(md_content);
        let report = AssetResolver::resolve_and_embed(&mut doc, tmp.path(), std::path::Path::new("doc.md"));

        assert_eq!(report.status, "BROKEN_REFERENCES");
        assert_eq!(report.broken_assets.len(), 1);
        assert_eq!(report.broken_assets[0].reference, "images/missing.png");
    }
}
