# 排版引擎项目待办事项和改进计划

## 已完成功能

- [x] 实现基础文档模型结构
- [x] 实现解析引擎
- [x] 实现布局引擎核心功能
- [x] 实现渲染引擎
- [x] 添加全面的测试覆盖（单元测试、集成测试、性能测试）
- [x] 添加详细的代码注释
- [x] 修复所有编译警告和clippy建议

## 待开发功能

### 核心功能改进

- [ ] 实现真实的文本测量系统
  - [ ] 集成字体处理库（如rusttype或ab_glyph）
  - [ ] 实现基于实际字体的文本尺寸计算
  - [ ] 支持不同字体族对尺寸的影响
  - [ ] 处理粗体、斜体等字体样式对尺寸的影响

- [ ] 实现基于宽度的文本换行算法
  - [ ] 根据容器宽度智能换行
  - [ ] 支持连字符和断字处理
  - [ ] 处理不可分割的长单词

- [ ] 支持多种文本对齐方式
  - [ ] 左对齐
  - [ ] 右对齐
  - [ ] 居中对齐
  - [ ] 两端对齐

- [ ] 实现测量结果缓存机制
  - [ ] 添加测量缓存以提高性能
  - [ ] 实现增量测量避免重复计算

### 高级排版功能

- [ ] 支持图片内容处理
  - [ ] 图片尺寸测量
  - [ ] 图片与文本的混合排版

- [ ] 支持表格内容处理
  - [ ] 表格布局计算
  - [ ] 表格与文本的混合排版

- [ ] 支持复杂文档结构
  - [ ] 列表处理（有序、无序）
  - [ ] 引用块处理
  - [ ] 代码块处理

- [ ] 实现分页优化算法
  - [ ] 孤行控制（widow/orphan control）
  - [ ] 标题与内容保持在同一页面
  - [ ] 图文混排的分页优化

### 性能优化

- [ ] 实现增量布局计算
  - [ ] 只重新计算受影响的部分
  - [ ] 优化大文档处理性能

- [ ] 实现虚拟分页
  - [ ] 按需计算和渲染页面
  - [ ] 支持大型文档的流畅浏览

### 跨平台支持

- [ ] 抽象文本测量接口
  - [ ] 定义平台无关的测量接口
  - [ ] 为不同平台提供特定实现

- [ ] 支持Web平台
  - [ ] 实现基于Canvas的文本测量
  - [ ] 实现Web端渲染适配

- [ ] 支持移动端
  - [ ] 实现移动端原生文本测量
  - [ ] 优化移动端渲染性能

## 改进点详细说明

### 当前文本测量方法的缺点

1. **过于简化的测量逻辑**
   - 只考虑行数和字体大小
   - 忽略不同字体族对宽度的影响
   - 没有考虑字符宽度差异

2. **固定宽度测量**
   - 所有内容块使用相同宽度
   - 无法支持不同宽度的内容块

3. **缺乏精确的文本布局计算**
   - 没有字符级别的测量
   - 没有处理复杂文本布局

### 优先级排序

#### 高优先级
1. 实现真实的文本测量系统
2. 实现基于宽度的文本换行算法
3. 实现测量结果缓存机制

#### 中优先级
1. 支持多种文本对齐方式
2. 支持图片内容处理
3. 实现分页优化算法

#### 低优先级
1. 支持表格内容处理
2. 跨平台支持
3. 高级排版功能

## 进度跟踪

### 2025年10月
- [x] 完成基础排版引擎架构设计与实现
- [x] 完成全面测试覆盖
- [x] 完成代码质量优化（消除警告和clippy建议）

### 2025年11月计划
- [ ] 实现真实的文本测量系统（进行中）
- [ ] 实现基于宽度的文本换行算法
- [ ] 添加测量结果缓存机制

### 2025年12月计划
- [ ] 支持多种文本对齐方式
- [ ] 实现图片内容处理
- [ ] 优化分页算法

## 技术选型建议

### 字体处理库
- `rusttype` - 纯Rust字体处理库
- `ab_glyph` - 现代化的Rust字体光栅化库

### 性能优化
- 使用`HashMap`实现测量缓存
- 考虑使用`DashMap`处理并发场景

## 测试计划

### 单元测试
- 文本测量准确性测试
- 换行算法正确性测试
- 对齐方式测试

### 性能测试
- 大文档处理性能测试
- 缓存机制效果测试
- 内存使用情况监控

## 设计文档

### 文本测量系统架构

```rust
/// 抽象文本测量接口
trait TextMeasurer {
    /// 测量文本尺寸
    fn measure_text(&self, text: &str, style: &TextStyle) -> LayoutMetrics;
    
    /// 测量内容块尺寸
    fn measure_block(&self, block: &ContentBlock) -> LayoutMetrics;
}

/// 基于系统字体的真实文本测量实现
struct SystemTextMeasurer {
    // 字体缓存
    font_cache: HashMap<String, Font>,
    // 测量缓存
    measurement_cache: HashMap<TextMeasurementKey, LayoutMetrics>,
}

impl TextMeasurer for SystemTextMeasurer {
    fn measure_text(&self, text: &str, style: &TextStyle) -> LayoutMetrics {
        // 实际的文本测量逻辑
        // 1. 获取字体信息
        // 2. 计算每个字符的宽度
        // 3. 计算总宽度和高度
        // 4. 考虑行高、字间距等参数
    }
}
```

### 换行算法设计

```rust
/// 文本换行器
struct TextWrapper {
    /// 容器宽度
    container_width: f32,
    /// 文本测量器
    measurer: Box<dyn TextMeasurer>,
}

impl TextWrapper {
    /// 根据宽度对文本进行换行
    fn wrap_text(&self, text: &str, style: &TextStyle) -> Vec<String> {
        // 实现智能换行算法
        // 1. 按空格分割单词
        // 2. 逐个添加单词并测量行宽
        // 3. 超出宽度时换行
        // 4. 处理长单词的特殊换行
    }
}
```

### 缓存机制设计

```rust
/// 测量缓存键
#[derive(Hash, PartialEq, Eq)]
struct TextMeasurementKey {
    text: String,
    font_family: String,
    font_size: f32,
    bold: bool,
    italic: bool,
}

/// 带缓存的文本测量器
struct CachedTextMeasurer {
    inner: SystemTextMeasurer,
    cache: HashMap<TextMeasurementKey, LayoutMetrics>,
}

impl TextMeasurer for CachedTextMeasurer {
    fn measure_text(&self, text: &str, style: &TextStyle) -> LayoutMetrics {
        let key = TextMeasurementKey {
            text: text.to_string(),
            font_family: style.font_family.clone(),
            font_size: style.font_size,
            bold: style.bold,
            italic: style.italic,
        };
        
        // 先尝试从缓存获取
        if let Some(metrics) = self.cache.get(&key) {
            return metrics.clone();
        }
        
        // 缓存未命中，进行实际测量
        let metrics = self.inner.measure_text(text, style);
        self.cache.insert(key, metrics.clone());
        metrics
    }
}
```