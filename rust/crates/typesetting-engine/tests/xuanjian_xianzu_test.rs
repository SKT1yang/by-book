//! 《玄鉴仙族》测试
//! 
//! 测试对实际小说文件的处理能力

use typesetting_engine::{ParserEngine, LayoutEngine, PageConfig};
use std::fs;

/// 测试《玄鉴仙族》文件处理
#[test]
fn test_xuanjian_xianzu_processing() {
    // 读取《玄鉴仙族》文件
    let content = fs::read_to_string("《玄鉴仙族》.txt").expect("无法读取《玄鉴仙族》文件");
    
    // 只测试前10000个字符，避免测试时间过长
    let content = if content.len() > 10000 {
        // 确保我们在字符边界上截断
        let mut end = 10000;
        while end > 0 && !content.is_char_boundary(end) {
            end -= 1;
        }
        &content[..end]
    } else {
        &content
    };
    
    let parser = ParserEngine::new();
    let document = parser.parse_txt(content);
    
    // 应该识别出至少一个章节
    assert!(document.chapters.len() > 0);
    
    // 验证章节内容不为空
    for (i, chapter) in document.chapters.iter().enumerate() {
        assert!(!chapter.content.is_empty(), "第{}章内容不应该为空", i + 1);
    }
    
    // 测试布局引擎
    let page_config = PageConfig {
        width: 400.0,
        height: 600.0,
        margin_top: 20.0,
        margin_bottom: 20.0,
        margin_left: 20.0,
        margin_right: 20.0,
    };
    
    let layout_engine = LayoutEngine::new(page_config);
    let pages = layout_engine.layout_document(&document);
    
    // 应该生成页面
    assert!(!pages.is_empty());
    
    // 验证页面内容
    for (i, page) in pages.iter().enumerate() {
        assert!(!page.blocks.is_empty(), "第{}页不应该为空", i + 1);
    }
}

/// 测试大文件分页
#[test]
fn test_large_file_pagination() {
    // 读取《玄鉴仙族》文件
    let content = fs::read_to_string("《玄鉴仙族》.txt").expect("无法读取《玄鉴仙族》文件");
    
    // 只测试前50000个字符，避免测试时间过长
    let content = if content.len() > 50000 {
        // 确保我们在字符边界上截断
        let mut end = 50000;
        while end > 0 && !content.is_char_boundary(end) {
            end -= 1;
        }
        &content[..end]
    } else {
        &content
    };
    
    let parser = ParserEngine::new();
    let document = parser.parse_txt(content);
    
    let page_config = PageConfig {
        width: 400.0,
        height: 600.0,
        margin_top: 20.0,
        margin_bottom: 20.0,
        margin_left: 20.0,
        margin_right: 20.0,
    };
    
    let layout_engine = LayoutEngine::new(page_config);
    let pages = layout_engine.layout_document(&document);
    
    // 应该生成多个页面
    assert!(pages.len() > 5); // 至少应该有5页
    
    // 验证页面内容
    for (i, page) in pages.iter().enumerate() {
        assert!(!page.blocks.is_empty(), "第{}页不应该为空", i + 1);
    }
}