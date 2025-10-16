//! 排版引擎示例程序
//! 
//! 演示如何使用自研排版引擎处理文档

use typesetting_engine::{ParserEngine, LayoutEngine, PageConfig};
use anyhow::Result;
use std::fs;

/// 程序入口点
/// 
/// 演示排版引擎的完整工作流程：
/// 1. 从文件读取内容
/// 2. 解析文档
/// 3. 布局计算
/// 4. 渲染输出（这里只输出布局数据）
fn main() -> Result<()> {
    // 从文件读取内容
    let content = fs::read_to_string("《玄鉴仙族》.txt")?;
    
    // 创建解析引擎
    let parser = ParserEngine::new();
    
    // 解析文档
    let document = parser.parse_txt(&content);
    println!("Parsed document with {} chapters", document.chapters.len());
    
    // 打印章节信息
    for (i, chapter) in document.chapters.iter().enumerate() {
        println!("Chapter {}: {} ({} blocks)", i+1, chapter.title, chapter.content.len());
    }
    
    // 创建布局引擎
    let page_config = PageConfig {
        width: 400.0,
        height: 600.0,
        margin_top: 20.0,
        margin_bottom: 20.0,
        margin_left: 20.0,
        margin_right: 20.0,
    };
    let layout_engine = LayoutEngine::new(page_config);
    
    // 布局文档
    let pages = layout_engine.layout_document(&document);
    println!("\nLayout completed: {} pages generated", pages.len());
    
    // 输出前几页布局数据，而不是渲染后的内容
    let pages_to_show = pages.len().min(3); // 只显示前3页
    for (i, page) in pages.iter().enumerate().take(pages_to_show) {
        println!("\n--- Page {} ---", i + 1);
        println!("Used height: {}", page.used_height);
        println!("Blocks: {}", page.blocks.len());
        for (j, block) in page.blocks.iter().enumerate() {
            // 只显示内容的前100个字符以避免输出过多，确保在字符边界上截断
            let content_preview = if block.content.chars().count() > 100 {
                block.content.chars().take(100).collect::<String>() + "..."
            } else {
                block.content.clone()
            };
            println!("  Block {}: {:?} - {}", j + 1, block.block_type, content_preview);
        }
    }
    
    if pages.len() > pages_to_show {
        println!("\n... and {} more pages", pages.len() - pages_to_show);
    }
    
    Ok(())
}