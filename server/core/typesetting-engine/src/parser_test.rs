//! 解析引擎单元测试
//! 
//! 测试解析引擎相关功能的正确性

#[cfg(test)]
mod tests {
    use crate::parser::*;

    /// 测试简单文档解析功能
    #[test]
    fn test_simple_parsing() {
        let content = "# Chapter 1\n\nThis is a paragraph.\n\nThis is another paragraph.";
        let parser = ParserEngine::new();
        let document = parser.parse_txt(content);

        assert_eq!(document.chapters.len(), 1);
        assert_eq!(document.chapters[0].content.len(), 2);
        assert_eq!(document.metadata.title, "Sample Document");
    }

    /// 测试多章节文档解析功能
    #[test]
    fn test_multiple_chapters() {
        let content = "# Chapter 1\n\nFirst chapter content.\n\n# Chapter 2\n\nSecond chapter content.";
        let parser = ParserEngine::new();
        let document = parser.parse_txt(content);

        assert_eq!(document.chapters.len(), 2);
        assert_eq!(document.chapters[0].title, "Chapter 1");
        assert_eq!(document.chapters[1].title, "Chapter 2");
    }

    /// 测试空文档解析
    #[test]
    fn test_empty_document() {
        let content = "";
        let parser = ParserEngine::new();
        let document = parser.parse_txt(content);

        // 空文档会创建一个章节
        assert_eq!(document.chapters.len(), 1);
    }

    /// 测试只有章节标题的文档
    #[test]
    fn test_document_with_only_chapters() {
        let content = "# Chapter 1\n\n# Chapter 2\n\n# Chapter 3";
        let parser = ParserEngine::new();
        let document = parser.parse_txt(content);

        // 当前解析器的实现方式下，章节标题会创建新的章节
        assert_eq!(document.chapters.len(), 0);
    }

    /// 测试包含空白段落的文档
    #[test]
    fn test_document_with_blank_paragraphs() {
        let content = "# Chapter 1\n\n\n\nThis is content.\n\n\n\nMore content.";
        let parser = ParserEngine::new();
        let document = parser.parse_txt(content);

        assert_eq!(document.chapters.len(), 1);
        // 应该包含空白块和文本块
        assert!(document.chapters[0].content.len() >= 3);
    }

    /// 测试复杂文档结构
    #[test]
    fn test_complex_document_structure() {
        let content = "# 第一章 简介\n\n这是第一章的内容。\n\n## 小节标题\n\n这是小节的内容。\n\n# 第二章 详细内容\n\n这是第二章的内容。\n\n列表项1\n列表项2\n列表项3";
        let parser = ParserEngine::new();
        let document = parser.parse_txt(content);

        // 当前解析器会根据章节标题创建章节
        assert_eq!(document.chapters.len(), 2);
        assert!(!document.chapters[0].content.is_empty());
    }
}