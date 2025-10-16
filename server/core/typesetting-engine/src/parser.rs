//! 解析引擎
//! 
//! 负责将原始文本内容解析为结构化的文档模型

use crate::document::{DocumentModel, DocumentMetadata, Chapter, ContentBlock, ContentBlockType, TextStyle};
use std::mem;

/// 解析引擎
/// 
/// 将各种格式的输入内容转换为统一的文档模型
#[derive(Default)]
pub struct ParserEngine;

impl ParserEngine {
    /// 创建一个新的解析引擎实例
    /// 
    /// # Returns
    /// 
    /// 返回一个新的ParserEngine实例
    pub fn new() -> Self {
        ParserEngine
    }

    /// 解析文本内容为文档模型
    /// 
    /// 将输入的文本内容解析为DocumentModel结构
    /// 
    /// # Arguments
    /// 
    /// * `content` - 需要解析的文本内容
    /// 
    /// # Returns
    /// 
    /// 返回解析后的DocumentModel实例
    pub fn parse_txt(&self, content: &str) -> DocumentModel {
        // 按段落分割内容（两个换行符分隔）
        let paragraphs: Vec<&str> = content.split("\n\n").collect();
        
        // 章节列表
        let mut chapters = Vec::new();

        // 当前章节的文本块列表
        let mut blocks = Vec::new();
        
        // 默认样式
        let default_style = TextStyle {
            // 字体大小
            font_size: 16.0,
            // 字体名称
            font_family: "Arial".to_string(),
            // 粗体
            bold: false,
            // 斜体
            italic: false,
        };
        
        for paragraph in paragraphs.iter() {
            // 如果段落以#开头，则认为是章节标题
            if paragraph.starts_with("# ") {
                if !blocks.is_empty() {
                    // 创建前一章节
                    let chapter = Chapter {
                        id: format!("chapter_{}", chapters.len()),
                        title: format!("Chapter {}", chapters.len() + 1),
                        content: mem::take(&mut blocks),
                    };
                    chapters.push(chapter);
                }
            } else {
                // 普通文本块
                let block_type = if paragraph.trim().is_empty() {
                    ContentBlockType::Blank
                } else {
                    ContentBlockType::Text
                };
                
                let block = ContentBlock {
                    block_type,
                    content: paragraph.to_string(),
                    styles: default_style.clone(),
                    metrics: None,
                };
                blocks.push(block);
            }
        }
        
        // 添加最后一个章节
        if !blocks.is_empty() {
            let chapter = Chapter {
                id: format!("chapter_{}", chapters.len()),
                title: format!("Chapter {}", chapters.len() + 1),
                content: blocks,
            };
            chapters.push(chapter);
        }
        
        DocumentModel {
            metadata: DocumentMetadata {
                title: "Sample Document".to_string(),
                author: "Unknown".to_string(),
                created_at: chrono::Utc::now().to_rfc3339(),
            },
            chapters,
            styles: vec![default_style],
        }
    }
}