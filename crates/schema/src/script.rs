//! 脚本调用类型定义
//!
//! 定义脚本调用的通用类型，可用于：
//! - 字段提取流程中的脚本步骤
//! - 登录流程中的脚本逻辑
//! - 其他需要脚本处理的场景
//!
//! # 设计理念
//!
//! 脚本调用统一使用 ScriptConfig 结构体，支持：
//! - 内联代码（code 字段）
//! - 外部文件（file 字段）
//! - 远程加载（url 字段）
//! - 安全配置覆盖（security 字段可覆盖全局配置）

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::config::ScriptSecurityConfig;

// ============================================================================
// 脚本引擎
// ============================================================================

/// 脚本引擎类型
///
/// 指定脚本执行环境，默认为 JavaScript。
#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "lowercase")]
pub enum ScriptEngine {
    /// JavaScript 脚本引擎
    #[default]
    JavaScript,
    /// Rhai 脚本引擎（轻量级，Rust 原生）
    Rhai,
    /// Lua 脚本引擎
    Lua,
    /// Python 脚本引擎
    Python,
}

// ============================================================================
// 脚本调用
// ============================================================================

/// 脚本调用配置
///
/// 统一的脚本定义结构，必须使用此类型（不再支持简单字符串）。
/// 支持多种脚本来源和完整的配置选项。
///
/// # 示例
///
/// ## 最简形式：内联代码
/// ```toml
/// [script]
/// code = "return input.trim().toUpperCase()"
/// ```
///
/// ## 指定引擎的内联代码
/// ```toml
/// [script]
/// code = '''
/// let result = input.split(",");
/// return result.map(s => s.trim());
/// '''
/// engine = "javascript"
/// ```
///
/// ## 引用外部文件
/// ```toml
/// [script]
/// file = "./scripts/login.js"
/// function = "handleLogin"
/// ```
///
/// ## 远程脚本
/// ```toml
/// [script]
/// url = "https://example.com/scripts/utils.js"
/// function = "processData"
/// ```
///
/// ## 带参数和安全配置
/// ```toml
/// [script]
/// code = "return input.replace(params.from, params.to)"
/// [script.params]
/// from = "old"
/// to = "new"
/// [script.security]
/// timeout_seconds = 60
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Script {
    /// 脚本来源（三选一：code、file、url）
    #[serde(flatten)]
    pub source: ScriptSource,

    /// 脚本引擎（可选，默认 JavaScript）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<ScriptEngine>,

    /// 要调用的函数名（可选）
    ///
    /// 如果脚本定义了多个函数，指定要调用的函数。
    /// 默认调用 `main` 或直接执行脚本。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function: Option<String>,

    /// 传递给脚本的参数（可选）
    ///
    /// 脚本可通过 `params` 对象访问这些参数。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<HashMap<String, serde_json::Value>>,

    /// 脚本安全配置（可选）
    ///
    /// 覆盖全局的 CrawlerRule 级别安全配置。
    /// 如果同时定义了全局和局部配置，局部配置优先。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security: Option<ScriptSecurityConfig>,
}

/// 脚本来源
///
/// 脚本代码的来源，三选一：
/// - 内联代码（code）
/// - 本地文件（file）
/// - 远程 URL（url）
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ScriptSource {
    /// 内联代码
    Code(String),
    /// 远程 URL
    Url(String),
}

// ============================================================================
// 实现
// ============================================================================

impl Script {
    /// 获取脚本来源
    pub fn source(&self) -> &ScriptSource {
        &self.source
    }

    /// 获取脚本引擎
    pub fn engine(&self) -> ScriptEngine {
        self.engine.unwrap_or_default()
    }

    /// 获取函数名
    pub fn function(&self) -> Option<&str> {
        self.function.as_deref()
    }

    /// 获取参数
    pub fn params(&self) -> Option<&HashMap<String, serde_json::Value>> {
        self.params.as_ref()
    }

    /// 获取安全配置
    pub fn security(&self) -> Option<&ScriptSecurityConfig> {
        self.security.as_ref()
    }
}

impl Default for Script {
    fn default() -> Self {
        Script {
            source: ScriptSource::Code(String::new()),
            engine: None,
            function: None,
            params: None,
            security: None,
        }
    }
}
