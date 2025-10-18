//! 解析引擎
//! 
//! 负责将原始文本内容解析为结构化的文档模型

use crate::document::{DocumentModel, DocumentMetadata, Chapter, ContentBlock, ContentBlockType, TextStyle};
use std::mem;
use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use regex::Regex;

/// 解析引擎
/// 
/// 将各种格式的输入内容转换为统一的文档模型
pub struct ParserEngine {
    /// 章节缓存，用于存储已解析的章节以避免重复处理
    chapter_cache: Arc<Mutex<HashMap<String, Chapter>>>,
}

impl ParserEngine {
    /// 创建一个新的解析引擎实例
    /// 
    /// # Returns
    /// 
    /// 返回一个新的ParserEngine实例
    pub fn new() -> Self {
        ParserEngine {
            chapter_cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// 创建一个新的解析引擎实例（带缓存）
    /// 
    /// # Arguments
    /// 
    /// * `chapter_cache` - 章节缓存
    /// 
    /// # Returns
    /// 
    /// 返回一个新的ParserEngine实例
    pub fn with_cache(chapter_cache: Arc<Mutex<HashMap<String, Chapter>>>) -> Self {
        ParserEngine {
            chapter_cache,
        }
    }

    /// 从缓存中获取章节
    /// 
    /// # Arguments
    /// 
    /// * `title` - 章节标题
    /// 
    /// # Returns
    /// 
    /// 如果缓存中存在该章节则返回Some(Chapter)，否则返回None
    pub fn get_cached_chapter(&self, title: &str) -> Option<Chapter> {
        let cache = self.chapter_cache.lock().unwrap();
        cache.get(title).cloned()
    }

    /// 将章节添加到缓存中
    /// 
    /// # Arguments
    /// 
    /// * `chapter` - 要缓存的章节
    pub fn cache_chapter(&self, chapter: Chapter) {
        let mut cache = self.chapter_cache.lock().unwrap();
        cache.insert(chapter.title.to_string(), chapter);
    }

    /// 获取缓存实例
    /// 
    /// # Returns
    /// 
    /// 返回缓存实例的Arc引用
    pub fn get_cache(&self) -> Arc<Mutex<HashMap<String, Chapter>>> {
        self.chapter_cache.clone()
    }

    /// 解析特定章节的文本内容
    /// 
    /// 将输入的文本内容解析为DocumentModel结构，只包含指定章节
    /// 
    /// # Arguments
    /// 
    /// * `content` - 需要解析的文本内容
    /// * `chapter_index` - 要解析的章节索引
    /// 
    /// # Returns
    /// 
    /// 返回解析后的DocumentModel实例，只包含指定章节
    pub fn parse_txt_chapter(&self, content: &str, chapter_index: usize) -> DocumentModel {
        let all_chapters = self.parse_all_chapters(content);
        
        if chapter_index < all_chapters.len() {
            let chapter = all_chapters[chapter_index].clone();
            DocumentModel {
                metadata: DocumentMetadata {
                    title: Cow::Borrowed("Sample Document"),
                    author: Cow::Borrowed("Unknown"),
                    created_at: Cow::Owned(chrono::Utc::now().to_rfc3339()),
                },
                chapters: vec![chapter],
                styles: vec![TextStyle {
                    font_size: 16.0,
                    font_family: Cow::Borrowed("Arial"),
                    bold: false,
                    italic: false,
                }],
            }
        } else {
            // 如果索引超出范围，返回空文档
            DocumentModel {
                metadata: DocumentMetadata {
                    title: Cow::Borrowed("Sample Document"),
                    author: Cow::Borrowed("Unknown"),
                    created_at: Cow::Owned(chrono::Utc::now().to_rfc3339()),
                },
                chapters: vec![],
                styles: vec![TextStyle {
                    font_size: 16.0,
                    font_family: Cow::Borrowed("Arial"),
                    bold: false,
                    italic: false,
                }],
            }
        }
    }

    /// 解析所有章节但不进行缓存
    /// 
    /// # Arguments
    /// 
    /// * `content` - 需要解析的文本内容
    /// 
    /// # Returns
    /// 
    /// 返回解析后的章节列表
    fn parse_all_chapters(&self, content: &str) -> Vec<Chapter> {
        // 按行分割内容
        let lines: Vec<&str> = content.lines().collect();
        
        // 章节列表
        let mut chapters = Vec::new();

        // 当前章节的文本块列表
        let mut blocks = Vec::new();
        
        // 当前正在累积的段落内容 - 预分配容量以提高性能
        let mut current_paragraph = String::with_capacity(1024);
        
        // 默认样式 - 创建一次，通过引用使用
        let default_style = TextStyle {
            // 字体大小
            font_size: 16.0,
            // 字体名称
            font_family: Cow::Borrowed("Arial"),
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
        let mut current_chapter_title = Cow::Borrowed("全文"); // 默认标题
        
        for line in lines.iter() {
            // 检查是否为章节标题
            if line.starts_with("# ") || CHAPTER_REGEX.is_match(line.trim()) {
                // 如果有累积的段落内容，添加到当前章节
                if !current_paragraph.is_empty() {
                    let block = ContentBlock {
                        block_type: ContentBlockType::Text,
                        content: Cow::Owned(mem::take(&mut current_paragraph)),
                        styles: default_style.clone(),
                        metrics: None,
                    };
                    // 重新分配容量
                    current_paragraph.reserve(1024);
                    blocks.push(block);
                }
                
                // 如果有已有的章节内容，保存为一个章节
                if !blocks.is_empty() {
                    let chapter = Chapter {
                        id: Cow::Owned(format!("chapter_{}", chapters.len())),
                        title: mem::take(&mut current_chapter_title), // 使用实际的章节标题
                        content: mem::take(&mut blocks),
                    };
                    
                    chapters.push(chapter);
                } else if !chapters.is_empty() {
                    // 特殊情况：如果blocks为空但已有章节，说明是连续的章节标题
                    // 在这种情况下，我们仍然需要更新当前章节标题
                    current_chapter_title = if let Some(stripped) = line.strip_prefix("# ") {
                        Cow::Owned(stripped.to_string()) // 移除 "# " 前缀
                    } else {
                        Cow::Owned(line.trim().to_string()) // 使用整行作为章节标题
                    };
                    continue;
                }
                
                // 提取新的章节标题
                current_chapter_title = if let Some(stripped) = line.strip_prefix("# ") {
                    Cow::Owned(stripped.to_string()) // 移除 "# " 前缀
                } else {
                    Cow::Owned(line.trim().to_string()) // 使用整行作为章节标题
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
                        content: Cow::Owned(mem::take(&mut current_paragraph)),
                        styles: default_style.clone(),
                        metrics: None,
                    };
                    // 重新分配容量
                    current_paragraph.reserve(1024);
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
                content: Cow::Owned(mem::take(&mut current_paragraph)),
                styles: default_style.clone(),
                metrics: None,
            };
            blocks.push(block);
        }
        
        // 添加最后一个章节
        if !blocks.is_empty() {
            let chapter = Chapter {
                id: Cow::Owned(format!("chapter_{}", chapters.len())),
                title: current_chapter_title,
                content: blocks,
            };
            
            chapters.push(chapter);
        } else if chapters.is_empty() {
            // 如果没有任何内容，创建一个默认章节
            let block = ContentBlock {
                block_type: ContentBlockType::Text,
                content: Cow::Owned(content.to_string()),
                styles: default_style.clone(),
                metrics: None,
            };
            
            let chapter = Chapter {
                id: Cow::Borrowed("chapter_0"),
                title: Cow::Borrowed("全文"),
                content: vec![block],
            };
            
            chapters.push(chapter);
        }
        
        chapters
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
        let chapters = self.parse_all_chapters(content);
        
        // 缓存所有章节
        for chapter in &chapters {
            self.cache_chapter(chapter.clone());
        }
        
        DocumentModel {
            metadata: DocumentMetadata {
                title: Cow::Borrowed("Sample Document"),
                author: Cow::Borrowed("Unknown"),
                created_at: Cow::Owned(chrono::Utc::now().to_rfc3339()),
            },
            chapters,
            styles: vec![TextStyle {
                font_size: 16.0,
                font_family: Cow::Borrowed("Arial"),
                bold: false,
                italic: false,
            }],
        }
    }
}