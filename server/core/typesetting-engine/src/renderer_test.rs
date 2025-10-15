//! 渲染引擎单元测试
//! 
//! 测试渲染引擎相关功能的正确性

#[cfg(test)]
mod tests {
    use crate::renderer::*;
    use crate::layout::*;
    use crate::document::*;

    /// 测试渲染器创建功能
    #[test]
    fn test_renderer_creation() {
        let _renderer = Renderer::new();
        // Just test that it can be created without panic
        assert!(true);
    }

    /// 测试页面渲染功能
    #[test]
    fn test_page_rendering() {
        let page = Page {
            blocks: vec![],
            used_height: 0.0,
        };

        let renderer = Renderer::new();
        let output = renderer.render_page(&page);
        
        assert!(output.contains("--- Page Start ---"));
        assert!(output.contains("--- Page End ---"));
    }

    /// 测试不同类型内容块的渲染
    #[test]
    fn test_different_block_types_rendering() {
        let style = TextStyle {
            font_size: 12.0,
            font_family: "Arial".to_string(),
            bold: false,
            italic: false,
        };

        let blocks = vec![
            ContentBlock {
                block_type: ContentBlockType::Text,
                content: "Text content".to_string(),
                styles: style.clone(),
                metrics: None,
            },
            ContentBlock {
                block_type: ContentBlockType::Title,
                content: "Title content".to_string(),
                styles: style.clone(),
                metrics: None,
            },
            ContentBlock {
                block_type: ContentBlockType::Image,
                content: "Image content".to_string(),
                styles: style.clone(),
                metrics: None,
            },
            ContentBlock {
                block_type: ContentBlockType::Blank,
                content: "".to_string(),
                styles: style.clone(),
                metrics: None,
            },
        ];

        let page = Page {
            blocks,
            used_height: 0.0,
        };

        let renderer = Renderer::new();
        let output = renderer.render_page(&page);
        
        assert!(output.contains("Text: Text content"));
        assert!(output.contains("Title: Title content"));
        assert!(output.contains("Image: Image content"));
    }

    /// 测试多页面渲染
    #[test]
    fn test_multiple_pages_rendering() {
        let page1 = Page {
            blocks: vec![ContentBlock {
                block_type: ContentBlockType::Text,
                content: "Page 1 content".to_string(),
                styles: TextStyle {
                    font_size: 12.0,
                    font_family: "Arial".to_string(),
                    bold: false,
                    italic: false,
                },
                metrics: None,
            }],
            used_height: 0.0,
        };

        let page2 = Page {
            blocks: vec![ContentBlock {
                block_type: ContentBlockType::Text,
                content: "Page 2 content".to_string(),
                styles: TextStyle {
                    font_size: 12.0,
                    font_family: "Arial".to_string(),
                    bold: false,
                    italic: false,
                },
                metrics: None,
            }],
            used_height: 0.0,
        };

        let pages = vec![page1, page2];
        let renderer = Renderer::new();
        let output = renderer.render_pages(&pages);
        
        assert!(output.contains("Page 1:"));
        assert!(output.contains("Page 2:"));
        assert!(output.contains("Text: Page 1 content"));
        assert!(output.contains("Text: Page 2 content"));
    }

    /// 测试空页面渲染
    #[test]
    fn test_empty_page_rendering() {
        let page = Page {
            blocks: vec![],
            used_height: 0.0,
        };

        let renderer = Renderer::new();
        let output = renderer.render_page(&page);
        
        assert!(output.contains("--- Page Start ---"));
        assert!(output.contains("--- Page End ---"));
        // 空页面不应该包含其他内容
        assert!(!output.contains("Text:"));
    }
}