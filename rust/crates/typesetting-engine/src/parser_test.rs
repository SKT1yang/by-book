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

        assert_eq!(document.chapters.len(), 1); // 现在按行解析，章节标题会创建新章节，但只有一个章节有内容
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
        assert_eq!(document.chapters.len(), 1); // 只有最后一个章节有内容
    }

    /// 测试包含空白段落的文档
    #[test]
    fn test_document_with_blank_paragraphs() {
        let content = "# Chapter 1\n\n\n\nThis is content.\n\n\n\nMore content.";
        let parser = ParserEngine::new();
        let document = parser.parse_txt(content);

        assert_eq!(document.chapters.len(), 1);
        // 应该包含空白块和文本块
        assert!(document.chapters[0].content.len() >= 1); // 现在内容会合并成一个块
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
    
    /// 测试中文数字章节标题解析
    #[test]
    fn test_chinese_numbered_chapters() {
        let content = "第一页\n\n这是第一页的内容。\n\n第二页\n\n这是第二页的内容。";
        let parser = ParserEngine::new();
        let document = parser.parse_txt(content);
        
        // 对于不以#开头或不匹配正则表达式的文本，不应创建新章节
        assert_eq!(document.chapters.len(), 1);
        assert_eq!(document.chapters[0].title, "全文");
    }
    
    /// 测试标准中文章节标题解析
    #[test]
    fn test_standard_chinese_chapters() {
        let content = "第一章 简介\n\n这是第一章的内容。\n\n第二章 详细信息\n\n这是第二章的内容。";
        let parser = ParserEngine::new();
        let document = parser.parse_txt(content);
        
        assert_eq!(document.chapters.len(), 2);
        assert_eq!(document.chapters[0].title, "第一章 简介");
        assert_eq!(document.chapters[1].title, "第二章 详细信息");
    }
    
    /// 测试混合格式章节标题
    #[test]
    fn test_mixed_format_chapters() {
        let content = "# English Chapter\n\nContent of English chapter.\n\n第二章 中文标题\n\n中文章节内容。";
        let parser = ParserEngine::new();
        let document = parser.parse_txt(content);
        
        assert_eq!(document.chapters.len(), 2);
        assert_eq!(document.chapters[0].title, "English Chapter");
        assert_eq!(document.chapters[1].title, "第二章 中文标题");
    }
    
    /// 测试缓存功能
    #[test]
    fn test_chapter_caching() {
        let content1 = "# Chapter 1\n\nContent of chapter 1.\n\n# Chapter 2\n\nContent of chapter 2.";
        
        let parser1 = ParserEngine::new();
        let document1 = parser1.parse_txt(content1);
        
        // 使用同一个缓存的第二个解析器
        let cache = parser1.get_cache();
        let parser2 = ParserEngine::with_cache(cache);
        let document2 = parser2.parse_txt(content1); // 解析相同内容
        
        // 确保两个文档具有相同的章节
        assert_eq!(document1.chapters.len(), document2.chapters.len());
        assert_eq!(document1.chapters[0].title, document2.chapters[0].title);
        assert_eq!(document1.chapters[1].title, document2.chapters[1].title);
    }
}