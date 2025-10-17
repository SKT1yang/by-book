//! 文件加载器
//! 
//! 负责从文件系统加载多种格式的文件，并处理不同的字符编码

use crate::prelude::*;
use std::path::Path;

/// 支持的文件格式
#[derive(Debug, Clone, PartialEq)]
pub enum FileFormat {
    /// 纯文本文件
    Txt,
    /// 未来支持的其他格式
    #[allow(dead_code)]
    Other(String),
}

/// 文件加载器
/// 
/// 提供跨平台的文件加载和编码处理功能，支持多种文件格式
#[derive(Default)]
pub struct FileLoader;

impl FileLoader {
    /// 创建一个新的文件加载器实例
    /// 
    /// # Returns
    /// 
    /// 返回一个新的FileLoader实例
    pub fn new() -> Self {
        FileLoader
    }

    /// 检测文件格式
    /// 
    /// 根据文件扩展名检测文件格式
    /// 
    /// # Arguments
    /// 
    /// * `file_path` - 文件路径
    /// 
    /// # Returns
    /// 
    /// 返回检测到的文件格式
    pub fn detect_format(file_path: &str) -> FileFormat {
        let path = Path::new(file_path);
        if let Some(extension) = path.extension() {
            match extension.to_str() {
                Some("txt") | Some("TXT") => FileFormat::Txt,
                Some(ext) => FileFormat::Other(ext.to_lowercase()),
                None => FileFormat::Other(String::new()),
            }
        } else {
            // 默认为txt格式
            FileFormat::Txt
        }
    }

    /// 加载文本文件
    /// 
    /// 自动检测文件编码并将其转换为UTF-8字符串
    /// 
    /// # Arguments
    /// 
    /// * `file_path` - 文件路径
    /// 
    /// # Returns
    /// 
    /// 返回文件内容的UTF-8字符串表示
    /// 
    /// # Errors
    /// 
    /// 当文件无法读取时返回错误
    pub fn load_text_file(&self, file_path: &str) -> Result<String> {
        use std::fs;
        
        // 读取为字节以便进行编码检测
        let bytes = fs::read(file_path)?;
        
        // 首先尝试以UTF-8读取
        match String::from_utf8(bytes.clone()) {
            Ok(content) => {
                Ok(content)
            },
            Err(_) => {
                // 如果UTF-8读取失败，使用Latin-1编码作为后备
                // Latin-1编码是一种单字节编码，可以无损地表示所有256个字节值
                let content = bytes.iter().map(|&b| b as char).collect();
                Ok(content)
            }
        }
    }
    
    /// 加载并解析文档
    /// 
    /// 加载文件并将其解析为文档模型
    /// 
    /// # Arguments
    /// 
    /// * `file_path` - 文件路径
    /// 
    /// # Returns
    /// 
    /// 返回解析后的文档模型
    /// 
    /// # Errors
    /// 
    /// 当文件无法读取或解析时返回错误
    pub fn load_and_parse_document(&self, file_path: &str) -> Result<crate::DocumentModel> {
        use crate::ParserEngine;
        
        // 检测文件格式
        let format = Self::detect_format(file_path);
        
        // 根据不同格式加载文件
        let content = match format {
            FileFormat::Txt => {
                self.load_text_file(file_path)?
            },
            FileFormat::Other(_) => {
                // 对于其他格式，暂时也使用文本加载方式
                self.load_text_file(file_path)?
            }
        };
        
        // 创建解析器并解析内容
        let parser = ParserEngine::new();
        let document = parser.parse_txt(&content);
        
        Ok(document)
    }
}