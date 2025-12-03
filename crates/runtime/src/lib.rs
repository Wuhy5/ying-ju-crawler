//! # Crawler Runtime - 爬虫运行时库
//!
//! 为 crawler-schema 提供运行时执行能力，包括：
//! - 模板渲染
//! - HTTP 请求
//! - 数据提取
//! - 流程执行
//! - WebView 集成（通过依赖注入）
//!
//! ## 架构设计
//!
//! ```text
//! CrawlerRuntime (入口)
//!   ↓
//! FlowExecutor (流程执行)
//!   ↓
//! TemplateRenderer → HttpClient → ExtractEngine
//!   ↓                    ↓
//! WebViewProvider ← ChallengeHandler
//!   ↓
//! 输出结果
//! ```

// 错误类型
pub mod error;

// 上下文管理
pub mod context;

// 模板引擎
pub mod template;

// HTTP 客户端
pub mod http;

// 数据提取器
pub mod extractor;

// 流程执行器
pub mod flow;

// 脚本执行引擎
pub mod script;

// 爬虫运行时主入口
pub mod crawler;

// WebView 提供者
pub mod webview;

// 工具函数
pub mod util;

// 重新导出常用类型
pub use context::Context;
pub use crawler::CrawlerRuntime;
pub use error::RuntimeError;
pub use script::{RhaiScriptEngine, ScriptContext, ScriptEngine};
pub use webview::{SharedWebViewProvider, WebViewProvider, WebViewRequest, WebViewResponse};

/// 运行时结果类型
pub type Result<T> = std::result::Result<T, RuntimeError>;
