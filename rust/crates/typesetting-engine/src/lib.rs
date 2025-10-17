//! 自研排版引擎
//! 
//! 这是一个用Rust编写的排版引擎，用于处理文档的解析、布局和渲染。
//! 
//! # 架构
//! 
//! 排版引擎采用分层架构设计：
//! 
//! - [document] - 文档模型定义
//! - [parser] - 解析引擎
//! - [layout] - 布局引擎
//! - [file_loader] - 文件加载器
//! - [renderer] - 渲染引擎

mod document;
mod parser;
mod layout;
mod file_loader;
// 移除渲染器模块，因为渲染应该由各个平台自己实现

// 测试模块
#[cfg(test)]
mod document_test;
#[cfg(test)]
mod parser_test;
#[cfg(test)]
mod layout_test;
// 移除渲染器测试模块

pub use document::*;
pub use parser::*;
pub use layout::*;
pub use file_loader::*;
// 移除渲染器的导出

/// 预导入模块
/// 
/// 提供常用的类型和错误处理
pub mod prelude {
    pub use anyhow::Result;
}