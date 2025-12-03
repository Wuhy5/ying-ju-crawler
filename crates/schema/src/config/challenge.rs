//! 人机验证/反爬挑战处理 (Challenge Handling)
//!
//! 处理 Cloudflare、reCAPTCHA 等反爬保护机制。
//!
//! # 设计理念
//!
//! 1. **检测触发**：通过响应特征判断是否遇到验证
//! 2. **多种策略**：WebView 手动、自动重试、Cookie 复用等
//! 3. **状态持久化**：验证通过后缓存凭证，避免重复验证
//!
//! # 支持的验证类型
//!
//! - Cloudflare（JS Challenge、Turnstile、Under Attack Mode）
//! - reCAPTCHA v2/v3
//! - hCaptcha
//! - 自定义验证（滑块、点选等）

use crate::{script::Script, template::Template};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// ============================================================================
// 主配置
// ============================================================================

/// 人机验证处理配置
///
/// 定义如何检测和处理各种反爬验证机制
///
/// # 示例
///
/// ## Cloudflare 验证
/// ```toml
/// [challenge]
/// enabled = true
///
/// [[challenge.detectors]]
/// type = "cloudflare"
///
/// [challenge.handler]
/// type = "webview"
/// timeout_seconds = 120
/// success_check = "return !document.body.innerHTML.includes('Just a moment');"
/// ```
///
/// ## 自定义验证检测
/// ```toml
/// [challenge]
/// enabled = true
///
/// [[challenge.detectors]]
/// type = "custom"
/// status_codes = [403, 503]
/// body_patterns = ["验证码", "human verification"]
///
/// [challenge.handler]
/// type = "webview"
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ChallengeConfig {
    /// 是否启用验证处理（默认 true）
    #[serde(default = "default_true")]
    pub enabled: bool,

    /// 验证检测器列表
    /// 按顺序检查，首个匹配的触发处理
    pub detectors: Vec<ChallengeDetector>,

    /// 验证处理器
    pub handler: ChallengeHandler,

    /// 验证通过后的凭证缓存时间（秒）
    /// 在此时间内不会重新触发验证
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_duration: Option<u32>,

    /// 最大验证尝试次数（默认 3）
    #[serde(default = "default_max_attempts")]
    pub max_attempts: u32,
}

// ============================================================================
// 验证检测器
// ============================================================================

/// 验证检测器
///
/// 判断响应是否为人机验证页面
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum ChallengeDetector {
    /// Cloudflare 验证检测（自动识别）
    Cloudflare(CloudflareDetector),

    /// reCAPTCHA 检测
    Recaptcha(RecaptchaDetector),

    /// hCaptcha 检测
    Hcaptcha(HcaptchaDetector),

    /// 自定义检测规则
    Custom(Box<CustomDetector>),
}

/// Cloudflare 验证检测配置
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
#[serde(deny_unknown_fields)]
pub struct CloudflareDetector {
    /// 额外的检测模式（补充默认检测）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_patterns: Option<Vec<String>>,
}

/// reCAPTCHA 检测配置
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
#[serde(deny_unknown_fields)]
pub struct RecaptchaDetector {
    /// reCAPTCHA 版本（v2 或 v3）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<RecaptchaVersion>,
}

/// reCAPTCHA 版本
#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, Default)]
#[serde(rename_all = "lowercase")]
pub enum RecaptchaVersion {
    #[default]
    V2,
    V3,
}

/// hCaptcha 检测配置
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
#[serde(deny_unknown_fields)]
pub struct HcaptchaDetector {}

/// 自定义验证检测配置
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CustomDetector {
    /// 触发验证的 HTTP 状态码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_codes: Option<Vec<u16>>,

    /// 响应头匹配规则
    /// key: 头名称, value: 匹配模式（支持正则）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<std::collections::HashMap<String, String>>,

    /// 响应体包含的文本模式
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_patterns: Option<Vec<String>>,

    /// URL 匹配模式（正则）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_pattern: Option<String>,

    /// 自定义检测脚本
    /// 输入：响应对象，返回 true 表示检测到验证
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detect_script: Option<Script>,
}

// ============================================================================
// 验证处理器
// ============================================================================

