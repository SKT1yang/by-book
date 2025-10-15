//! 边界情况测试
//! 
//! 测试各种边界情况和异常情况的处理

use typesetting_engine::{ParserEngine, LayoutEngine, PageConfig};

/// 测试非常大的文档
#[test]
fn test_very_large_document() {
    let mut content = String::new();
    content.push_str("# 大文档测试\n\n");
    
    // 创建一个非常大的文档
    for i in 0..1000 {
        content.push_str(&format!("这是第{}个段落，用来测试非常大的文档处理能力。\n\n", i));
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
    
    // 应该生成多个页面
    assert!(pages.len() > 1);
    
    // 每个页面都应该有内容
    for (i, page) in pages.iter().enumerate() {
        assert!(!page.blocks.is_empty(), "第{}页不应该为空", i + 1);
    }
}

/// 测试包含特殊字符的文档
#[test]
fn test_document_with_special_characters() {
    let content = "# 特殊字符测试\n\n这是一段包含特殊字符的文本：\n\n数学符号: ∑ ∏ ∫ ∞ ≠ ≤ ≥\n货币符号: $ € £ ¥ ¢\n其他符号: © ® ™ § ¶ † ‡";
    
    let parser = ParserEngine::new();
    let document = parser.parse_txt(content);
    
    assert_eq!(document.chapters.len(), 1);
    assert!(!document.chapters[0].content.is_empty());
}

/// 测试极小页面配置
#[test]
fn test_extreme_page_config() {
    let content = "# 测试章节\n\n这是测试内容。";
    
    let parser = ParserEngine::new();
    let document = parser.parse_txt(content);
    
    // 极小的页面配置
    let page_config = PageConfig {
        width: 100.0,
        height: 100.0,
        margin_top: 10.0,
        margin_bottom: 10.0,
        margin_left: 10.0,
        margin_right: 10.0,
    };
    
    let layout_engine = LayoutEngine::new(page_config);
    let pages = layout_engine.layout_document(&document);
    
    // 即使页面很小，也应该能生成页面
    assert!(!pages.is_empty());
}

/// 测试空行和空白字符处理
#[test]
fn test_whitespace_handling() {
    let content = "# 空白字符测试\n\n\n\n\n这是内容。\n\n\n\n\n更多内容。   \n\t\t\t\n结束内容。";
    
    let parser = ParserEngine::new();
    let document = parser.parse_txt(content);
    
    assert_eq!(document.chapters.len(), 1);
    // 应该正确处理空行和空白字符
    assert!(document.chapters[0].content.len() >= 3);
}

/// 测试长单词和长行处理
#[test]
fn test_long_words_and_lines() {
    let content = "# 长文本测试\n\n这是一个包含超长单词的段落：Pneumonoultramicroscopicsilicovolcanoconiosissupercalifragilisticexpialidocious\n\n这是另一个包含很长行的段落，其中包含了很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多很多文字。";
    
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
    
    assert!(!pages.is_empty());
}