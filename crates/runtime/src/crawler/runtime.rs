//! # 爬虫运行时
//!
//! 主入口，整合所有模块

use crate::{
    Result,
    context::Context,
    error::RuntimeError,
    extractor::ExtractEngine,
    flow::{
        FlowExecutor,
        detail::{DetailFlowExecutor, DetailRequest, DetailResponse},
        search::{SearchFlowExecutor, SearchRequest, SearchResponse},
    },
    http::HttpClient,
    template::TemplateEngine,
    webview::{SharedWebViewProvider, noop_provider},
};
use crawler_schema::CrawlerRule;
use std::sync::Arc;

/// 爬虫运行时
///
/// 整合所有组件，提供统一的爬虫接口
pub struct CrawlerRuntime {
    /// 爬虫规则
    rule: Arc<CrawlerRule>,
    /// HTTP 客户端
    http_client: Arc<HttpClient>,
    /// 模板引擎
    template_engine: Arc<TemplateEngine>,
    /// 提取引擎
    extract_engine: Arc<ExtractEngine>,
    /// WebView 提供者
    webview_provider: SharedWebViewProvider,
    /// 全局上下文
    context: Context,
}

impl CrawlerRuntime {
    /// 创建新的运行时实例（不带 WebView 支持）
    ///
    /// 如果规则包含需要 WebView 的配置（如登录、人机验证），
    /// 相关功能将不可用。推荐使用 `builder()` 方法注入 WebView 提供者。
    pub fn new(rule: CrawlerRule) -> Result<Self> {
        Self::with_webview_provider(rule, noop_provider())
    }

    /// 创建带 WebView 支持的运行时实例
    pub fn with_webview_provider(
        rule: CrawlerRule,
        webview_provider: SharedWebViewProvider,
    ) -> Result<Self> {
        // 创建 HTTP 客户端
        let http_config = rule.http.clone().unwrap_or_default();
        let http_client = Arc::new(HttpClient::new(http_config)?);

        // 创建模板引擎
        let template_engine = Arc::new(TemplateEngine::new()?);

        // 创建提取引擎
        let extract_engine = Arc::new(ExtractEngine::new());

        // 创建全局上下文
        let context = Context::new();

        Ok(Self {
            rule: Arc::new(rule),
            http_client,
            template_engine,
            extract_engine,
            webview_provider,
            context,
        })
    }

    /// 获取构建器
    pub fn builder() -> crate::crawler::CrawlerRuntimeBuilder {
        crate::crawler::CrawlerRuntimeBuilder::new()
    }

    /// 搜索
    pub async fn search(&self, keyword: &str, page: u32) -> Result<SearchResponse> {
        let executor = SearchFlowExecutor::new(self.rule.search.clone());
        let request = SearchRequest {
            keyword: keyword.to_string(),
            page,
        };

        let mut context = self.context.clone();
        executor.execute(request, &mut context).await
    }

    /// 获取详情
    pub async fn detail(&self, url: &str) -> Result<DetailResponse> {
        let executor = DetailFlowExecutor::new(self.rule.detail.clone());
        let request = DetailRequest {
            url: url.to_string(),
        };

        let mut context = self.context.clone();
        executor.execute(request, &mut context).await
    }

    /// 获取规则元信息
    pub fn meta(&self) -> &crawler_schema::Meta {
        &self.rule.meta
    }

    /// 获取 HTTP 客户端
    pub fn http_client(&self) -> &HttpClient {
        &self.http_client
    }

    /// 获取模板引擎
    pub fn template_engine(&self) -> &TemplateEngine {
        &self.template_engine
    }

    /// 获取提取引擎
    pub fn extract_engine(&self) -> &ExtractEngine {
        &self.extract_engine
    }

    /// 获取 WebView 提供者
    pub fn webview_provider(&self) -> &SharedWebViewProvider {
        &self.webview_provider
    }

    /// 获取全局上下文
    pub fn context(&self) -> &Context {
        &self.context
    }

    /// 获取全局上下文的可变引用
    pub fn context_mut(&mut self) -> &mut Context {
        &mut self.context
    }

    /// 检查是否支持 WebView 功能
    pub fn has_webview_support(&self) -> bool {
        self.webview_provider.name() != "NoopWebViewProvider"
    }
}

impl TryFrom<CrawlerRule> for CrawlerRuntime {
    type Error = RuntimeError;

    fn try_from(rule: CrawlerRule) -> Result<Self> {
        Self::new(rule)
    }
}
