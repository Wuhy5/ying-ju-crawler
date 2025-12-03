//! WebView 提供者 trait

use super::{WebViewRequest, WebViewResponse};
use crate::Result;
use async_trait::async_trait;
use std::sync::Arc;

/// WebView 提供者 trait
///
/// 由外部实现，注入到 Runtime 中使用。
///
/// # 实现示例
///
/// ## Tauri 实现
/// ```rust,ignore
/// use crawler_runtime::webview::{WebViewProvider, WebViewRequest, WebViewResponse};
///
/// struct TauriWebViewProvider {
///     app_handle: tauri::AppHandle,
/// }
///
/// #[async_trait]
/// impl WebViewProvider for TauriWebViewProvider {
///     async fn open(&self, request: WebViewRequest) -> Result<WebViewResponse> {
///         // 创建 Tauri WebView 窗口
///         let window = tauri::WebviewWindowBuilder::new(
///             &self.app_handle,
///             "challenge",
///             tauri::WebviewUrl::External(request.url.parse()?),
///         )
///         .title(request.title.unwrap_or("验证".to_string()))
///         .build()?;
///         
///         // 等待验证完成...
///     }
/// }
/// ```
///
/// ## wry 独立实现
/// ```rust,ignore
/// struct WryWebViewProvider;
///
/// #[async_trait]
/// impl WebViewProvider for WryWebViewProvider {
///     async fn open(&self, request: WebViewRequest) -> Result<WebViewResponse> {
///         // 使用 wry 创建窗口
///     }
/// }
/// ```
#[async_trait]
pub trait WebViewProvider: Send + Sync {
    /// 打开 WebView 窗口
    ///
    /// 阻塞直到用户完成操作或超时
    async fn open(&self, request: WebViewRequest) -> Result<WebViewResponse>;

    /// 是否支持无头模式
    ///
    /// 某些实现（如 Playwright）可以在无 GUI 环境下运行
    fn supports_headless(&self) -> bool {
        false
    }

    /// 获取提供者名称（用于日志）
    fn name(&self) -> &str {
        "WebViewProvider"
    }
}

/// 空实现（用于不需要 WebView 的场景）
///
/// 当规则不包含 WebView 相关配置时可以使用
pub struct NoopWebViewProvider;

#[async_trait]
impl WebViewProvider for NoopWebViewProvider {
    async fn open(&self, _request: WebViewRequest) -> Result<WebViewResponse> {
        Err(crate::error::RuntimeError::WebViewUnavailable(
            "WebView 提供者未配置".to_string(),
        ))
    }

    fn name(&self) -> &str {
        "NoopWebViewProvider"
    }
}

/// WebView 提供者的共享引用类型
pub type SharedWebViewProvider = Arc<dyn WebViewProvider>;

/// 创建空的 WebView 提供者
pub fn noop_provider() -> SharedWebViewProvider {
    Arc::new(NoopWebViewProvider)
}
