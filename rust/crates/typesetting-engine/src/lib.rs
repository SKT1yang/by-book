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
//! - [renderer] - 渲染引擎

mod document;
mod parser;
mod layout;
mod renderer;

// 测试模块
#[cfg(test)]
mod document_test;
#[cfg(test)]
mod parser_test;
#[cfg(test)]
mod layout_test;
#[cfg(test)]
mod renderer_test;

pub use document::*;
pub use parser::*;
pub use layout::*;
pub use renderer::*;

/// 预导入模块
/// 
/// 提供常用的类型和错误处理
pub mod prelude {
    pub use anyhow::Result;
}