//! 异步操作模块
//! 
//! 包含排版引擎的异步处理功能

use crate::{ParserEngine, DocumentModel, DocumentMetadata, layout::{LayoutEngine, PageConfig, Page}};
use std::borrow::Cow;

/// 异步解析整个文档
/// 
/// # Arguments
/// 
/// * `content` - 文档内容
/// 
/// # Returns
/// 
/// 返回解析后的文档模型
pub async fn parse_document_async(content: String) -> DocumentModel {
    // 在后台线程中执行计算密集型任务
    tokio::task::spawn_blocking(move || {
        let parser = ParserEngine::new();
        parser.parse_txt(&content)
    }).await.unwrap_or_else(|_| DocumentModel {
        metadata: DocumentMetadata {
            title: Cow::Borrowed("Error Document"),
            author: Cow::Borrowed("Unknown"),
            created_at: Cow::Owned(chrono::Utc::now().to_rfc3339()),
        },
        chapters: Vec::new(),
        styles: Vec::new(),
    })
}

/// 异步布局文档
/// 
/// # Arguments
/// 
/// * `document` - 文档模型
/// * `page_config` - 页面配置
/// 
/// # Returns
/// 
/// 返回布局后的页面列表
pub async fn layout_document_async(
    document: DocumentModel, 
    page_config: PageConfig
) -> Vec<Page> {
    // 在后台线程中执行计算密集型任务
    tokio::task::spawn_blocking(move || {
        let layout_engine = LayoutEngine::new(page_config);
        layout_engine.layout_document(&document)
    }).await.unwrap_or_else(|_| Vec::new())
}

/// 按需加载和布局特定章节
/// 
/// # Arguments
/// 
/// * `content` - 文档内容
/// * `chapter_index` - 章节索引
/// * `page_config` - 页面配置
/// 
/// # Returns
/// 
/// 返回布局后的页面列表
pub fn layout_chapter_on_demand(
    content: &str, 
    chapter_index: usize, 
    page_config: PageConfig
) -> Vec<Page> {
    let parser = ParserEngine::new();
    let document = parser.parse_txt_chapter(content, chapter_index);
    
    let layout_engine = LayoutEngine::new(page_config);
    layout_engine.layout_document_chapter(&document, 0) // 章节文档中的索引始终是0
}

/// 异步按需加载和布局特定章节
/// 
/// # Arguments
/// 
/// * `content` - 文档内容
/// * `chapter_index` - 章节索引
/// * `page_config` - 页面配置
/// 
/// # Returns
/// 
/// 返回布局后的页面列表
pub async fn layout_chapter_on_demand_async(
    content: String, 
    chapter_index: usize, 
    page_config: PageConfig
) -> Vec<Page> {
    // 在后台线程中执行计算密集型任务
    tokio::task::spawn_blocking(move || {
        layout_chapter_on_demand(&content, chapter_index, page_config)
    }).await.unwrap_or_else(|_| Vec::new())
}