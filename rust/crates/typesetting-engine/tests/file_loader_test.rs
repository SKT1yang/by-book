//! 文件加载器测试

use typesetting_engine::FileLoader;

use std::fs;
use std::io::Write;
use tempfile::TempDir;

#[test]
fn test_load_utf8_file() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test_utf8.txt");
    
    // 创建UTF-8编码的测试文件
    let content = "这是一个UTF-8编码的测试文件\n包含中文内容";
    fs::write(&file_path, content).unwrap();
    
    // 测试文件加载
    let loader = FileLoader::new();
    let loaded_content = loader.load_text_file(file_path.to_str().unwrap()).unwrap();
    
    assert_eq!(loaded_content, content);
}

#[test]
fn test_load_latin1_file() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test_latin1.txt");
    
    // 创建包含Latin-1字符的测试文件（模拟非UTF-8文件）
    let content = vec![0xC4, 0xE9, 0xF3, 0xF1]; // 一些Latin-1字符
    let mut file = fs::File::create(&file_path).unwrap();
    file.write_all(&content).unwrap();
    
    // 测试文件加载（应该能够加载，即使不是完美的UTF-8）
    let loader = FileLoader::new();
    let loaded_content = loader.load_text_file(file_path.to_str().unwrap()).unwrap();
    
    // 内容应该被加载，即使可能不是原始文本
    assert!(!loaded_content.is_empty());
}

#[test]
fn test_load_and_parse_document() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test_document.txt");
    
    // 创建测试文档
    let content = "# 第一章 测试章节\n\n这是一个测试段落。\n\n第二段内容。";
    fs::write(&file_path, content).unwrap();
    
    // 测试加载并解析文档
    let loader = FileLoader::new();
    let document = loader.load_and_parse_document(file_path.to_str().unwrap()).unwrap();
    
    // 验证文档结构
    assert!(!document.chapters.is_empty());
    assert_eq!(document.chapters[0].title, "Chapter 1");
}

#[test]
fn test_file_not_found() {
    let loader = FileLoader::new();
    let result = loader.load_text_file("non_existent_file.txt");
    
    // 应该返回错误
    assert!(result.is_err());
}