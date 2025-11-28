//! 配置Trait模块
//!
//! 提供配置合并和继承机制，允许：
//! - 局部配置覆盖全局配置
//! - 配置项的智能合并
//! - 默认值填充

use std::collections::HashMap;

/// 配置合并Trait
/// 允许配置项实现继承和覆盖机制
pub trait ConfigMerge: Sized {
    /// 将other的非None字段合并到self
    /// other的值优先（覆盖self）
    fn merge(&self, other: &Self) -> Self;

    /// 使用默认值填充None字段
    fn with_defaults(&self) -> Self
    where
        Self: Default,
    {
        Self::default().merge(self)
    }
}

/// 为Option类型实现合并
impl<T: Clone> ConfigMerge for Option<T> {
    fn merge(&self, other: &Self) -> Self {
        other.clone().or_else(|| self.clone())
    }
}

/// 为HashMap实现合并（other的值覆盖self的值）
impl<K: Clone + Eq + std::hash::Hash, V: Clone> ConfigMerge for HashMap<K, V> {
    fn merge(&self, other: &Self) -> Self {
        let mut result = self.clone();
        for (k, v) in other {
            result.insert(k.clone(), v.clone());
        }
        result
    }
}

/// 配置验证Trait
pub trait ConfigValidate {
    /// 验证配置是否有效
    fn validate(&self) -> Result<(), crate::error::CrawlerError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_option_merge() {
        let a: Option<i32> = Some(1);
        let b: Option<i32> = Some(2);
        let c: Option<i32> = None;

        assert_eq!(a.merge(&b), Some(2)); // other覆盖self
        assert_eq!(a.merge(&c), Some(1)); // other为None，保留self
        assert_eq!(c.merge(&a), Some(1)); // self为None，使用other
    }

    #[test]
    fn test_hashmap_merge() {
        let mut a = HashMap::new();
        a.insert("key1", "value1");
        a.insert("key2", "value2");

        let mut b = HashMap::new();
        b.insert("key2", "new_value2");
        b.insert("key3", "value3");

        let merged = a.merge(&b);
        assert_eq!(merged.get("key1"), Some(&"value1"));
        assert_eq!(merged.get("key2"), Some(&"new_value2")); // 被覆盖
        assert_eq!(merged.get("key3"), Some(&"value3"));
    }
}
