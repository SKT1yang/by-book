//! 文档模型定义
//! 
//! 这个模块定义了排版引擎的核心数据结构，包括文档、章节、内容块等。

use std::borrow::Cow;

/// 文档元数据
/// 
/// 包含文档的基本信息，如标题、作者和创建时间
#[derive(Debug, Clone, PartialEq)]
pub struct DocumentMetadata {
    /// 文档标题
    pub title: Cow<'static, str>,
    /// 文档作者
    pub author: Cow<'static, str>,
    /// 创建时间（RFC3339格式）
    pub created_at: Cow<'static, str>,
}

/// 章节结构
/// 
/// 代表文档中的一个章节，包含章节ID、标题和内容块列表
#[derive(Debug, Clone, PartialEq)]
pub struct Chapter {
    /// 章节唯一标识符
    pub id: Cow<'static, str>,
    /// 章节标题
    pub title: Cow<'static, str>,
    /// 章节内容块列表
    pub content: Vec<ContentBlock>,
}

/// 内容块类型枚举
/// 
/// 定义了文档中可能的内容类型
#[derive(Debug, Clone, PartialEq)]
pub enum ContentBlockType {
    /// 普通文本内容
    Text,
    /// 图片内容
    Image,
    /// 标题内容
    Title,
    /// 空白内容
    Blank,
}

/// 文本样式
/// 
/// 定义文本的显示样式属性
#[derive(Debug, Clone, PartialEq)]
pub struct TextStyle {
    /// 字体大小（像素）
    pub font_size: f32,
    /// 字体族
    pub font_family: Cow<'static, str>,
    /// 是否粗体
    pub bold: bool,
    /// 是否斜体
    pub italic: bool,
}

/// 内容块
/// 
/// 文档的基本组成单元，可以是文本、图片等不同类型的内容
#[derive(Debug, Clone, PartialEq)]
pub struct ContentBlock {
    /// 内容块类型
    pub block_type: ContentBlockType,
    /// 内容文本
    pub content: Cow<'static, str>,
    /// 文本样式
    pub styles: TextStyle,
    /// 布局测量数据（可选）
    pub metrics: Option<LayoutMetrics>,
}

/// 布局测量数据
/// 
/// 包含内容块的尺寸信息，用于布局计算
#[derive(Debug, Clone, PartialEq)]
pub struct LayoutMetrics {
    /// 宽度（像素）
    pub width: f32,
    /// 高度（像素）
    pub height: f32,
}

/// 文档模型
/// 
/// 排版引擎的核心数据结构，表示整个文档
#[derive(Debug, Clone, PartialEq)]
pub struct DocumentModel {
    /// 文档元数据
    pub metadata: DocumentMetadata,
    /// 章节列表
    pub chapters: Vec<Chapter>,
    /// 样式定义列表
    pub styles: Vec<TextStyle>,
}