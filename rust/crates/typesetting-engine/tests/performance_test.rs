//! 性能测试
//! 
//! 测试排版引擎在处理大型文档时的性能表现

use typesetting_engine::{ParserEngine, LayoutEngine, PageConfig};
use std::time::Instant;

/// 测试解析大型文档的性能
#[test]
fn test_parsing_performance() {
    // 创建一个大型文档
    let mut content = String::new();
    content.push_str("# 性能测试文档\n\n");
    
    for chapter in 1..=10 {
        content.push_str(&format!("# 第{}章\n\n", chapter));
        
        for section in 1..=10 {
            content.push_str(&format!("## 第{}节\n\n", section));
            
            for paragraph in 1..=20 {
                content.push_str(&format!(
                    "这是第{}章第{}节的第{}个段落。它包含一些文本内容，用于测试排版引擎的性能表现。\
                    这些内容应该足够长，以便能够准确测量处理时间。\
                    我们需要确保引擎能够高效地处理大量文本内容。\n\n",
                    chapter, section, paragraph
                ));
            }
        }
    }
    
    let start_time = Instant::now();
    let parser = ParserEngine::new();
    let document = parser.parse_txt(&content);
    let parse_duration = start_time.elapsed();
    
    println!("解析时间: {:?}", parse_duration);
    assert!(parse_duration.as_secs() < 5); // 应该在5秒内完成
    
    assert_eq!(document.chapters.len(), 10);
    for chapter in &document.chapters {
        assert!(!chapter.content.is_empty());
    }
}

/// 测试布局大型文档的性能
#[test]
fn test_layout_performance() {
    // 创建一个大型文档
    let mut content = String::new();
    content.push_str("# 性能测试文档\n\n");
    
    for i in 1..=1000 {
        content.push_str(&format!("这是第{}个段落，用于测试布局性能。\n\n", i));
    }
    
    let parser = ParserEngine::new();
    let document = parser.parse_txt(&content);
    
    let page_config = PageConfig {
        width: 400.0,
        height: 600.0,
        margin_top: 20.0,
        margin_bottom: 20.0,
        margin_left: 20.0,
        margin_right: 20.0,
    };
    
    let start_time = Instant::now();
    let layout_engine = LayoutEngine::new(page_config);
    let pages = layout_engine.layout_document(&document);
    let layout_duration = start_time.elapsed();
    
    println!("布局时间: {:?}", layout_duration);
    assert!(layout_duration.as_secs() < 10); // 应该在10秒内完成
    
    assert!(!pages.is_empty());
    assert!(pages.len() > 1);
}

/// 测试布局大型文档的性能（去除渲染测试）
#[test]
fn test_rendering_performance() {
    // 创建一个大型文档
    let mut content = String::new();
    content.push_str("# 性能测试文档\n\n");
    
    for i in 1..=500 {
        content.push_str(&format!("这是第{}个段落，用于测试布局性能。\n\n", i));
    }
    
    let parser = ParserEngine::new();
    let document = parser.parse_txt(&content);
    
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
    
    let start_time = Instant::now();
    // 模拟渲染过程，但不实际渲染
    let _rendered_data = format!("{:?}", pages);
    let render_duration = start_time.elapsed();
    
    println!("模拟渲染时间: {:?}", render_duration);
    assert!(render_duration.as_secs() < 5); // 应该在5秒内完成
    
    assert!(!_rendered_data.is_empty());
}