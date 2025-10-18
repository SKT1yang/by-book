//! 解析引擎
//! 
//! 负责将原始文本内容解析为结构化的文档模型

use crate::document::{DocumentModel, DocumentMetadata, Chapter, ContentBlock, ContentBlockType, TextStyle};
use std::mem;
use regex::Regex;

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
        // 按行分割内容
        let lines: Vec<&str> = content.lines().collect();
        
        // 章节列表
        let mut chapters = Vec::new();

        // 当前章节的文本块列表
        let mut blocks = Vec::new();
        
        // 当前正在累积的段落内容
        let mut current_paragraph = String::new();
        
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
        
        // 章节标题的正则表达式 - 使用 lazy_static 避免重复编译
        lazy_static::lazy_static! {
            static ref CHAPTER_REGEX: Regex = Regex::new(r"^第[一二三四五六七八九十百千\d]+章").unwrap();
        }
        
        // 存储当前章节标题
        let mut current_chapter_title = String::from("全文"); // 默认标题
        
        for line in lines.iter() {
            // 检查是否为章节标题
            if line.starts_with("# ") || CHAPTER_REGEX.is_match(line.trim()) {
                // 如果有累积的段落内容，添加到当前章节
                if !current_paragraph.is_empty() {
                    let block = ContentBlock {
                        block_type: ContentBlockType::Text,
                        content: mem::take(&mut current_paragraph),
                        styles: default_style.clone(),
                        metrics: None,
                    };
                    blocks.push(block);
                }
                
                // 如果有已有的章节内容，保存为一个章节
                if !blocks.is_empty() {
                    let chapter = Chapter {
                        id: format!("chapter_{}", chapters.len()),
                        title: mem::take(&mut current_chapter_title), // 使用实际的章节标题
                        content: mem::take(&mut blocks),
                    };
                    chapters.push(chapter);
                } else if !chapters.is_empty() {
                    // 特殊情况：如果blocks为空但已有章节，说明是连续的章节标题
                    // 在这种情况下，我们仍然需要更新当前章节标题
                    current_chapter_title = if line.starts_with("# ") {
                        line[2..].to_string() // 移除 "# " 前缀
                    } else {
                        line.trim().to_string() // 使用整行作为章节标题
                    };
                    continue;
                }
                
                // 提取新的章节标题
                current_chapter_title = if line.starts_with("# ") {
                    line[2..].to_string() // 移除 "# " 前缀
                } else {
                    line.trim().to_string() // 使用整行作为章节标题
                };
            } else if line.trim().is_empty() {
                // 空行表示段落结束
                if !current_paragraph.is_empty() {
                    let block = ContentBlock {
                        block_type: if current_paragraph.trim().is_empty() {
                            ContentBlockType::Blank
                        } else {
                            ContentBlockType::Text
                        },
                        content: mem::take(&mut current_paragraph),
                        styles: default_style.clone(),
                        metrics: None,
                    };
                    blocks.push(block);
                }
            } else {
                // 普通文本行，添加到当前段落
                if !current_paragraph.is_empty() {
                    current_paragraph.push('\n');
                }
                current_paragraph.push_str(line);
            }
        }
        
        // 处理最后的段落
        if !current_paragraph.is_empty() {
            let block = ContentBlock {
                block_type: ContentBlockType::Text,
                content: mem::take(&mut current_paragraph),
                styles: default_style.clone(),
                metrics: None,
            };
            blocks.push(block);
        }
        
        // 添加最后一个章节
        if !blocks.is_empty() {
            let chapter = Chapter {
                id: format!("chapter_{}", chapters.len()),
                title: current_chapter_title,
                content: blocks,
            };
            chapters.push(chapter);
        } else if chapters.is_empty() {
            // 如果没有任何内容，创建一个默认章节
            let chapter = Chapter {
                id: "chapter_0".to_string(),
                title: "全文".to_string(),
                content: vec![ContentBlock {
                    block_type: ContentBlockType::Text,
                    content: content.to_string(),
                    styles: default_style.clone(),
                    metrics: None,
                }],
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