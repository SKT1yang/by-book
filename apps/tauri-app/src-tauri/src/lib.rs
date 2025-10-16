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

#[tauri::command]
fn save_document(app_handle: tauri::AppHandle, filename: &str, content: &str) -> Result<String, String> {
    use std::fs;
    use tauri::Manager;
    
    // 获取应用数据目录
    let app_dir = app_handle.path().app_data_dir()
        .map_err(|e| format!("无法获取应用数据目录: {}", e))?;
    
    // 创建documents目录
    let docs_dir = app_dir.join("documents");
    fs::create_dir_all(&docs_dir)
        .map_err(|e| format!("无法创建documents目录: {}", e))?;
    
    // 保存文件，确保使用UTF-8编码
    let file_path = docs_dir.join(filename);
    fs::write(&file_path, content.as_bytes())
        .map_err(|e| format!("无法保存文件: {}", e))?;
    
    Ok(format!("文件已保存到: {:?}", file_path))
}

#[tauri::command]
fn load_document(app_handle: tauri::AppHandle, filename: &str) -> Result<String, String> {
    use std::fs;
    use tauri::Manager;
    
    // 获取应用数据目录
    let app_dir = app_handle.path().app_data_dir()
        .map_err(|e| format!("无法获取应用数据目录: {}", e))?;
    
    // 构建文件路径
    let file_path = app_dir.join("documents").join(filename);
    let content = fs::read_to_string(&file_path)
        .map_err(|e| format!("无法读取文件: {}", e))?;
    
    Ok(content)
}

#[tauri::command]
fn list_documents(app_handle: tauri::AppHandle) -> Result<Vec<String>, String> {
    use std::fs;
    use tauri::Manager;
    
    // 获取应用数据目录
    let app_dir = app_handle.path().app_data_dir()
        .map_err(|e| format!("无法获取应用数据目录: {}", e))?;
    
    // 获取documents目录
    let docs_dir = app_dir.join("documents");
    
    // 确保目录存在
    if !docs_dir.exists() {
        return Ok(vec![]);
    }
    
    // 读取目录中的文件
    let entries = fs::read_dir(&docs_dir)
        .map_err(|e| format!("无法读取目录: {}", e))?;
    
    let mut files = Vec::new();
    for entry in entries {
        if let Ok(entry) = entry {
            if let Some(file_name) = entry.file_name().to_str() {
                files.push(file_name.to_string());
            }
        }
    }
    
    Ok(files)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            greet, 
            typeset_document,
            save_document,
            load_document,
            list_documents
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}