/// 验证处理器
///
/// 定义如何处理检测到的人机验证
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum ChallengeHandler {
    /// WebView 手动验证
    /// 打开浏览器让用户完成验证
    Webview(WebviewHandler),

    /// 自动重试
    /// 等待后重试请求（适用于 JS Challenge）
    Retry(RetryHandler),

    /// Cookie 注入
    /// 使用预设的验证 Cookie
    Cookie(CookieHandler),

    /// 外部服务
    /// 调用第三方打码平台
    External(ExternalHandler),

    /// 自定义脚本处理
    Script(ScriptHandler),
}

/// WebView 手动验证处理器
///
/// 打开内嵌浏览器，让用户手动完成验证
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct WebviewHandler {
    /// 提示用户的说明文案
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tip: Option<String>,

    /// 验证超时时间（秒，默认 120）
    #[serde(default = "default_webview_timeout")]
    pub timeout_seconds: u32,

    /// 自定义 User-Agent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,

    /// 验证成功检测脚本
    /// 返回 true 表示验证完成
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_check: Option<String>,

    /// 验证成功检测间隔（毫秒，默认 500）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_interval_ms: Option<u32>,

    /// 验证完成后执行的脚本
    /// 用于提取和保存验证凭证
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_script: Option<Script>,

    /// 需要提取的 Cookie 名称
    /// 验证通过后自动保存这些 Cookie
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extract_cookies: Option<Vec<String>>,
}

/// 自动重试处理器
///
/// 适用于 Cloudflare JS Challenge 等自动完成的验证
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct RetryHandler {
    /// 重试前等待时间（毫秒，默认 5000）
    #[serde(default = "default_retry_delay")]
    pub delay_ms: u32,

    /// 最大重试次数（默认 3）
    #[serde(default = "default_max_retries")]
    pub max_retries: u32,

    /// 重试时使用 WebView 渲染
    /// 某些 JS Challenge 需要完整的浏览器环境
    #[serde(default)]
    pub use_webview: bool,

    /// 每次重试的延迟倍增因子（默认 1.5）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backoff_factor: Option<f32>,
}

/// Cookie 注入处理器
///
/// 使用预设的验证 Cookie 绕过验证
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CookieHandler {
    /// Cookie 来源
    #[serde(flatten)]
    pub source: CookieSource,

    /// Cookie 过期检测
    /// 返回 true 表示 Cookie 有效
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validate_script: Option<Script>,
}

/// Cookie 来源
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CookieSource {
    /// 从用户输入获取
    UserInput {
        /// 提示用户的说明
        tip: Option<String>,
        /// 需要的 Cookie 名称列表
        cookie_names: Vec<String>,
    },
    /// 从配置文件读取
    Config {
        /// Cookie 字符串或键值对
        cookies: String,
    },
    /// 从脚本获取
    Script(Script),
}

/// 外部服务处理器
///
/// 调用第三方打码平台（如 2captcha、anti-captcha）
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ExternalHandler {
    /// 服务商类型
    pub provider: CaptchaProvider,

    /// API 密钥（支持模板变量）
    pub api_key: Template,

    /// API 端点（可选，用于自建服务）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,

    /// 请求超时时间（秒，默认 120）
    #[serde(default = "default_external_timeout")]
    pub timeout_seconds: u32,

    /// 额外参数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_params: Option<std::collections::HashMap<String, serde_json::Value>>,
}

/// 打码平台提供商
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CaptchaProvider {
    /// 2captcha
    TwoCaptcha,
    /// Anti-Captcha
    AntiCaptcha,
    /// CapSolver
    CapSolver,
    /// 自定义服务
    Custom,
}

/// 脚本处理器
///
/// 完全自定义的验证处理逻辑
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ScriptHandler {
    /// 处理脚本
    /// 输入：请求上下文和响应，输出：处理后的凭证
    pub script: Script,

    /// 超时时间（秒）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<u32>,
}

// ============================================================================
// 默认值函数
// ============================================================================

fn default_true() -> bool {
    true
}

fn default_max_attempts() -> u32 {
    3
}

fn default_webview_timeout() -> u32 {
    120
}

fn default_retry_delay() -> u32 {
    5000
}

fn default_max_retries() -> u32 {
    3
}

fn default_external_timeout() -> u32 {
    120
}
