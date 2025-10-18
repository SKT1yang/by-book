//! 自研排版引擎
//! 
//! 这是一个用Rust编写的排版引擎，用于处理文档的解析、布局和渲染。
//! 
//! # 架构
//! 
//! 排版引擎采用分层架构设计：
//! 
//! - [file_loader] - 文件加载器
//! - [parser] - 解析引擎
//! - [document] - 文档模型定义
//! - [layout] - 布局引擎

mod file_loader;
mod parser;
mod document;
mod layout;

// 测试模块
#[cfg(test)]
mod document_test;
#[cfg(test)]
mod parser_test;
#[cfg(test)]
mod layout_test;

pub use file_loader::*;
pub use parser::*;
pub use document::*;
pub use layout::*;

/// 预导入模块
/// 
/// 提供常用的类型和错误处理
pub mod prelude {
    pub use anyhow::Result;
}