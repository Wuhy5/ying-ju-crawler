//! 运行时限制配置 (RuntimeLimits)
//!
//! 提供对运行时资源使用的限制，防止：
//! - 无限递归
//! - 内存耗尽
//! - 超时问题

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// 默认最大管道长度
pub const DEFAULT_MAX_PIPELINE_LENGTH: usize = 100;
/// 默认最大递归深度
pub const DEFAULT_MAX_RECURSION_DEPTH: usize = 10;
/// 默认最大变量数量
pub const DEFAULT_MAX_VARIABLES: usize = 1000;
/// 默认最大字符串长度
pub const DEFAULT_MAX_STRING_LENGTH: usize = 10 * 1024 * 1024; // 10MB
/// 默认执行超时（毫秒）
pub const DEFAULT_EXECUTION_TIMEOUT_MS: u64 = 30_000; // 30秒
/// 默认最大循环迭代次数
pub const DEFAULT_MAX_LOOP_ITERATIONS: usize = 10_000;

/// 运行时限制配置
/// 用于限制规则执行时的资源使用
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct RuntimeLimits {
    /// 单个管道的最大步骤数
    #[serde(default = "default_max_pipeline_length")]
    pub max_pipeline_length: usize,

    /// 组件调用的最大递归深度
    #[serde(default = "default_max_recursion_depth")]
    pub max_recursion_depth: usize,

    /// 变量上下文中的最大变量数量
    #[serde(default = "default_max_variables")]
    pub max_variables: usize,

    /// 单个字符串的最大长度（字节）
    #[serde(default = "default_max_string_length")]
    pub max_string_length: usize,

    /// 整体执行超时时间（毫秒）
    #[serde(default = "default_execution_timeout_ms")]
    pub execution_timeout_ms: u64,

    /// 单次循环的最大迭代次数
    #[serde(default = "default_max_loop_iterations")]
    pub max_loop_iterations: usize,

    /// 最大HTTP请求数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_http_requests: Option<usize>,

    /// 最大并发请求数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrent_requests: Option<usize>,

    /// 最大响应体大小（字节）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_response_size: Option<usize>,
}

impl Default for RuntimeLimits {
    fn default() -> Self {
        Self {
            max_pipeline_length: DEFAULT_MAX_PIPELINE_LENGTH,
            max_recursion_depth: DEFAULT_MAX_RECURSION_DEPTH,
            max_variables: DEFAULT_MAX_VARIABLES,
            max_string_length: DEFAULT_MAX_STRING_LENGTH,
            execution_timeout_ms: DEFAULT_EXECUTION_TIMEOUT_MS,
            max_loop_iterations: DEFAULT_MAX_LOOP_ITERATIONS,
            max_http_requests: None,
            max_concurrent_requests: None,
            max_response_size: None,
        }
    }
}

impl RuntimeLimits {
    /// 创建宽松的限制配置（用于开发/测试）
    pub fn relaxed() -> Self {
        Self {
            max_pipeline_length: 500,
            max_recursion_depth: 50,
            max_variables: 10_000,
            max_string_length: 100 * 1024 * 1024, // 100MB
            execution_timeout_ms: 300_000,        // 5分钟
            max_loop_iterations: 100_000,
            max_http_requests: None,
            max_concurrent_requests: None,
            max_response_size: None,
        }
    }

    /// 创建严格的限制配置（用于生产/安全场景）
    pub fn strict() -> Self {
        Self {
            max_pipeline_length: 50,
            max_recursion_depth: 5,
            max_variables: 500,
            max_string_length: 1024 * 1024, // 1MB
            execution_timeout_ms: 10_000,   // 10秒
            max_loop_iterations: 1_000,
            max_http_requests: Some(50),
            max_concurrent_requests: Some(5),
            max_response_size: Some(5 * 1024 * 1024), // 5MB
        }
    }

    /// 验证管道长度
    pub fn check_pipeline_length(&self, length: usize) -> Result<(), crate::error::CrawlerError> {
        if length > self.max_pipeline_length {
            Err(crate::error::CrawlerError::ResourceLimitExceeded {
                limit_type: "管道长度".to_string(),
                current: length,
                max: self.max_pipeline_length,
            })
        } else {
            Ok(())
        }
    }

    /// 验证递归深度
    pub fn check_recursion_depth(&self, depth: usize) -> Result<(), crate::error::CrawlerError> {
        if depth > self.max_recursion_depth {
            Err(crate::error::CrawlerError::RecursionLimitExceeded {
                current: depth,
                max: self.max_recursion_depth,
            })
        } else {
            Ok(())
        }
    }

    /// 验证变量数量
    pub fn check_variable_count(&self, count: usize) -> Result<(), crate::error::CrawlerError> {
        if count > self.max_variables {
            Err(crate::error::CrawlerError::ResourceLimitExceeded {
                limit_type: "变量数量".to_string(),
                current: count,
                max: self.max_variables,
            })
        } else {
            Ok(())
        }
    }

    /// 验证循环迭代次数
    pub fn check_loop_iterations(
        &self,
        iterations: usize,
    ) -> Result<(), crate::error::CrawlerError> {
        if iterations > self.max_loop_iterations {
            Err(crate::error::CrawlerError::ResourceLimitExceeded {
                limit_type: "循环迭代次数".to_string(),
                current: iterations,
                max: self.max_loop_iterations,
            })
        } else {
            Ok(())
        }
    }
}

// 默认值函数
fn default_max_pipeline_length() -> usize {
    DEFAULT_MAX_PIPELINE_LENGTH
}

fn default_max_recursion_depth() -> usize {
    DEFAULT_MAX_RECURSION_DEPTH
}

fn default_max_variables() -> usize {
    DEFAULT_MAX_VARIABLES
}

fn default_max_string_length() -> usize {
    DEFAULT_MAX_STRING_LENGTH
}

fn default_execution_timeout_ms() -> u64 {
    DEFAULT_EXECUTION_TIMEOUT_MS
}

fn default_max_loop_iterations() -> usize {
    DEFAULT_MAX_LOOP_ITERATIONS
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_limits() {
        let limits = RuntimeLimits::default();
        assert_eq!(limits.max_pipeline_length, DEFAULT_MAX_PIPELINE_LENGTH);
        assert_eq!(limits.max_recursion_depth, DEFAULT_MAX_RECURSION_DEPTH);
    }

    #[test]
    fn test_check_pipeline_length() {
        let limits = RuntimeLimits::default();
        assert!(limits.check_pipeline_length(50).is_ok());
        assert!(limits.check_pipeline_length(200).is_err());
    }

    #[test]
    fn test_strict_vs_relaxed() {
        let strict = RuntimeLimits::strict();
        let relaxed = RuntimeLimits::relaxed();

        assert!(strict.max_pipeline_length < relaxed.max_pipeline_length);
        assert!(strict.max_recursion_depth < relaxed.max_recursion_depth);
    }
}
