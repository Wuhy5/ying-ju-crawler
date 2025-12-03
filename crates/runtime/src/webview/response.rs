//! WebView 响应类型

use std::collections::HashMap;

/// WebView 响应
///
/// WebView 关闭后返回的结果
#[derive(Debug, Clone, Default)]
pub struct WebViewResponse {
    /// 是否成功完成（用户完成验证）
    pub success: bool,

    /// 最终 URL（可能经过重定向）
    pub final_url: Option<String>,

    /// 提取的 Cookie
    pub cookies: HashMap<String, String>,

    /// 提取的 Header（如果有）
    pub headers: HashMap<String, String>,

    /// finish_script 的执行结果
    pub script_result: Option<String>,

    /// 页面 HTML 内容（可选）
    pub html: Option<String>,

    /// 错误信息（如果失败）
    pub error: Option<String>,

    /// WebView 关闭原因
    pub close_reason: WebViewCloseReason,
}

/// WebView 关闭原因
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum WebViewCloseReason {
    /// 成功完成（检测脚本返回 true）
    #[default]
    Success,
    /// 用户手动关闭
    UserClosed,
    /// 超时
    Timeout,
    /// 发生错误
    Error,
}

impl WebViewResponse {
    /// 创建成功响应
    pub fn success() -> Self {
        Self {
            success: true,
            close_reason: WebViewCloseReason::Success,
            ..Default::default()
        }
    }

    /// 创建失败响应
    pub fn failure(reason: WebViewCloseReason, error: Option<String>) -> Self {
        Self {
            success: false,
            close_reason: reason,
            error,
            ..Default::default()
        }
    }

    /// 设置 Cookie
    pub fn with_cookies(mut self, cookies: HashMap<String, String>) -> Self {
        self.cookies = cookies;
        self
    }

    /// 设置 Header
    pub fn with_headers(mut self, headers: HashMap<String, String>) -> Self {
        self.headers = headers;
        self
    }

    /// 设置最终 URL
    pub fn with_final_url(mut self, url: impl Into<String>) -> Self {
        self.final_url = Some(url.into());
        self
    }

    /// 设置脚本结果
    pub fn with_script_result(mut self, result: impl Into<String>) -> Self {
        self.script_result = Some(result.into());
        self
    }
}
