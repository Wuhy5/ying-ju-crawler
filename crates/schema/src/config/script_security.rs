//! 脚本执行安全配置

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// ============================================================================
// 默认值常量
// ============================================================================

/// 脚本安全配置的默认最大内存限制（MB）
pub const DEFAULT_MAX_MEMORY_MB: u64 = 128;

/// 脚本安全配置的默认超时时间（秒）
pub const DEFAULT_TIMEOUT_SECONDS: u64 = 30;

// ============================================================================
// 默认值函数
// ============================================================================

/// 默认是否允许文件访问
fn default_allow_file_access() -> bool {
    false
}

/// 默认是否允许网络访问
fn default_allow_network() -> bool {
    false
}

/// 辅助函数：判断 bool 是否为 false（用于 skip_serializing_if）
fn is_false(b: &bool) -> bool {
    !b
}

// ============================================================================
// 脚本安全配置
// ============================================================================

/// 脚本执行安全配置
///
/// 定义脚本执行时的安全限制（内存、文件访问、网络访问、超时）。
/// 可在全局 (CrawlerRule) 或局部 (Script) 定义，局部配置优先级更高。
///
/// # 示例
///
/// ## 全局配置（CrawlerRule 顶级）
/// ```toml
/// [script_security]
/// max_memory_mb = 128
/// allow_file_access = false
/// allow_network = false
/// timeout_seconds = 30
/// ```
///
/// ## 局部覆盖配置（Script 中）
/// ```toml
/// [script]
/// code = "..."
/// [script.security]
/// timeout_seconds = 60  # 覆盖全局的 30 秒
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ScriptSecurityConfig {
    /// 最大内存限制（MB）
    ///
    /// 脚本执行过程中允许分配的最大内存。超过此限制时中断脚本。
    /// 默认值：128 MB
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_memory_mb: Option<u64>,

    /// 是否允许文件系统访问
    ///
    /// - `true`: 允许脚本调用文件 I/O 函数（如 `fs.read`）
    /// - `false`: 禁用所有文件 I/O 操作
    /// - 默认值：false
    #[serde(
        default = "default_allow_file_access",
        skip_serializing_if = "is_false"
    )]
    pub allow_file_access: bool,

    /// 是否允许网络访问
    ///
    /// - `true`: 允许脚本发起网络请求
    /// - `false`: 禁用所有网络操作
    /// - 默认值：false
    ///
    /// **注意**：即使禁用，脚本也可通过内置的 `http` 函数进行受控网络访问。
    #[serde(default = "default_allow_network", skip_serializing_if = "is_false")]
    pub allow_network: bool,

    /// 脚本执行超时时间（秒）
    ///
    /// 脚本执行超过此时间时强制中断。`0` 表示无限制（不推荐）。
    /// 默认值：30 秒
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<u64>,
}

impl ScriptSecurityConfig {
    /// 合并配置：局部配置覆盖全局配置
    ///
    /// 用于解决配置继承问题。使用优先级规则：
    /// 1. 如果局部字段是 `Some`, 使用局部值
    /// 2. 如果局部字段是 `None`, 则：
    ///    - 对于 bool 字段，保留局部值（已有默认值）
    ///    - 对于 Option 字段，尝试使用全局值
    pub fn merge_with(mut self, global: Option<&ScriptSecurityConfig>) -> Self {
        if let Some(global) = global {
            if self.max_memory_mb.is_none() {
                self.max_memory_mb = global.max_memory_mb;
            }
            if !self.allow_file_access && global.allow_file_access {
                self.allow_file_access = true;
            }
            if !self.allow_network && global.allow_network {
                self.allow_network = true;
            }
            if self.timeout_seconds.is_none() {
                self.timeout_seconds = global.timeout_seconds;
            }
        }
        self
    }
}
