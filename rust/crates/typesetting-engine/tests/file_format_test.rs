//! 文件格式检测测试

use typesetting_engine::FileLoader;
use typesetting_engine::FileFormat;

#[test]
fn test_detect_txt_format() {
    assert_eq!(FileLoader::detect_format("test.txt"), FileFormat::Txt);
    assert_eq!(FileLoader::detect_format("test.TXT"), FileFormat::Txt);
    assert_eq!(FileLoader::detect_format("path/to/document.txt"), FileFormat::Txt);
}

#[test]
fn test_detect_other_format() {
    assert_eq!(FileLoader::detect_format("test.epub"), FileFormat::Other("epub".to_string()));
    assert_eq!(FileLoader::detect_format("test.pdf"), FileFormat::Other("pdf".to_string()));
}

#[test]
fn test_detect_no_extension() {
    assert_eq!(FileLoader::detect_format("test"), FileFormat::Txt);
    assert_eq!(FileLoader::detect_format("path/to/test"), FileFormat::Txt);
}