//! # 运行时构建器
//!
//! 使用构建器模式创建 CrawlerRuntime

use crate::{
    Result,
    crawler::CrawlerRuntime,
    error::RuntimeError,
    webview::{SharedWebViewProvider, noop_provider},
};
use crawler_schema::CrawlerRule;
use std::path::Path;

/// 爬虫运行时构建器
///
/// 提供便捷的 API 来创建 CrawlerRuntime 实例
///
/// # 示例
///
/// ```rust,ignore
/// // 基本用法
/// let runtime = CrawlerRuntime::builder()
///     .rule_file("rule.toml")?
///     .build()?;
///
/// // 带 WebView 支持
/// let runtime = CrawlerRuntime::builder()
///     .rule(rule)
///     .webview_provider(my_webview_provider)
///     .build()?;
/// ```
pub struct CrawlerRuntimeBuilder {
    rule: Option<CrawlerRule>,
    webview_provider: Option<SharedWebViewProvider>,
}

impl CrawlerRuntimeBuilder {
    /// 创建新的构建器
    pub fn new() -> Self {
        Self {
            rule: None,
            webview_provider: None,
        }
    }

    /// 从规则对象构建
    pub fn rule(mut self, rule: CrawlerRule) -> Self {
        self.rule = Some(rule);
        self
    }

    /// 设置 WebView 提供者
    ///
    /// 如果规则包含登录流程（webview 模式）或人机验证配置，
    /// 必须提供 WebView 提供者，否则相关功能将失败。
    pub fn webview_provider<P: crate::webview::WebViewProvider + 'static>(
        mut self,
        provider: P,
    ) -> Self {
        self.webview_provider = Some(std::sync::Arc::new(provider));
        self
    }

    /// 设置共享的 WebView 提供者
    pub fn webview_provider_shared(mut self, provider: SharedWebViewProvider) -> Self {
        self.webview_provider = Some(provider);
        self
    }

    /// 从 TOML 文件加载规则
    pub fn rule_file<P: AsRef<Path>>(mut self, path: P) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| RuntimeError::Config(format!("Failed to read rule file: {}", e)))?;

        let rule: CrawlerRule = toml::from_str(&content)
            .map_err(|e| RuntimeError::Config(format!("Failed to parse TOML: {}", e)))?;

        self.rule = Some(rule);
        Ok(self)
    }

    /// 从 TOML 字符串加载规则
    pub fn rule_toml(mut self, toml: &str) -> Result<Self> {
        let rule: CrawlerRule = toml::from_str(toml)
            .map_err(|e| RuntimeError::Config(format!("Failed to parse TOML: {}", e)))?;

        self.rule = Some(rule);
        Ok(self)
    }

    /// 从 JSON 字符串加载规则
    pub fn rule_json(mut self, json: &str) -> Result<Self> {
        let rule: CrawlerRule = serde_json::from_str(json)
            .map_err(|e| RuntimeError::Config(format!("Failed to parse JSON: {}", e)))?;

        self.rule = Some(rule);
        Ok(self)
    }

    /// 构建运行时实例
    pub fn build(self) -> Result<CrawlerRuntime> {
        let rule = self
            .rule
            .ok_or_else(|| RuntimeError::Config("Rule not provided".to_string()))?;

        let webview_provider = self.webview_provider.unwrap_or_else(noop_provider);

        CrawlerRuntime::with_webview_provider(rule, webview_provider)
    }
}

impl Default for CrawlerRuntimeBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder() {
        let toml = r#"
            [meta]
            name = "Test"
            version = "1.0.0"
            author = "Test Author"

            [detail]
            url = "https://example.com/detail/{{ id }}"

            [search]
            url = "https://example.com/search?q={{ keyword }}"
        "#;

        let runtime = CrawlerRuntimeBuilder::new()
            .rule_toml(toml)
            .unwrap()
            .build();

        assert!(runtime.is_ok());

        // 默认没有 WebView 支持
        let runtime = runtime.unwrap();
        assert!(!runtime.has_webview_support());
    }
}
