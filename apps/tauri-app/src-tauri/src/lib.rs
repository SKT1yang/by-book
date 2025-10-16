// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn typeset_document(content: &str) -> Result<String, String> {
    use typesetting_engine::{ParserEngine, LayoutEngine, Renderer, PageConfig};
    
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
    let pages = layout_engine.layout_document(&document);
    
    // 创建渲染器
    let renderer = Renderer::new();
    
    // 渲染页面
    let rendered = renderer.render_pages(&pages);
    
    Ok(rendered)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, typeset_document])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
