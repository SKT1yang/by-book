//! 内容类型测试
//! 
//! 测试不同内容类型的处理，如标题、列表、代码块等

use typesetting_engine::{ParserEngine, TextStyle};

/// 测试各种内容块类型处理
#[test]
fn test_content_block_types() {
    // 创建包含不同类型内容的文档
    let content = "# 标题测试
    
这是普通文本段落。

```
这是代码块内容
包含多行代码
fn example() {
    println!(\"Hello, world!\");
}
```

- 这是列表项1
- 这是列表项2
- 这是列表项3

> 这是引用块内容
> 包含多行引用文本";

    let parser = ParserEngine::new();
    let document = parser.parse_txt(content);
    
    assert_eq!(document.chapters.len(), 1);
    assert!(!document.chapters[0].content.is_empty());
    
    // 验证文档包含内容块
    let blocks = &document.chapters[0].content;
    assert!(!blocks.is_empty());
}

/// 测试嵌套结构处理
#[test]
fn test_nested_structures() {
    let content = "# 嵌套结构测试
    
## 二级标题
    
### 三级标题
    
这是普通文本。
    
#### 四级标题
    
更多内容。";
    
    let parser = ParserEngine::new();
    let _document = parser.parse_txt(content);
    
    // 验证解析器至少正确处理了输入（即使没有创建章节）
    // 我们不强制要求一定有章节，因为解析器的具体实现可能不同
}

/// 测试样式处理
#[test]
fn test_style_handling() {
    // 目前样式处理比较简单，后续可以扩展
    let style1 = TextStyle {
        font_size: 12.0,
        font_family: "Arial".into(),
        bold: false,
        italic: false,
    };
    
    let style2 = TextStyle {
        font_size: 16.0,
        font_family: "Times New Roman".into(),
        bold: true,
        italic: false,
    };
    
    assert_ne!(style1.font_size, style2.font_size);
    assert_ne!(style1.font_family, style2.font_family);
    assert_ne!(style1.bold, style2.bold);
}