// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn typeset_document(content: &str) -> Result<String, String> {
    use typesetting_engine::{ParserEngine, LayoutEngine, PageConfig, Page};
    
    // 创建解析引擎
    let parser = ParserEngine::new();
    
    // 解析文档
    let document = parser.parse_txt(content);
    
    // 创建布局引擎
    let page_config = PageConfig {
        width: 400.0,
        height: 600.0,
        margin_top: 20.0,
        margin_bottom: 20.0,
        margin_left: 20.0,
        margin_right: 20.0,
    };
    let layout_engine = LayoutEngine::new(page_config);
    
    // 布局文档
    let pages: Vec<Page> = layout_engine.layout_document(&document);
    
    // 在Tauri应用中实现自己的渲染逻辑
    let rendered = render_pages_for_tauri(&pages);
    
    Ok(rendered)
}

/// 为Tauri应用实现的渲染函数
fn render_pages_for_tauri(pages: &[typesetting_engine::Page]) -> String {
    let mut result = String::new();
    for (i, page) in pages.iter().enumerate() {
        result.push_str(&format!("Page {}: \n", i + 1));
        result.push_str("--- Page Start ---\n");
        
        // 遍历页面中的所有内容块
        for block in &page.blocks {
            match block.block_type {
                typesetting_engine::ContentBlockType::Text => {
                    result.push_str(&format!("Text: {}\n", block.content));
                }
                typesetting_engine::ContentBlockType::Title => {
                    result.push_str(&format!("Title: {}\n", block.content));
                }
                typesetting_engine::ContentBlockType::Image => {
                    result.push_str(&format!("Image: {}\n", block.content));
                }
                typesetting_engine::ContentBlockType::Blank => {
                    result.push('\n');
                }
            }
        }
        
        result.push_str("--- Page End ---\n\n");
    }
    result
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, typeset_document])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}