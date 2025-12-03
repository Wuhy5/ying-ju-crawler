//! # 过滤器注册表
//!
//! 使用工厂模式管理所有过滤器

use crate::{Result, error::RuntimeError, extractor::ExtractValue};
use serde_json::Value;
use std::{collections::HashMap, sync::Arc};

/// 过滤器 trait
pub trait Filter: Send + Sync {
    /// 应用过滤器
    fn apply(&self, input: &ExtractValue, args: &[Value]) -> Result<ExtractValue>;
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
    pub fn apply(&self, name: &str, input: &ExtractValue, args: &[Value]) -> Result<ExtractValue> {
        let filter = self
            .get(name)
            .ok_or_else(|| RuntimeError::Extraction(format!("Filter not found: {}", name)))?;

        filter.apply(input, args)
    }

    /// 注册所有内置过滤器
    fn register_builtin_filters(&mut self) {
        // 字符串过滤器
        self.register("trim", crate::extractor::filter::string::TrimFilter);
        self.register("lower", crate::extractor::filter::string::LowerFilter);
        self.register("upper", crate::extractor::filter::string::UpperFilter);

        // 类型转换过滤器
        self.register("to_int", crate::extractor::filter::convert::ToIntFilter);
        self.register(
            "to_string",
            crate::extractor::filter::convert::ToStringFilter,
        );

        // TODO: 注册更多过滤器
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
