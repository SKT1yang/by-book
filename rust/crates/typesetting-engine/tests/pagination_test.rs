//! 分页测试
//! 
//! 测试大文档的分页功能

use typesetting_engine::{ParserEngine, LayoutEngine, Renderer, PageConfig};

/// 测试大文档的分页功能
/// 
/// 验证引擎能否正确处理大文档并将其分布到多个页面上
#[test]
fn test_large_document_pagination() {
    // 创建一个大文档来测试分页
    let mut content = String::new();
    content.push_str("# 第一章 大文档测试\n\n");
    
    // 添加大量段落以确保分页
    for i in 1..50 {
        content.push_str(&format!("这是第{}个段落。它包含一些文本内容，用来测试分页功能。\n\n", i));
    }
    
    content.push_str("# 第二章 更多内容\n\n");
    
    for i in 50..100 {
        content.push_str(&format!("这是第{}个段落。继续添加内容以测试分页。\n\n", i));
    }
    
    // Parse
    let parser = ParserEngine::new();
    let document = parser.parse_txt(&content);
    
    // Layout
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
    
    // 验证分页是否正确工作
    assert!(pages.len() > 1, "大文档应该被分到多个页面");
    assert!(pages.len() < 100, "页面数应该远小于段落数");
    
    // 验证每个页面都有内容
    for (i, page) in pages.iter().enumerate() {
        assert!(!page.blocks.is_empty(), "第{}页不应该为空", i + 1);
    }
    
    // Render
    let renderer = Renderer::new();
    let rendered = renderer.render_pages(&pages);
    
    assert!(rendered.contains("--- Page Start ---"));
    assert!(rendered.contains("--- Page End ---"));
}