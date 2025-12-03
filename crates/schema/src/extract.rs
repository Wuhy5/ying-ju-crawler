//! 字段提取流程 (Field Extraction Pipeline)
//!
//! 定义如何从响应数据中提取单个字段值的步骤流程。
//! 设计理念：**全流程化，每个操作都是显式的步骤**。
//!
//! # 设计原则
//!
//! 1. **全流程化**：所有提取都通过显式步骤定义
//! 2. **原子化步骤**：每个步骤只执行一个明确的操作
//! 3. **格式驱动**：HTML/JSON/XML 各有专属的选择步骤
//! 4. **人类可读**：配置文件清晰易懂，便于维护
//!
//! # 快速示例
//!
//! ```toml
//! # 单步骤提取
//! title.steps = [{ css = ".title" }]
//!
//! # 多步骤提取
//! cover.steps = [
//!     { css = "img" },
//!     { attr = "src" },
//!     { filter = "absolute_url" }
//! ]
//!
//! # 带回退
//! author.steps = [{ css = ".author" }]
//! author.fallback = [
//!     [{ css = ".writer" }],
//!     [{ const = "佚名" }]
//! ]
//! ```
//!
//! # 步骤类型
//!
//! ## 选择步骤（从文档提取数据）
//!
//! | 步骤 | 格式 | 说明 |
//! |------|------|------|
//! | `css` | HTML | CSS 选择器 |
//! | `json` | JSON | JSONPath 表达式 |
//! | `xpath` | XML/HTML | XPath 表达式 |
//! | `regex` | 文本 | 正则表达式匹配 |
//!
//! ## 过滤步骤（转换数据）
//!
//! | 步骤 | 说明 |
//! |------|------|
//! | `filter` | 应用过滤器（trim, lower, to_int 等） |
//! | `attr` | 提取元素属性 |
//! | `index` | 索引或切片操作 |
//!
//! ## 特殊步骤
//!
//! | 步骤 | 说明 |
//! |------|------|
//! | `const` | 常量值 |
//! | `var` | 上下文变量 |
//! | `script` | 自定义脚本 |

use crate::script::Script;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// ============================================================================
// 核心提取器
// ============================================================================

/// 字段提取器 (FieldExtractor)
///
/// 定义如何从响应数据中提取单个字段的值。
/// 通过步骤列表定义提取流程。
///
/// # 示例
///
/// ```toml
/// # 单步骤
/// title.steps = [{ css = ".title" }]
///
/// # 多步骤
/// cover.steps = [
///     { css = "img" },
///     { attr = "src" },
///     { filter = "absolute_url" }
/// ]
///
/// # 带回退和默认值
/// author.steps = [{ css = ".author" }]
/// author.fallback = [
///     [{ css = ".writer" }],
///     [{ css = ".creator" }]
/// ]
/// author.default = "佚名"
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct FieldExtractor {
    /// 提取步骤列表
    ///
    /// 按顺序执行，前一步骤的输出作为后一步骤的输入
    pub steps: Vec<ExtractStep>,

    /// 回退步骤链
    ///
    /// 当主步骤提取失败（结果为空或 null）时，按顺序尝试这些备选流程
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fallback: Option<Vec<Vec<ExtractStep>>>,

    /// 默认值
    ///
    /// 所有提取（包括回退）都失败时使用此值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<serde_json::Value>,

    /// 是否允许空值
    #[serde(default)]
    pub nullable: bool,
}

// ============================================================================
// 提取步骤 (ExtractStep)
// ============================================================================

/// 提取步骤 (ExtractStep)
///
/// 单个原子化操作。步骤类型：
/// - **选择步骤**：css, json, regex
/// - **过滤步骤**：filter, attr, index
/// - **特殊步骤**：const, var, script
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExtractStep {
    // ========== 选择步骤 ==========
    /// CSS 选择器（HTML）
    Css(SelectorStep),

    /// JSONPath 表达式（JSON）
    Json(SelectorStep),

    /// 正则表达式（文本）
    Regex(RegexStep),

    // ========== 过滤步骤 ==========
    /// 过滤器管道
    Filter(FilterStep),

    /// 属性提取
    Attr(String),

    /// 索引/切片
    Index(IndexStep),

    // ========== 特殊步骤 ==========
    /// 常量值
    Const(serde_json::Value),

    /// 上下文变量
    Var(String),

    /// 脚本调用
    Script(Script),
}

