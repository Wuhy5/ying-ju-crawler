//! 登录流程 (LoginFlow)
//!
//! 支持多种登录方式的模式化设计，通过 `type` 字段区分完全不同的结构：
//! - `script`: 脚本交互模式，App 渲染原生 UI，脚本处理逻辑
//! - `webview`: 网页模式，打开浏览器，用户操作网页，脚本检测状态
//! - `credential`: 凭证模式，手动粘贴 Cookie/Token/Header 等认证信息

use crate::{script::Script, template::Template};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// ============================================================================
// 登录流程主类型
// ============================================================================

/// 登录流程配置
///
/// 通过 `type` 字段区分三种完全不同的登录模式：
/// - `script`: 脚本交互模式（覆盖 Legado 的 loginUi + loginUrl 场景）
/// - `webview`: 网页模式
/// - `credential`: 凭证模式（支持 Cookie、Header 等多种认证方式）
///
/// # 示例
///
/// ## 脚本交互模式
/// ```toml
/// [login]
/// type = "script"
/// ui = [
///   { type = "text", key = "username", label = "用户名" },
///   { type = "password", key = "password", label = "密码" },
///   { type = "image", key = "captcha_img", action = { name = "refresh_captcha" } },
///   { type = "text", key = "captcha", label = "验证码" },
/// ]
/// init_script = { name = "init_login" }
/// login_script = { name = "do_login" }
/// ```
///
/// ## WebView 模式
/// ```toml
/// [login]
/// type = "webview"
/// start_url = "https://example.com/login"
/// check_script = "return document.querySelector('.user-info') !== null;"
/// finish_script = { name = "save_cookies" }
/// ```
///
/// ## Credential 模式（Cookie）
/// ```toml
/// [login]
/// type = "credential"
/// tip = "请从浏览器开发者工具中复制 Cookie"
/// docs_url = "https://example.com/help/cookie"
/// storage = [{ type = "cookie" }]
/// ```
///
/// ## Credential 模式（Header Token）
/// ```toml
/// [login]
/// type = "credential"
/// tip = "请输入您的 API Token"
/// fields = [{ key = "token", label = "API Token", field_type = "password" }]
/// storage = [{ type = "header", header_name = "Authorization", header_template = "Bearer {{ token }}" }]
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum LoginFlow {
    /// 脚本交互模式
    /// App 渲染原生 UI，脚本处理逻辑（覆盖 Legado 的 loginUi + loginUrl 场景）
    Script(ScriptLoginFlow),

    /// 网页模式
    /// 打开浏览器，用户操作网页，脚本检测状态
    Webview(WebViewLoginFlow),

    /// 凭证模式
    /// 手动粘贴 Cookie/Token/Header 等认证信息
    Credential(CredentialLoginFlow),
}

// ============================================================================
// 脚本交互模式 (Script)
// ============================================================================

/// 脚本交互模式配置
///
/// App 渲染原生 UI（输入框、按钮、验证码图片），脚本处理逻辑。
/// 这种模式提供最灵活的登录体验，适用于复杂的登录场景。
///
/// # 工作流程
/// 1. App 根据 `ui` 定义渲染原生界面
/// 2. 界面打开时自动执行 `init_script`（如加载验证码）
/// 3. 用户填写表单，点击按钮触发对应的 `action` 脚本
/// 4. 用户点击登录按钮时执行 `login_script`
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ScriptLoginFlow {
    /// 流程的功能描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// 定义原生界面元素（输入框、按钮、验证码图片）
    pub ui: Vec<LoginUIElement>,

    /// 界面打开时自动执行的脚本
    /// 例如：自动加载图形验证码，或获取初始 Cookie
    #[serde(skip_serializing_if = "Option::is_none")]
    pub init_script: Option<Script>,

    /// 用户点击界面底部"登录/确认"按钮时执行的主逻辑脚本
    pub login_script: Script,
}

/// 登录界面 UI 元素定义
///
/// 仅用于 Script 模式，定义 App 原生渲染的界面元素
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum LoginUIElement {
    /// 文本输入框
    Text(TextInput),

    /// 密码输入框（隐藏显示）
    Password(PasswordInput),

    /// 功能按钮（如获取短信验证码）
    Button(ButtonElement),

    /// 图片显示（如图形验证码）
    Image(ImageElement),
}

/// 文本输入框配置
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct TextInput {
    /// 字段标识符（变量名，脚本通过此 key 获取用户输入）
    pub key: String,

    /// 字段显示名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

    /// 占位符文本
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,

    /// 是否必填（默认 true）
    #[serde(default = "default_true")]
    pub required: bool,
}

/// 密码输入框配置
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct PasswordInput {
    /// 字段标识符（变量名，脚本通过此 key 获取用户输入）
    pub key: String,

    /// 字段显示名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

    /// 占位符文本
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,

    /// 是否必填（默认 true）
    #[serde(default = "default_true")]
    pub required: bool,
}

/// 功能按钮配置
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ButtonElement {
    /// 按钮显示文本
    pub label: String,

    /// 点击按钮触发的脚本逻辑
    /// 例如：获取短信验证码、刷新图形验证码
    pub action: Script,

    /// 是否禁用（可选，用于条件性禁用按钮）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
}

