//! 渲染引擎
//! 
//! 负责将页面结构渲染为可视化的输出

use crate::layout::Page;

/// 渲染引擎
/// 
/// 将页面结构转换为可视化的文本表示
#[derive(Default)]
pub struct Renderer;

impl Renderer {
    /// 创建一个新的渲染器实例
    /// 
    /// # Returns
    /// 
    /// 返回一个新的Renderer实例
    pub fn new() -> Self {
        Renderer
    }

    /// 渲染单个页面为字符串表示
    /// 
    /// 将页面内容转换为可读的文本格式
    /// 
    /// # Arguments
    /// 
    /// * `page` - 需要渲染的页面
    /// 
    /// # Returns
    /// 
    /// 返回渲染后的字符串
    pub fn render_page(&self, page: &Page) -> String {
        let mut result = String::new();
        result.push_str("--- Page Start ---\n");
        
        // 遍历页面中的所有内容块
        for block in &page.blocks {
            match block.block_type {
                crate::document::ContentBlockType::Text => {
                    result.push_str(&format!("Text: {}\n", block.content));
                }
                crate::document::ContentBlockType::Title => {
                    result.push_str(&format!("Title: {}\n", block.content));
                }
                crate::document::ContentBlockType::Image => {
                    result.push_str(&format!("Image: {}\n", block.content));
                }
                crate::document::ContentBlockType::Blank => {
                    result.push('\n');
                }
            }
        }
        
        result.push_str("--- Page End ---\n");
        result
    }
    
    /// 渲染所有页面
    /// 
    /// 将页面列表转换为可读的文本格式
    /// 
    /// # Arguments
    /// 
    /// * `pages` - 需要渲染的页面列表
    /// 
    /// # Returns
    /// 
    /// 返回渲染后的字符串
    pub fn render_pages(&self, pages: &[Page]) -> String {
        let mut result = String::new();
        for (i, page) in pages.iter().enumerate() {
            result.push_str(&format!("Page {}: \n{}\n", i + 1, self.render_page(page)));
        }
        result
    }
}