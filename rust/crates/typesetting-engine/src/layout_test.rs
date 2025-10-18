//! 布局引擎单元测试
//! 
//! 测试布局引擎相关功能的正确性

#[cfg(test)]
mod tests {
    use crate::layout::*;
    use crate::document::*;

    /// 测试页面配置功能
    #[test]
    fn test_page_config() {
        let config = PageConfig {
            width: 400.0,
            height: 600.0,
            margin_top: 20.0,
            margin_bottom: 20.0,
            margin_left: 20.0,
            margin_right: 20.0,
        };

        assert_eq!(config.content_width(), 360.0);
        assert_eq!(config.content_height(), 560.0);
    }

    /// 测试布局引擎创建功能
    #[test]
    fn test_layout_engine_creation() {
        let config = PageConfig {
            width: 400.0,
            height: 600.0,
            margin_top: 20.0,
            margin_bottom: 20.0,
            margin_left: 20.0,
            margin_right: 20.0,
        };

        let _engine = LayoutEngine::new(config);
        // Just test that it can be created without panic
        assert!(true);
    }

    /// 测试空文档布局
    #[test]
    fn test_empty_document_layout() {
        let config = PageConfig {
            width: 400.0,
            height: 600.0,
            margin_top: 20.0,
            margin_bottom: 20.0,
            margin_left: 20.0,
            margin_right: 20.0,
        };

        let engine = LayoutEngine::new(config);
        
        let document = DocumentModel {
            metadata: DocumentMetadata {
                title: "Test".into(),
                author: "Author".into(),
                created_at: "2023-01-01".into(),
            },
            chapters: vec![],
            styles: vec![],
        };

        let pages = engine.layout_document(&document);
        assert_eq!(pages.len(), 0);
    }

    /// 测试单个内容块布局
    #[test]
    fn test_single_block_layout() {
        let config = PageConfig {
            width: 400.0,
            height: 600.0,
            margin_top: 20.0,
            margin_bottom: 20.0,
            margin_left: 20.0,
            margin_right: 20.0,
        };

        let engine = LayoutEngine::new(config);
        
        let style = TextStyle {
            font_size: 16.0,
            font_family: "Arial".into(),
            bold: false,
            italic: false,
        };

        let block = ContentBlock {
            block_type: ContentBlockType::Text,
            content: "This is a test block.".into(),
            styles: style,
            metrics: None,
        };

        let document = DocumentModel {
            metadata: DocumentMetadata {
                title: "Test".into(),
                author: "Author".into(),
                created_at: "2023-01-01".into(),
            },
            chapters: vec![Chapter {
                id: "1".into(),
                title: "Test Chapter".into(),
                content: vec![block],
            }],
            styles: vec![],
        };

        let pages = engine.layout_document(&document);
        assert_eq!(pages.len(), 1);
        assert_eq!(pages[0].blocks.len(), 1);
    }

    /// 测试内容块测量
    #[test]
    fn test_block_measurement() {
        let config = PageConfig {
            width: 400.0,
            height: 600.0,
            margin_top: 20.0,
            margin_bottom: 20.0,
            margin_left: 20.0,
            margin_right: 20.0,
        };

        let engine = LayoutEngine::new(config);
        
        let style = TextStyle {
            font_size: 16.0,
            font_family: "Arial".into(),
            bold: false,
            italic: false,
        };

        let block = ContentBlock {
            block_type: ContentBlockType::Text,
            content: "Line 1\nLine 2\nLine 3".into(),
            styles: style,
            metrics: None,
        };

        let metrics = engine.measure_block(&block);
        assert_eq!(metrics.width, 360.0); // content_width
        assert!(metrics.height > 0.0); // Should have some height
    }

    /// 测试页面适应性检查
    #[test]
    fn test_page_fit_check() {
        let config = PageConfig {
            width: 400.0,
            height: 600.0,
            margin_top: 20.0,
            margin_bottom: 20.0,
            margin_left: 20.0,
            margin_right: 20.0,
        };

        let engine = LayoutEngine::new(config);
        
        let page = Page {
            blocks: vec![],
            used_height: 100.0,
        };

        let metrics_small = LayoutMetrics {
            width: 100.0,
            height: 100.0,
        };

        let metrics_large = LayoutMetrics {
            width: 100.0,
            height: 500.0,
        };

        // 小块应该能适应
        assert!(engine.can_fit_in_page(&metrics_small, &page));
        // 大块不应该能适应
        assert!(!engine.can_fit_in_page(&metrics_large, &page));
    }
}