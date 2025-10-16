//! 布局引擎
//! 
//! 负责计算内容在页面上的具体位置和分页逻辑

use crate::document::{DocumentModel, ContentBlock, LayoutMetrics};

/// 页面配置
/// 
/// 定义页面的尺寸和边距设置
#[derive(Debug, Clone, PartialEq)]
pub struct PageConfig {
    /// 页面宽度（像素）
    pub width: f32,
    /// 页面高度（像素）
    pub height: f32,
    /// 上边距（像素）
    pub margin_top: f32,
    /// 下边距（像素）
    pub margin_bottom: f32,
    /// 左边距（像素）
    pub margin_left: f32,
    /// 右边距（像素）
    pub margin_right: f32,
}

impl PageConfig {
    /// 获取内容区域宽度
    /// 
    /// # Returns
    /// 
    /// 返回去除左右边距后的内容区域宽度
    pub fn content_width(&self) -> f32 {
        self.width - self.margin_left - self.margin_right
    }
    
    /// 获取内容区域高度
    /// 
    /// # Returns
    /// 
    /// 返回去除上下边距后的内容区域高度
    pub fn content_height(&self) -> f32 {
        self.height - self.margin_top - self.margin_bottom
    }
}

/// 页面结构
/// 
/// 表示一个页面，包含页面上的内容块和已使用的高度
#[derive(Debug, Clone, PartialEq)]
pub struct Page {
    /// 页面上的内容块列表
    pub blocks: Vec<ContentBlock>,
    /// 页面已使用的高度（像素）
    pub used_height: f32,
}

/// 布局引擎
/// 
/// 负责将文档模型转换为页面结构，处理分页逻辑
pub struct LayoutEngine {
    /// 页面配置
    page_config: PageConfig,
}

impl LayoutEngine {
    /// 创建一个新的布局引擎实例
    /// 
    /// # Arguments
    /// 
    /// * `page_config` - 页面配置参数
    /// 
    /// # Returns
    /// 
    /// 返回一个新的LayoutEngine实例
    pub fn new(page_config: PageConfig) -> Self {
        LayoutEngine { page_config }
    }

    /// 布局文档
    /// 
    /// 将文档模型按照页面配置进行布局，生成页面列表
    /// 
    /// # Arguments
    /// 
    /// * `document` - 需要布局的文档模型
    /// 
    /// # Returns
    /// 
    /// 返回布局后的页面列表
    pub fn layout_document(&self, document: &DocumentModel) -> Vec<Page> {
        let mut pages: Vec<Page> = Vec::new();
        let mut current_page = self.create_empty_page();
        
        // 遍历所有章节和内容块
        for chapter in &document.chapters {
            for block in &chapter.content {
                // 测量块的尺寸
                let block_metrics = self.measure_block(block);
                
                // 检查当前页是否能容纳这个块
                if self.can_fit_in_page(&block_metrics, &current_page) {
                    // 可以容纳，添加到当前页
                    current_page.used_height += block_metrics.height;
                    current_page.blocks.push(block.clone());
                } else {
                    // 无法容纳，保存当前页并创建新页
                    if !current_page.blocks.is_empty() {
                        pages.push(current_page);
                    }
                    current_page = self.create_empty_page();
                    
                    // 如果块太大无法适应空页面，需要拆分内容
                    if block_metrics.height > self.page_config.content_height() {
                        // 对于过大的块进行拆分处理
                        self.layout_large_block(block, &mut pages);
                    } else {
                        // 添加块到新页
                        current_page.used_height += block_metrics.height;
                        current_page.blocks.push(block.clone());
                    }
                }
            }
            
            // 在章节之间添加分页（如果当前页已经有内容）
            if !current_page.blocks.is_empty() && current_page.used_height > 0.0 {
                pages.push(current_page);
                current_page = self.create_empty_page();
            }
        }
        
        // 添加最后一页（如果有内容）
        if !current_page.blocks.is_empty() {
            pages.push(current_page);
        }
        
        pages
    }
    
    /// 处理过大的内容块
    /// 
    /// 当内容块太大无法适应单页时，将其拆分成多个较小的块
    /// 
    /// # Arguments
    /// 
    /// * `block` - 需要拆分的内容块
    /// * `pages` - 页面列表的可变引用
    fn layout_large_block(&self, block: &ContentBlock, pages: &mut Vec<Page>) {
        // 按行拆分内容
        let lines: Vec<&str> = block.content.lines().collect();
        let lines_per_page = (self.page_config.content_height() / (block.styles.font_size * 1.2)) as usize;
        
        // 分批处理行
        for chunk in lines.chunks(lines_per_page) {
            let mut new_page = self.create_empty_page();
            let chunk_content = chunk.join("\n");
            
            let new_block = ContentBlock {
                block_type: block.block_type.clone(),
                content: chunk_content,
                styles: block.styles.clone(),
                metrics: None,
            };
            
            let block_metrics = self.measure_block(&new_block);
            new_page.used_height = block_metrics.height;
            new_page.blocks.push(new_block);
            pages.push(new_page);
        }
    }
    
    /// 创建空页面
    /// 
    /// # Returns
    /// 
    /// 返回一个没有任何内容的新页面
    fn create_empty_page(&self) -> Page {
        Page {
            blocks: Vec::new(),
            used_height: 0.0,
        }
    }
    
    /// 测量块尺寸
    /// 
    /// 根据内容和样式计算内容块的尺寸
    /// 
    /// # Arguments
    /// 
    /// * `block` - 需要测量的内容块
    /// 
    /// # Returns
    /// 
    /// 返回内容块的尺寸信息
    pub fn measure_block(&self, block: &ContentBlock) -> LayoutMetrics {
        // 简化的测量逻辑 - 实际应该基于字体、文本内容等计算
        let lines = block.content.lines().count().max(1) as f32;
        let height = lines * block.styles.font_size * 1.2; // 行高1.2倍
        
        LayoutMetrics {
            width: self.page_config.content_width(),
            height,
        }
    }
    
    /// 检查块是否能放入页面
    /// 
    /// 根据页面剩余空间判断内容块是否能放入当前页面
    /// 
    /// # Arguments
    /// 
    /// * `metrics` - 内容块的尺寸信息
    /// * `page` - 当前页面
    /// 
    /// # Returns
    /// 
    /// 如果能放入返回true，否则返回false
    pub fn can_fit_in_page(&self, metrics: &LayoutMetrics, page: &Page) -> bool {
        page.used_height + metrics.height <= self.page_config.content_height()
    }
}