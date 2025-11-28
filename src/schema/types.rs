//! 辅助枚举类型与标识符
//!
//! 本模块提供类型安全的标识符和枚举定义

use regex::Regex;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;

use crate::error::CrawlerError;

/// 标识符验证正则
static IDENTIFIER_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*$").unwrap());

/// 标识符 (Identifier)
/// 用于校验变量名、组件名、流程名等。
/// 必须由字母、数字和下划线组成，且不能以数字开头。
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Hash)]
#[serde(try_from = "String", into = "String")]
pub struct Identifier(#[schemars(pattern("^[a-zA-Z_][a-zA-Z0-9_]*$"))] String);

impl Identifier {
    /// 创建新标识符（带验证）
    pub fn new(s: impl Into<String>) -> Result<Self, CrawlerError> {
        let s = s.into();
        if Self::is_valid(&s) {
            Ok(Self(s))
        } else {
            Err(CrawlerError::InvalidIdentifier {
                identifier: s.clone(),
                reason: "标识符必须以字母或下划线开头，只能包含字母、数字和下划线".to_string(),
            })
        }
    }

    /// 创建新标识符（不验证，用于已知有效的情况）
    pub fn new_unchecked(s: impl Into<String>) -> Self {
        Self(s.into())
    }

    /// 验证标识符是否有效
    pub fn is_valid(s: &str) -> bool {
        !s.is_empty() && IDENTIFIER_PATTERN.is_match(s)
    }

    /// 获取内部字符串
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// 转换为String
    pub fn into_string(self) -> String {
        self.0
    }
}

impl TryFrom<String> for Identifier {
    type Error = CrawlerError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::new(s)
    }
}

impl From<Identifier> for String {
    fn from(id: Identifier) -> Self {
        id.0
    }
}

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for Identifier {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

// --- 筛选器相关 ---

/// ## 筛选器组 (FilterGroup)
/// 代表UI上一组相关的筛选选项，如"地区"、"年份"。
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct FilterGroup {
    /// 筛选器组的显示名称，如 "按类型"。
    pub name: String,
    /// 此筛选器组在URL模板中对应的键 (`key`)。
    /// 例如，如果 `key` 是 `"cate_id"`，则UI会将用户选择的值替换到URL的 `{{cate_id}}` 位置。
    #[schemars(with = "Identifier")]
    pub key: String,
    /// 是否允许多选。
    #[serde(default)]
    pub multiselect: bool,
    /// 此筛选器组下所有可用的选项。
    pub options: Vec<FilterOption>,
}

/// ## 筛选器选项 (FilterOption)
/// 代表一个具体的筛选选项，如"电影"或"2023年"。
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct FilterOption {
    /// 选项的显示名称，如 "美国"。
    pub name: String,
    /// 选项的值，将用于替换URL模板中对应的 `key`。
    /// 例如，如果 `key` 是 `"area"`，此 `value` 可能是 `"USA"`。
    pub value: String,
}

// --- 媒体类型 ---

/// 用于指定规则适用的媒体内容类型。
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Hash, Copy)]
#[serde(rename_all = "lowercase")]
#[derive(Default)]
pub enum MediaType {
    /// 视频类型，如电影、电视剧等。
    #[default]
    Video,
    /// 音频类型，如音乐、播客等。
    Audio,
    /// 书籍类型，如电子书、小说等。
    Book,
    /// 漫画类型，如漫画、图画书等。
    Manga,
}

impl MediaType {
    /// 获取显示名称
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Video => "视频",
            Self::Audio => "音频",
            Self::Book => "书籍",
            Self::Manga => "漫画",
        }
    }
}

// --- HTTP方法 ---

/// HTTP 请求方法 (HttpMethod)
/// 用于指定网络请求的 HTTP 方法。
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Hash, Copy, Default)]
#[serde(rename_all = "UPPERCASE")]
pub enum HttpMethod {
    /// GET 请求，通常用于获取数据。
    #[default]
    Get,
    /// POST 请求，通常用于提交数据。
    Post,
    /// PUT 请求，通常用于更新数据。
    Put,
    /// DELETE 请求，通常用于删除数据。
    Delete,
    /// HEAD 请求，类似于 GET，但只获取响应头。
    Head,
    /// OPTIONS 请求，获取服务器支持的HTTP方法。
    Options,
    /// PATCH 请求，用于部分更新数据。
    Patch,
}

impl HttpMethod {
    /// 是否需要请求体
    pub fn has_body(&self) -> bool {
        matches!(self, Self::Post | Self::Put | Self::Patch)
    }

    /// 获取方法名称
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Get => "GET",
            Self::Post => "POST",
            Self::Put => "PUT",
            Self::Delete => "DELETE",
            Self::Head => "HEAD",
            Self::Options => "OPTIONS",
            Self::Patch => "PATCH",
        }
    }
}

