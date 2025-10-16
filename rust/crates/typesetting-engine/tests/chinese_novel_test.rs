//! 中文小说测试
//! 
//! 测试中文小说章节识别和处理功能

use typesetting_engine::{ParserEngine, LayoutEngine, PageConfig};

/// 测试中文小说章节标题识别
#[test]
fn test_chinese_novel_chapter_detection() {
    let content = "第1章 开始
    
这是第一章的内容。

第2章 继续

这是第二章的内容。

第十一章 复杂章节标题

这是第十一章的内容。

第一百二十三章 数字章节

这是第一百二十三章的内容。";

    let parser = ParserEngine::new();
    let document = parser.parse_txt(content);
    
    // 应该识别出4个章节
    assert_eq!(document.chapters.len(), 4);
    
    // 验证章节内容不为空
    for (i, chapter) in document.chapters.iter().enumerate() {
        assert!(!chapter.content.is_empty(), "第{}章内容不应该为空", i + 1);
    }
}

/// 测试《玄鉴仙族》样例章节
#[test]
fn test_xuanjian_xianzu_chapter_sample() {
    let content = "玄鉴仙族
作者：季越人

第一章 初入
    
陆江仙做了一个很长很长的梦，梦见田间种稻，梦见刀光剑影，梦见仙宗、女子、大湖。

第二章 李家

李木田寅时醒了，睁着个眼盯着自家破房顶，黑漆漆的透出一点点辉光。

第三章 鉴子

“害，项平哥。”
李项平背着筐往回走，远远地走来一个女孩，女孩脸蛋圆圆，五官很是平凡，满满的笑容让眉眼平添不少魅力。";

    let parser = ParserEngine::new();
    let document = parser.parse_txt(content);
    
    // 应该识别出3个章节（加上开头的介绍）
    assert_eq!(document.chapters.len(), 4); // 实际上是4个章节，因为开头的内容也会被当作一个章节
    
    // 验证章节内容不为空
    for (i, chapter) in document.chapters.iter().enumerate() {
        assert!(!chapter.content.is_empty(), "第{}章内容不应该为空", i + 1);
    }
}

/// 测试中文小说分页功能
#[test]
fn test_chinese_novel_pagination() {
    let content = "第1章 开始
    
这是第一章的内容，包含一些文本用于测试分页功能。".repeat(100);

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