// ============================================================================
// 步骤配置类型
// ============================================================================

/// 选择器步骤（CSS/JSONPath/XPath 通用）
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum SelectorStep {
    /// 简单选择器字符串
    Simple(String),
    /// 带配置的选择器
    WithOptions {
        /// 选择器表达式
        selector: String,
        /// 是否选择所有匹配（默认 false）
        #[serde(default)]
        all: bool,
    },
}

/// 正则表达式步骤
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum RegexStep {
    /// 简单正则（默认取第1组）
    Simple(String),
    /// 带配置的正则
    WithOptions {
        /// 正则表达式模式
        pattern: String,
        /// 捕获组索引（默认 1）
        #[serde(default = "default_regex_group")]
        group: usize,
        /// 是否全局匹配
        #[serde(default)]
        global: bool,
    },
}

fn default_regex_group() -> usize {
    1
}

/// 过滤器步骤
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum FilterStep {
    /// 管道字符串：`"trim | lower | replace(a, b)"`
    Pipeline(String),
    /// 过滤器数组（复杂参数场景）
    List(Vec<FilterConfig>),
}

/// 索引/切片步骤
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum IndexStep {
    /// 单个索引
    Single(i32),
    /// 切片表达式 "start:end" 或 "start:end:step"
    Slice(String),
}

/// 过滤器配置（结构化形式）
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct FilterConfig {
    /// 过滤器名称
    pub name: String,
    /// 过滤器参数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<serde_json::Value>>,
}

// ============================================================================
// 内置过滤器
// ============================================================================

/// 内置过滤器枚举
///
/// 用于 JSON Schema 生成和文档，运行时通过字符串解析
///
/// # 字符串处理
/// - `trim` - 去首尾空白
/// - `lower` / `upper` - 大小写转换
/// - `replace(from, to)` - 文本替换
/// - `strip_html` - 移除 HTML 标签
/// - `split(sep)` / `join(sep)` - 分割/连接
///
/// # 类型转换
/// - `to_int` / `to_float` / `to_string` / `to_bool`
/// - `from_json` / `to_json`
///
/// # URL 处理
/// - `absolute_url` - 转绝对 URL
/// - `url_encode` / `url_decode`
/// - `extract_domain` / `query_param(name)`
///
/// # 数组处理
/// - `first` / `last` / `nth(n)`
/// - `slice(start, end)` / `reverse` / `unique`
///
/// # 条件处理
/// - `default(value)` - 默认值
/// - `if_empty(value)` - 空值替换
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Filter {
    // === 字符串处理 ===
    Trim,
    TrimStart,
    TrimEnd,
    Lower,
    Upper,
    Capitalize,
    StripHtml,
    CollapseWhitespace,
    Replace,
    RegexReplace,
    Split,
    Join,
    Substring,
    Reverse,

    // === 类型转换 ===
    ToInt,
    ToFloat,
    ToString,
    ToBool,
    ToJson,
    FromJson,

    // === 数值处理 ===
    Round,
    Floor,
    Ceil,
    Abs,
    Add,
    Sub,
    Mul,
    Div,
    Clamp,

    // === URL 处理 ===
    AbsoluteUrl,
    UrlEncode,
    UrlDecode,
    ExtractDomain,
    ExtractPath,
    QueryParam,

    // === 编码处理 ===
    Base64Encode,
    Base64Decode,
    HtmlEncode,
    HtmlDecode,
    Md5,

    // === 正则处理 ===
    RegexExtract,
    RegexMatch,
    RegexFindAll,

    // === 条件处理 ===
    Default,
    IfEmpty,
    IfNull,
    MapValue,

    // === 数组处理 ===
    First,
    Last,
    Nth,
    Slice,
    Unique,
    Sort,
    Flatten,
    Length,
}
