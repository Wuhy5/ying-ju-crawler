//! 模板字符串类型定义
//!
//! 本模块仅包含模板字符串的 Schema 定义，不包含运行时逻辑。
//! 运行时渲染和验证功能请使用 `crate::runtime::template` 模块。

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// 模板字符串 (Template)
///
/// 支持 Tera 模板语法的字符串类型，用于在运行时进行变量插值。
///
/// ## 语法规范
/// - 变量插值: `{{ variable }}`
/// - 嵌套访问: `{{ user.name }}`, `{{ items[0] }}`
/// - 过滤器: `{{ name | upper }}`
/// - 条件: `{% if condition %}...{% endif %}`
/// - 循环: `{% for item in items %}...{% endfor %}`
///
/// ## 示例
/// ```text
/// https://example.com/search?q={{ keyword }}
/// Hello, {{ user.name }}!
/// {{ items[0].title }}
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Hash, Default)]
#[serde(transparent)]
pub struct Template(
    #[schemars(pattern(
        "{{\\s*([a-zA-Z_][a-zA-Z0-9_]*(?:\\.[a-zA-Z_][a-zA-Z0-9_]*|\\[[0-9]+\\])*)\\s*}}"
    ))]
    String,
);

impl Template {
    /// 创建新模板
    #[inline]
    pub fn new(template: impl Into<String>) -> Self {
        Self(template.into())
    }

    /// 获取原始字符串
    #[inline]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// 转换为内部字符串
    #[inline]
    pub fn into_string(self) -> String {
        self.0
    }

    /// 检查是否为空
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl From<String> for Template {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for Template {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl AsRef<str> for Template {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Template {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_creation() {
        let t1 = Template::new("Hello {{ name }}");
        assert_eq!(t1.as_str(), "Hello {{ name }}");

        let t2: Template = "World".into();
        assert_eq!(t2.as_str(), "World");

        let t3: Template = String::from("Test").into();
        assert_eq!(t3.as_str(), "Test");
    }

    #[test]
    fn test_template_empty() {
        let empty = Template::default();
        assert!(empty.is_empty());

        let not_empty = Template::new("content");
        assert!(!not_empty.is_empty());
    }
}