/// 图片元素配置（用于验证码等）
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ImageElement {
    /// 绑定变量名
    /// 脚本更新此变量时，图片自动刷新
    pub key: String,

    /// 图片显示描述（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

    /// 点击图片触发的脚本（通常用于刷新验证码）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<Script>,
}

// ============================================================================
// 网页模式 (WebView)
// ============================================================================

/// 网页登录模式配置
///
/// 打开内嵌浏览器，用户手动操作网页完成登录，脚本检测登录状态。
/// 适用于复杂的网页登录（OAuth、多步骤验证等）。
///
/// # 工作流程
/// 1. App 打开 WebView 加载 `start_url`
/// 2. 页面加载完成后执行 `inject_script`（可选）
/// 3. 周期性执行 `check_script` 检测登录状态
/// 4. 当 `check_script` 返回 true 时，执行 `finish_script`
/// 5. 关闭 WebView，登录完成
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct WebViewLoginFlow {
    /// 流程的功能描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// 登录起始页 URL
    pub start_url: Template,

    /// 自定义 User-Agent（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,

    /// 是否允许重定向（默认 true）
    #[serde(default = "default_true")]
    pub allow_redirects: bool,

    /// 页面加载完成后注入的 JavaScript
    /// 用于处理网页内的 DOM，如点击协议、自动填表等
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inject_script: Option<String>,

    /// 周期性执行的检测 JavaScript
    /// 返回 true 代表登录成功
    /// 例如: `return document.querySelector('.user-info') !== null;`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_script: Option<String>,

    /// 检测间隔（毫秒，默认 500）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_interval_ms: Option<u32>,

    /// 登录成功后（WebView 关闭前）执行的整理脚本
    /// 用于从 WebView 提取特定 Cookie/LocalStorage 并保存到全局变量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_script: Option<Script>,

    /// 登录超时时间（秒，默认 300）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<u32>,
}

// ============================================================================
// 凭证模式 (Credential)
// ============================================================================

/// 凭证登录模式配置
///
/// 用户手动提供认证信息（Cookie/Token/Header 等）。
/// 适用于不支持自动登录或用户已有现成凭证的场景。
///
/// # 支持的认证方式
/// - **Cookie**: 将凭证存储为 Cookie，随请求自动发送
/// - **Header**: 将凭证添加到请求头（如 `Authorization: Bearer xxx`）
/// - **组合**: 同时使用多种方式
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CredentialLoginFlow {
    /// 流程的功能描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// 提示用户如何获取凭证的说明文案
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tip: Option<String>,

    /// 获取教程的链接（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docs_url: Option<String>,

    /// 需要用户提供的凭证字段（可选）
    /// 如果不提供，默认显示一个多行文本框用于输入 Cookie
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<CredentialField>>,

    /// 凭证存储方式（可选）
    /// 定义如何将用户输入的凭证应用到 HTTP 请求
    /// 如果不提供，默认作为 Cookie 处理
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage: Option<Vec<CredentialStorage>>,

    /// 凭证验证脚本（可选，验证用户输入的凭证是否有效）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validate_script: Option<Script>,
}

/// 凭证存储方式
///
/// 定义如何将用户输入的凭证应用到 HTTP 请求中
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum CredentialStorage {
    /// 存储为 Cookie
    /// 凭证将作为 Cookie 随请求发送
    Cookie(CookieStorage),

    /// 添加到请求头
    /// 凭证将添加到指定的 HTTP Header
    Header(HeaderStorage),
}

/// Cookie 存储配置
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CookieStorage {
    /// 要存储的字段 key（对应 fields 中的 key）
    /// 如果不指定，则使用名为 "cookie" 的字段或第一个 textarea 类型字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_key: Option<String>,

    /// Cookie 的域名（可选，默认为规则的 domain）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
}

/// Header 存储配置
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct HeaderStorage {
    /// HTTP Header 名称
    /// 例如: "Authorization", "X-Token", "X-Api-Key"
    pub header_name: String,

    /// Header 值模板
    /// 支持变量插值，如 "Bearer {{ token }}" 或直接 "{{ api_key }}"
    pub header_template: Template,
}

/// 凭证字段定义
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CredentialField {
    /// 字段标识符（变量名）
    pub key: String,

    /// 字段显示名称
    pub label: String,

    /// 字段类型
    #[serde(default)]
    pub field_type: CredentialFieldType,

    /// 占位符文本
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,

    /// 是否必填（默认 true）
    #[serde(default = "default_true")]
    pub required: bool,

    /// 帮助说明文本
    #[serde(skip_serializing_if = "Option::is_none")]
    pub help: Option<String>,
}

/// 凭证字段类型
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CredentialFieldType {
    /// 普通文本（单行）
    #[default]
    Text,
    /// 密码（隐藏显示）
    Password,
    /// 多行文本（如 Cookie 字符串、Token）
    Textarea,
}

// ============================================================================
// 辅助函数
// ============================================================================

fn default_true() -> bool {
    true
}
