//! Runtime 错误类型
//!
//! 包含运行时相关的错误类型，用于模板渲染、验证和执行阶段。

use thiserror::Error;

/// 运行时错误类型
#[derive(Debug, Error, Clone)]
pub enum RuntimeError {
    // --- 模板相关错误 ---
    /// 模板语法错误
    #[error("模板语法错误: {message}")]
    TemplateSyntax { message: String },

    /// 模板渲染错误
    #[error("模板渲染错误: {message}")]
    TemplateRender { message: String },

    /// 模板变量未定义
    #[error("模板变量 '{variable}' 未定义")]
    UndefinedVariable { variable: String },

    // --- 验证相关错误 ---
    /// 组件未定义
    #[error("组件 '{component}' 未定义")]
    UndefinedComponent { component: String },

    /// 流程未定义
    #[error("流程 '{flow}' 未定义")]
    UndefinedFlow { flow: String },

    /// 循环引用检测
    #[error("检测到循环引用: {path}")]
    CircularReference { path: String },

    /// 脚本模块未定义
    #[error("脚本模块 '{module}' 未定义")]
    UndefinedScriptModule { module: String },

    /// 脚本函数未定义
    #[error("脚本函数 '{module}.{function}' 未定义")]
    UndefinedScriptFunction { module: String, function: String },

    // --- 配置相关错误 ---
    /// 配置缺失
    #[error("缺少必需的配置项: {field}")]
    MissingConfig { field: String },

    /// 配置值无效
    #[error("配置项 '{field}' 的值无效: {reason}")]
    InvalidConfigValue { field: String, reason: String },

    // --- 运行时资源限制错误 ---
    /// 执行超时
    #[error("执行超时: {operation} (耗时: {elapsed_ms}ms, 限制: {limit_ms}ms)")]
    ExecutionTimeout {
        operation: String,
        elapsed_ms: u64,
        limit_ms: u64,
    },

    // --- HTTP 相关错误 ---
    /// HTTP 配置错误
    #[error("HTTP 配置错误: {0}")]
    HttpConfig(String),

    /// HTTP 请求错误
    #[error("HTTP 请求错误: {0}")]
    HttpRequest(String),

    // --- 数据提取错误 ---
    /// 数据提取错误
    #[error("数据提取错误: {0}")]
    Extraction(String),

    // --- 配置文件错误 ---
    /// 配置文件错误
    #[error("配置文件错误: {0}")]
    Config(String),

    /// 模板验证错误
    #[error("模板验证错误 '{template}': {error}")]
    TemplateValidation { template: String, error: String },

    // --- 脚本执行错误 ---
    /// 脚本语法错误
    #[error("脚本语法错误: {0}")]
    ScriptSyntax(String),

    /// 脚本运行时错误
    #[error("脚本运行时错误: {0}")]
    ScriptRuntime(String),

    /// 脚本执行超时
    #[error("脚本执行超时")]
    ScriptTimeout,

    // --- WebView 相关错误 ---
    /// WebView 不可用
    #[error("WebView 不可用: {0}")]
    WebViewUnavailable(String),

    /// WebView 操作超时
    #[error("WebView 操作超时")]
    WebViewTimeout,

    /// WebView 被用户关闭
    #[error("WebView 被用户关闭")]
    WebViewUserClosed,

    /// WebView 执行错误
    #[error("WebView 执行错误: {0}")]
    WebViewError(String),

    // --- 人机验证相关错误 ---
    /// 验证检测失败
    #[error("检测到人机验证但处理失败: {0}")]
    ChallengeFailed(String),

    /// 验证尝试次数超限
    #[error("验证尝试次数超过限制 ({attempts} 次)")]
    ChallengeMaxAttempts { attempts: u32 },
}
