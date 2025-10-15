use std::{fs::File, io::Read, path::Path};
use zip::ZipArchive;
use nom::{Parser, Err};

pub struct EpubContainer {
    archive: ZipArchive<std::fs::File>,
}

impl EpubContainer {
    pub fn open(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let archive = ZipArchive::new(file)?;
        Ok(Self { archive })
    }

    pub fn read_container_xml(&mut self) -> Result<String, std::io::Error> {
        let mut file = self.archive.by_name("META-INF/container.xml")?; 
        let mut xml = String::new();
        file.read_to_string(&mut  xml)?;
        Ok(xml)
    }

    // 使用nom解析容器XML获取rootfile路径
    fn parse_container(xml: &str) -> Result<(&str, &str), Err<nom::error::Error<&str>>> {
        let (_, path) = nom::bytes::complete::tag("<rootfile full-path=\"")
            .and(nom::bytes::complete::take_until("\""))
            .parse(xml)?;
        Ok(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::fs::File;
    use std::path::Path;
    use tempfile::tempdir;
    use zip::write::FileOptions;

    // Helper: create a minimal epub with META-INF/container.xml
    fn create_epub_with_container_xml(path: &Path, container_xml: &str) {
        let file = File::create(path).unwrap();
        let mut zip = zip::ZipWriter::new(file);
        let options = FileOptions::<()>::default();
        zip.start_file("META-INF/container.xml", options).unwrap();
        zip.write_all(container_xml.as_bytes()).unwrap();
        zip.finish().unwrap();
    }

    #[test]
    fn test_open_valid_epub() {
        let dir = tempdir().unwrap();
        let epub_path = dir.path().join("test.epub");
        let container_xml: &'static str = r#"<rootfile full-path="OEBPS/content.opf" media-type="application/oebps-package+xml"/>"#;
        create_epub_with_container_xml(&epub_path, container_xml);

        let result = EpubContainer::open(&epub_path);
        assert!(result.is_ok());
    }

    #[test]
    fn test_open_invalid_epub_path() {
        let result = EpubContainer::open(Path::new("nonexistent.epub"));
        assert!(result.is_err());
    }

    #[test]
    fn test_read_container_xml_success() {
        let dir = tempdir().unwrap();
        let epub_path = dir.path().join("test.epub");
        let container_xml = r#"<rootfile full-path="OEBPS/content.opf" media-type="application/oebps-package+xml"/>"#;
        create_epub_with_container_xml(&epub_path, container_xml);

        let mut epub = EpubContainer::open(&epub_path).unwrap();
        let xml = epub.read_container_xml().unwrap();
        assert!(xml.contains("full-path=\"OEBPS/content.opf\""));
    }

    #[test]
    fn test_read_container_xml_missing_file() {
        let dir = tempdir().unwrap();
        let epub_path = dir.path().join("test.epub");
        // Create empty zip (no container.xml)
        let file = File::create(&epub_path).unwrap();
        let zip = zip::ZipWriter::new(file);
        zip.finish().unwrap();

        let mut epub = EpubContainer::open(&epub_path).unwrap();
        let result = epub.read_container_xml();
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_container_success() {
        let xml = r#"<rootfile full-path="OEBPS/content.opf" media-type="application/oebps-package+xml"/>"#;
        let result = EpubContainer::parse_container(xml);
        assert!(result.is_ok());
        let (path, _) = result.unwrap();
        assert_eq!(path, "OEBPS/content.opf");
    }

    #[test]
    fn test_parse_container_fail() {
        let xml = r#"<notrootfile something="else"/>"#;
        let result = EpubContainer::parse_container(xml);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_container_with_additional_content() {
        let xml = r#"some text <rootfile full-path="OPS/book.opf" media-type="application/oebps-package+xml"/>"#;
        let result = EpubContainer::parse_container(xml);
        assert!(result.is_ok());
        let (path, _) = result.unwrap();
        assert_eq!(path, "OPS/book.opf");
    }

    #[test]
    fn test_parse_container_with_multiple_rootfiles() {
        let xml = r#"
            <rootfile full-path="OPS/book1.opf" media-type="application/oebps-package+xml"/>
            <rootfile full-path="OPS/book2.opf" media-type="application/oebps-package+xml"/>
        "#;
        let result = EpubContainer::parse_container(xml);
        assert!(result.is_ok());
        let (path, _) = result.unwrap();
        assert_eq!(path, "OPS/book1.opf");
    }

    #[test]
    fn test_read_container_xml_invalid_utf8() {
        let dir = tempdir().unwrap();
        let epub_path = dir.path().join("test.epub");
        // Write invalid UTF-8 bytes to container.xml
        let file = File::create(&epub_path).unwrap();
        let mut zip = zip::ZipWriter::new(file);
        let options = FileOptions::<()>::default();
        zip.start_file("META-INF/container.xml", options).unwrap();
        zip.write_all(&[0xff, 0xfe, 0xfd]).unwrap();
        zip.finish().unwrap();

        let mut epub = EpubContainer::open(&epub_path).unwrap();
        let result = epub.read_container_xml();
        assert!(result.is_err());
    }
}