// --- 脚本引擎 ---

/// 脚本引擎类型 (ScriptEngine)
/// 用于指定脚本执行环境的类型。
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Hash, Copy, Default)]
#[serde(rename_all = "lowercase")]
pub enum ScriptEngine {
    /// Rhai 脚本引擎（默认）。
    #[default]
    Rhai,
    /// JavaScript 脚本引擎。
    JavaScript,
    /// Python 脚本引擎。
    Python,
    /// Lua 脚本引擎。
    Lua,
}

impl ScriptEngine {
    /// 获取文件扩展名
    pub fn file_extension(&self) -> &'static str {
        match self {
            Self::Rhai => "rhai",
            Self::JavaScript => "js",
            Self::Python => "py",
            Self::Lua => "lua",
        }
    }
}

// --- 缓存相关 ---

/// 缓存后端 (CacheBackend)
/// 用于指定缓存存储的后端类型。
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Hash, Copy, Default)]
#[serde(rename_all = "lowercase")]
pub enum CacheBackend {
    /// 内存缓存，适合临时数据存储（默认）。
    #[default]
    Memory,
    /// SQLite 数据库存储，适合持久化缓存。
    Sqlite,
}

/// 缓存作用域 (CacheScope)
/// 用于指定缓存数据的生命周期范围。
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Hash, Copy, Default)]
#[serde(rename_all = "lowercase")]
pub enum CacheScope {
    /// 流程级缓存，在单个流程执行期间有效（默认）。
    #[default]
    Flow,
    /// 规则级缓存，在整个规则生命周期内有效，规则删除时缓存也删除。
    Rule,
}

// --- 选择器提取方式 ---

/// 选择器提取方式
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub enum ExtractType {
    /// 提取文本内容
    #[default]
    Text,
    /// 提取内部HTML
    Html,
    /// 提取完整HTML
    OuterHtml,
    /// 提取 `href` 属性
    #[serde(rename = "attr:href")]
    AttrHref,
    /// 提取 `src` 属性
    #[serde(rename = "attr:src")]
    AttrSrc,
    /// 提取 `data-src` 属性
    #[serde(rename = "attr:data-src")]
    AttrDataSrc,
    /// 提取 `alt` 属性
    #[serde(rename = "attr:alt")]
    AttrAlt,
    /// 提取 `title` 属性
    #[serde(rename = "attr:title")]
    AttrTitle,
    /// 提取 `class` 属性
    #[serde(rename = "attr:class")]
    AttrClass,
    /// 提取 `id` 属性
    #[serde(rename = "attr:id")]
    AttrId,
    /// 提取 `value` 属性
    #[serde(rename = "attr:value")]
    AttrValue,
    /// 提取自定义属性，格式为 `attr:your-attribute-name`
    #[serde(untagged)]
    CustomAttr(String),
}

impl ExtractType {
    /// 获取属性名（如果是属性提取类型）
    pub fn attribute_name(&self) -> Option<&str> {
        match self {
            Self::AttrHref => Some("href"),
            Self::AttrSrc => Some("src"),
            Self::AttrDataSrc => Some("data-src"),
            Self::AttrAlt => Some("alt"),
            Self::AttrTitle => Some("title"),
            Self::AttrClass => Some("class"),
            Self::AttrId => Some("id"),
            Self::AttrValue => Some("value"),
            Self::CustomAttr(s) => s.strip_prefix("attr:"),
            _ => None,
        }
    }

    /// 是否是属性提取类型
    pub fn is_attribute(&self) -> bool {
        self.attribute_name().is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identifier_valid() {
        assert!(Identifier::new("valid_name").is_ok());
        assert!(Identifier::new("_private").is_ok());
        assert!(Identifier::new("Name123").is_ok());
    }

    #[test]
    fn test_identifier_invalid() {
        assert!(Identifier::new("123start").is_err());
        assert!(Identifier::new("has-dash").is_err());
        assert!(Identifier::new("has space").is_err());
        assert!(Identifier::new("").is_err());
    }

    #[test]
    fn test_extract_type_attribute_name() {
        assert_eq!(ExtractType::AttrHref.attribute_name(), Some("href"));
        assert_eq!(ExtractType::Text.attribute_name(), None);
        assert_eq!(
            ExtractType::CustomAttr("attr:data-id".to_string()).attribute_name(),
            Some("data-id")
        );
    }

    #[test]
    fn test_http_method_has_body() {
        assert!(!HttpMethod::Get.has_body());
        assert!(HttpMethod::Post.has_body());
        assert!(HttpMethod::Put.has_body());
        assert!(HttpMethod::Patch.has_body());
    }
}
