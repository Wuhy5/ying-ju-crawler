//! # WebView 提供者模块
//!
//! 定义 WebView 运行器的抽象 trait，由外部实现注入。
//!
//! ## 设计理念
//!
//! Runtime 不直接依赖任何 WebView 库（如 wry、tauri），
//! 而是通过 trait 抽象，让调用方注入具体实现。
//!
//! ## 使用示例
//!
//! ```rust,ignore
//! // App 层实现
//! struct TauriWebViewProvider { /* ... */ }
//!
//! impl WebViewProvider for TauriWebViewProvider {
//!     async fn open(&self, request: WebViewRequest) -> Result<WebViewResponse> {
//!         // 使用 Tauri WebView 实现
//!     }
//! }
//!
//! // 创建 Runtime 时注入
//! let runtime = CrawlerRuntime::builder()
//!     .rule(rule)
//!     .webview_provider(TauriWebViewProvider::new())
//!     .build()?;
//! ```

mod provider;
mod request;
mod response;

pub use provider::*;
pub use request::*;
pub use response::*;
