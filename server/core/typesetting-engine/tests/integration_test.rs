//! 集成测试
//! 
//! 测试排版引擎完整工作流程的正确性

use typesetting_engine::{ParserEngine, LayoutEngine, Renderer, PageConfig};

/// 测试完整处理流程
/// 
/// 验证从解析到布局再到渲染的整个流程是否正常工作
#[test]
fn test_full_pipeline() {
    let content = "# Chapter 1\n\nThis is a test paragraph.\n\nThis is another paragraph.";
    
    // Parse
    let parser = ParserEngine::new();
    let document = parser.parse_txt(content);
    
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
    
    // Render
    let renderer = Renderer::new();
    let rendered = renderer.render_pages(&pages);
    
    // 注意：根据当前解析器实现，章节标题不会被直接渲染
    assert!(rendered.contains("This is a test paragraph."));
    assert!(rendered.contains("--- Page Start ---"));
    assert!(rendered.contains("--- Page End ---"));
    assert!(pages.len() >= 1);
}