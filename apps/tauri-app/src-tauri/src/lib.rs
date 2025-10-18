// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn parse_document_chapters(content: &str) -> Result<Vec<String>, String> {
    use typesetting_engine::ParserEngine;
    
    // 创建解析引擎
    let parser = ParserEngine::new();
    
    // 解析文档
    let document = parser.parse_txt(content);
    
    // 提取章节标题
    let chapter_titles: Vec<String> = document.chapters
        .iter()
        .map(|chapter| chapter.title.clone())
        .collect();
    
    Ok(chapter_titles)
}

#[tauri::command]
fn typeset_document_with_chapter_info(content: &str) -> Result<(String, Vec<(String, usize)>), String> {
    use typesetting_engine::{ParserEngine, LayoutEngine, PageConfig, Page};
    
    // 创建解析引擎
    let parser = ParserEngine::new();
    
    // 解析文档
    let document = parser.parse_txt(content);
    
    // 创建布局引擎
    let page_config = PageConfig {
        width: 800.0,  // 增大页面宽度
        height: 1000.0, // 增大页面高度
        margin_top: 40.0,
        margin_bottom: 40.0,
        margin_left: 40.0,
        margin_right: 40.0,
    };
    let layout_engine = LayoutEngine::new(page_config);
    
    // 布局文档
    let pages: Vec<Page> = layout_engine.layout_document(&document);
    
    // 简化处理：平均分配页面给章节
    let mut chapter_page_mapping: Vec<(String, usize)> = Vec::new();
    let pages_per_chapter = if document.chapters.is_empty() {
        0
    } else {
        pages.len() / document.chapters.len()
    };
    
    for (index, chapter) in document.chapters.iter().enumerate() {
        let start_page = index * pages_per_chapter;
        chapter_page_mapping.push((chapter.title.clone(), start_page));
    }
    
    // 在Tauri应用中实现自己的渲染逻辑
    let rendered = render_pages_for_tauri(&pages);
    
    Ok((rendered, chapter_page_mapping))
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
        width: 800.0,  // 增大页面宽度
        height: 1000.0, // 增大页面高度
        margin_top: 40.0,
        margin_bottom: 40.0,
        margin_left: 40.0,
        margin_right: 40.0,
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
        result.push_str(&format!("Page {}\n", i + 1));
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

/// 初始化应用数据目录结构
fn initialize_app_directories(app_handle: &tauri::AppHandle) -> Result<(), String> {
    use std::fs;
    use tauri::Manager;
    
    // 获取应用数据目录
    let app_dir = app_handle.path().app_data_dir()
        .map_err(|e| format!("无法获取应用数据目录: {}", e))?;
    
    // 定义应用所需的各种子目录
    let directories = vec![
        "documents",     // 用户上传的文档
        "cache",         // 缓存数据
        "cache/thumbnails", // 缓存的缩略图
        "cache/chapters",   // 缓存的章节内容
        "cache/metadata",   // 缓存的元数据
        "logs",          // 日志文件
        "settings",      // 用户设置
        "downloads",     // 下载的书籍
        "downloads/temp",   // 下载临时文件
        "downloads/completed", // 已完成的下载
        "temp",          // 临时文件
        "backups",       // 备份数据
        "backups/settings",    // 设置备份
        "backups/reading_progress" // 阅读进度备份
    ];
    
    // 创建所有必需的目录
    for dir in directories {
        let full_path = app_dir.join(dir);
        fs::create_dir_all(&full_path)
            .map_err(|e| format!("无法创建目录 '{}': {}", dir, e))?;
    }
    
    Ok(())
}

#[tauri::command]
fn save_document(app_handle: tauri::AppHandle, filename: &str, content: &str) -> Result<String, String> {
    use std::fs;
    use tauri::Manager;
    
    // 初始化应用目录结构
    initialize_app_directories(&app_handle)?;
    
    // 获取应用数据目录
    let app_dir = app_handle.path().app_data_dir()
        .map_err(|e| format!("无法获取应用数据目录: {}", e))?;
    
    // 创建documents目录
    let docs_dir = app_dir.join("documents");
    
    // 保存文件到documents目录，确保使用UTF-8编码
    let file_path = docs_dir.join(filename);
    println!("准备保存文件到: {:?}", file_path);
    
    // 确保内容以UTF-8编码保存
    match fs::write(&file_path, content.as_bytes()) {
        Ok(_) => {
            let message = format!("文件已保存到: {:?}", file_path);
            println!("{}", message);
            
            // 验证文件是否真的被保存
            if file_path.exists() {
                println!("验证: 文件确实存在于 {:?}", file_path);
            } else {
                println!("警告: 文件未在 {:?} 找到", file_path);
            }
            
            Ok(message)
        }
        Err(e) => {
            let error_msg = format!("无法保存文件 '{:?}': {}", file_path, e);
            println!("{}", error_msg);
            Err(error_msg)
        }
    }
}

#[tauri::command]
fn load_document(app_handle: tauri::AppHandle, filename: &str) -> Result<String, String> {
    use tauri::Manager;
    use typesetting_engine::FileLoader;
    
    // 获取应用数据目录
    let app_dir = app_handle.path().app_data_dir()
        .map_err(|e| format!("无法获取应用数据目录: {}", e))?;
    
    // 构建文件路径
    let file_path = app_dir.join("documents").join(filename);
    println!("正在尝试读取文件: {:?}", file_path);
    
    // 使用排版引擎的文件加载器加载文件
    let loader = FileLoader::new();
    match loader.load_text_file(file_path.to_str().unwrap_or("")) {
        Ok(content) => {
            println!("文件读取成功");
            Ok(content)
        }
        Err(e) => {
            Err(format!("无法读取文件: {}", e))
        }
    }
}

#[tauri::command]
fn delete_document(app_handle: tauri::AppHandle, filename: &str, delete_file: bool) -> Result<String, String> {
    use std::fs;
    use tauri::Manager;
    
    // 获取应用数据目录
    let app_dir = app_handle.path().app_data_dir()
        .map_err(|e| format!("无法获取应用数据目录: {}", e))?;
    
    // 构建文件路径
    let file_path = app_dir.join("documents").join(filename);
    println!("正在尝试删除文件: {:?}", file_path);
    
    // 从书架移除（无论是否删除本地文件）
    // 这里我们只是从UI上移除，实际上不需要做任何操作
    
    // 如果需要删除本地文件
    if delete_file {
        match fs::remove_file(&file_path) {
            Ok(_) => {
                println!("文件删除成功: {:?}", file_path);
                Ok(format!("文件已从书架移除并删除本地文件: {}", filename))
            }
            Err(e) => {
                println!("文件删除失败: {:?}", e);
                Err(format!("无法删除文件 '{}': {}", filename, e))
            }
        }
    } else {
        // 只是从书架移除，不删除本地文件
        println!("文件已从书架移除，但保留本地文件: {:?}", file_path);
        Ok(format!("文件已从书架移除，但保留本地文件: {}", filename))
    }
}

#[tauri::command]
fn list_documents(app_handle: tauri::AppHandle) -> Result<Vec<String>, String> {
    use std::fs;
    use tauri::Manager;
    
    // 初始化应用目录结构
    initialize_app_directories(&app_handle)?;
    
    // 获取应用数据目录
    let app_dir = app_handle.path().app_data_dir()
        .map_err(|e| format!("无法获取应用数据目录: {}", e))?;
    
    // 获取documents目录
    let docs_dir = app_dir.join("documents");
    
    println!("应用数据目录: {:?}", app_dir);
    println!("Documents目录路径: {:?}", docs_dir);
    
    // 如果目录不存在或无法读取，返回空列表
    if !docs_dir.exists() {
        println!("Documents目录不存在: {:?}", docs_dir);
        return Ok(vec![]);
    }
    
    println!("正在读取目录: {:?}", docs_dir);
    
    // 读取目录中的文件
    let entries = fs::read_dir(&docs_dir);
    if let Err(e) = &entries {
        println!("无法读取目录 {:?}: {}", docs_dir, e);
        return Err(format!("无法读取目录: {}", e));
    }
    
    let entries = entries.unwrap();
    let mut files = Vec::new();
    for entry in entries {
        match entry {
            Ok(entry) => {
                if let Some(file_name) = entry.file_name().to_str() {
                    println!("找到文件: {}", file_name);
                    files.push(file_name.to_string());
                }
            }
            Err(e) => {
                println!("读取目录项时出错: {}", e);
            }
        }
    }
    
    println!("总共找到 {} 个文件", files.len());
    Ok(files)
}

#[tauri::command]
fn get_app_data_structure(app_handle: tauri::AppHandle) -> Result<Vec<String>, String> {
    use std::fs;
    use tauri::Manager;
    
    // 初始化应用目录结构
    initialize_app_directories(&app_handle)?;
    
    // 获取应用数据目录
    let app_dir = app_handle.path().app_data_dir()
        .map_err(|e| format!("无法获取应用数据目录: {}", e))?;
    
    // 定义主要目录
    let main_dirs = vec!["documents", "cache", "logs", "settings", "downloads", "temp", "backups"];
    
    let mut structure = Vec::new();
    structure.push(format!("应用数据根目录: {:?}", app_dir));
    
    // 检查每个主要目录
    for dir in main_dirs {
        let full_path = app_dir.join(dir);
        if full_path.exists() {
            structure.push(format!("✓ {}", dir));
            
            // 尝试读取子目录
            if let Ok(entries) = fs::read_dir(&full_path) {
                for entry in entries {
                    if let Ok(entry) = entry {
                        if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
                            if let Some(name) = entry.file_name().to_str() {
                                structure.push(format!("  ├── {}", name));
                            }
                        }
                    }
                }
            }
        } else {
            structure.push(format!("✗ {} (不存在)", dir));
        }
    }
    
    Ok(structure)
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
            typeset_document_with_chapter_info,
            parse_document_chapters,
            save_document,
            load_document,
            delete_document,
            list_documents,
            get_app_data_structure
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}