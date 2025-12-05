//! # 过滤器注册表
//!
//! 使用工厂模式管理所有过滤器

use crate::{Result, error::RuntimeError, extractor::SharedValue};
use serde_json::Value;
use std::{collections::HashMap, sync::Arc};

/// 过滤器 trait
pub trait Filter: Send + Sync {
    /// 应用过滤器
    fn apply(&self, input: &SharedValue, args: &[Value]) -> Result<SharedValue>;
}

/// 过滤器注册表（全局单例）
pub struct FilterRegistry {
    filters: HashMap<String, Arc<dyn Filter>>,
}

impl FilterRegistry {
    /// 创建新的注册表
    pub fn new() -> Self {
        let mut registry = Self {
            filters: HashMap::new(),
        };

        // 注册内置过滤器
        registry.register_builtin_filters();

        registry
    }

    /// 注册过滤器
    pub fn register<F: Filter + 'static>(&mut self, name: &str, filter: F) {
        self.filters.insert(name.to_string(), Arc::new(filter));
    }

    /// 获取过滤器
    pub fn get(&self, name: &str) -> Option<Arc<dyn Filter>> {
        self.filters.get(name).cloned()
    }

    /// 应用过滤器
    ///
    /// 接受输入值的所有权，内部使用引用传递给过滤器
    pub fn apply(&self, name: &str, input: SharedValue, args: &[Value]) -> Result<SharedValue> {
        let filter = self
            .get(name)
            .ok_or_else(|| RuntimeError::Extraction(format!("Filter not found: {}", name)))?;

        filter.apply(&input, args)
    }

    /// 注册所有内置过滤器
    fn register_builtin_filters(&mut self) {
        use crate::extractor::filter::{convert, string, url};

        // 字符串过滤器
        self.register("trim", string::TrimFilter);
        self.register("lower", string::LowerFilter);
        self.register("upper", string::UpperFilter);
        self.register("replace", string::ReplaceFilter);
        self.register("regex_replace", string::RegexReplaceFilter);
        self.register("split", string::SplitFilter);
        self.register("join", string::JoinFilter);
        self.register("strip_html", string::StripHtmlFilter);
        self.register("substring", string::SubstringFilter);

        // 类型转换过滤器
        self.register("to_int", convert::ToIntFilter);
        self.register("to_string", convert::ToStringFilter);

        // URL 过滤器
        self.register("absolute_url", url::AbsoluteUrlFilter);
        self.register("url_encode", url::UrlEncodeFilter);
        self.register("url_decode", url::UrlDecodeFilter);
    }
}

impl Default for FilterRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// 全局过滤器注册表实例
static GLOBAL_REGISTRY: std::sync::OnceLock<FilterRegistry> = std::sync::OnceLock::new();

/// 获取全局过滤器注册表
pub fn global_registry() -> &'static FilterRegistry {
    GLOBAL_REGISTRY.get_or_init(FilterRegistry::new)
}
