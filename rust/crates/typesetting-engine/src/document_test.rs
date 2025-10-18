//! 文档模型单元测试
//! 
//! 测试文档模型相关功能的正确性

#[cfg(test)]
mod tests {
    use crate::document::*;
    use std::borrow::Cow;

    /// 测试文档创建功能
    #[test]
    fn test_document_creation() {
        let metadata = DocumentMetadata {
            title: Cow::Borrowed("Test Document"),
            author: Cow::Borrowed("Test Author"),
            created_at: Cow::Borrowed("2023-01-01"),
        };

        let chapter = Chapter {
            id: Cow::Borrowed("chapter_1"),
            title: Cow::Borrowed("Test Chapter"),
            content: vec![],
        };

        let document = DocumentModel {
            metadata,
            chapters: vec![chapter],
            styles: vec![],
        };

        assert_eq!(document.metadata.title, "Test Document");
        assert_eq!(document.chapters.len(), 1);
        assert_eq!(document.chapters[0].id, "chapter_1");
    }

    /// 测试内容块创建功能
    #[test]
    fn test_content_block_creation() {
        let style = TextStyle {
            font_size: 12.0,
            font_family: Cow::Borrowed("Arial"),
            bold: false,
            italic: true,
        };

        let block = ContentBlock {
            block_type: ContentBlockType::Text,
            content: Cow::Borrowed("Hello, world!"),
            styles: style.clone(),
            metrics: None,
        };

        assert_eq!(block.content, "Hello, world!");
        assert_eq!(block.styles.font_size, 12.0);
        assert_eq!(block.styles.italic, true);
    }

    /// 测试不同内容块类型
    #[test]
    fn test_content_block_types() {
        let style = TextStyle {
            font_size: 12.0,
            font_family: Cow::Borrowed("Arial"),
            bold: false,
            italic: false,
        };

        let text_block = ContentBlock {
            block_type: ContentBlockType::Text,
            content: Cow::Borrowed("Text content"),
            styles: style.clone(),
            metrics: None,
        };

        let title_block = ContentBlock {
            block_type: ContentBlockType::Title,
            content: Cow::Borrowed("Title content"),
            styles: style.clone(),
            metrics: None,
        };

        let image_block = ContentBlock {
            block_type: ContentBlockType::Image,
            content: Cow::Borrowed("Image content"),
            styles: style.clone(),
            metrics: None,
        };

        let blank_block = ContentBlock {
            block_type: ContentBlockType::Blank,
            content: Cow::Borrowed(""),
            styles: style.clone(),
            metrics: None,
        };

        assert!(matches!(text_block.block_type, ContentBlockType::Text));
        assert!(matches!(title_block.block_type, ContentBlockType::Title));
        assert!(matches!(image_block.block_type, ContentBlockType::Image));
        assert!(matches!(blank_block.block_type, ContentBlockType::Blank));
    }

    /// 测试带有布局测量数据的内容块
    #[test]
    fn test_content_block_with_metrics() {
        let style = TextStyle {
            font_size: 12.0,
            font_family: Cow::Borrowed("Arial"),
            bold: false,
            italic: false,
        };

        let metrics = LayoutMetrics {
            width: 100.0,
            height: 50.0,
        };

        let block = ContentBlock {
            block_type: ContentBlockType::Text,
            content: Cow::Borrowed("Text content"),
            styles: style,
            metrics: Some(metrics.clone()),
        };

        assert_eq!(block.metrics, Some(metrics));
    }
}