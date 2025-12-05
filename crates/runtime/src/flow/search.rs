//! # 搜索流程执行器

use crate::{
    Result,
    context::Context,
    error::RuntimeError,
    extractor::{ExtractEngine, SharedValue, value::ExtractValueData},
    flow::FlowExecutor,
    http::HttpClient,
    model::SearchItem,
    template::TemplateRenderer,
};
use async_trait::async_trait;
use crawler_schema::{extract::FieldExtractor, flow::SearchFlow};
use serde_json::{Map, Value};
use std::sync::Arc;

/// 搜索请求
#[derive(Debug, Clone)]
pub struct SearchRequest {
    /// 搜索关键词
    pub keyword: String,
    /// 页码
    pub page: u32,
}

/// 搜索结果
#[derive(Debug, Clone)]
pub struct SearchResponse {
    /// 搜索结果列表
    pub items: Vec<SearchItem>,
    /// 是否有下一页
    pub has_next: bool,
    /// 原始数据
    pub raw_items: Vec<Value>,
}

/// 搜索流程执行器
pub struct SearchFlowExecutor {
    flow: SearchFlow,
    http_client: Arc<HttpClient>,
    extract_engine: Arc<ExtractEngine>,
    base_url: String,
}

impl SearchFlowExecutor {
    pub fn new(flow: SearchFlow) -> Self {
        Self {
            flow,
            http_client: Arc::new(HttpClient::default()),
            extract_engine: Arc::new(ExtractEngine::new()),
            base_url: String::new(),
        }
    }

    pub fn with_http_client(mut self, client: Arc<HttpClient>) -> Self {
        self.http_client = client;
        self
    }

    pub fn with_extract_engine(mut self, engine: Arc<ExtractEngine>) -> Self {
        self.extract_engine = engine;
        self
    }

    pub fn with_base_url(mut self, base_url: String) -> Self {
        self.base_url = base_url;
        self
    }

    /// 提取字段值为字符串
    fn extract_string(
        &self,
        extractor: &FieldExtractor,
        input: &SharedValue,
        context: &Context,
    ) -> Option<String> {
        ExtractEngine::extract_field(extractor, input.as_ref(), context)
            .ok()
            .and_then(|v| v.as_str().map(|s| s.to_string()))
    }

    /// 从列表项提取搜索结果
    fn extract_item(&self, item_html: &SharedValue, context: &Context) -> Result<SearchItem> {
        let fields = &self.flow.fields;

        // 提取必需字段
        let title = self
            .extract_string(&fields.title.extractor, item_html, context)
            .ok_or_else(|| RuntimeError::Extraction("Failed to extract title".to_string()))?;

        let url = self
            .extract_string(&fields.url.extractor, item_html, context)
            .ok_or_else(|| RuntimeError::Extraction("Failed to extract url".to_string()))?;

        // 处理相对 URL
        let url = if !url.starts_with("http") && !self.base_url.is_empty() {
            if url.starts_with('/') {
                format!("{}{}", self.base_url.trim_end_matches('/'), url)
            } else {
                format!("{}/{}", self.base_url.trim_end_matches('/'), url)
            }
        } else {
            url
        };

        // 提取可选字段
        let cover = fields
            .cover
            .as_ref()
            .and_then(|f| self.extract_string(&f.extractor, item_html, context));

        let summary = fields
            .summary
            .as_ref()
            .and_then(|f| self.extract_string(&f.extractor, item_html, context));

        let author = fields
            .author
            .as_ref()
            .and_then(|f| self.extract_string(&f.extractor, item_html, context));

        let latest = fields
            .latest
            .as_ref()
            .and_then(|f| self.extract_string(&f.extractor, item_html, context));

        // 构建原始数据
        let mut raw = Map::new();
        raw.insert("title".to_string(), Value::String(title.clone()));
        raw.insert("url".to_string(), Value::String(url.clone()));
        if let Some(ref c) = cover {
            raw.insert("cover".to_string(), Value::String(c.clone()));
        }
        if let Some(ref s) = summary {
            raw.insert("summary".to_string(), Value::String(s.clone()));
        }
        if let Some(ref a) = author {
            raw.insert("author".to_string(), Value::String(a.clone()));
        }
        if let Some(ref l) = latest {
            raw.insert("latest".to_string(), Value::String(l.clone()));
        }

        Ok(SearchItem {
            title,
            url,
            cover,
            summary,
            author,
            latest,
            score: None,
            status: None,
            category: None,
            raw: Value::Object(raw),
        })
    }
}

#[async_trait]
impl FlowExecutor for SearchFlowExecutor {
    type Input = SearchRequest;
    type Output = SearchResponse;

    async fn execute(&self, input: Self::Input, context: &mut Context) -> Result<Self::Output> {
        // 设置上下文变量
        context.set("keyword", serde_json::json!(input.keyword));
        context.set("page", serde_json::json!(input.page));
        context.set("base_url", serde_json::json!(self.base_url));

        // 1. 渲染 URL
        let url = self.flow.url.render(context)?;
        let full_url = if !url.starts_with("http") && !self.base_url.is_empty() {
            format!("{}{}", self.base_url.trim_end_matches('/'), url)
        } else {
            url
        };

        // 2. 发起 HTTP 请求
        let response = self
            .http_client
            .get(&full_url)
            .await
            .map_err(|e| RuntimeError::HttpRequest(format!("Request failed: {}", e)))?;

        let html = response
            .text()
            .await
            .map_err(|e| RuntimeError::HttpRequest(format!("Failed to read response: {}", e)))?;

        // 3. 提取列表
        let html_value = Arc::new(ExtractValueData::Html(Arc::from(html.into_boxed_str())));
        let list_result =
            ExtractEngine::extract_field(&self.flow.list, html_value.as_ref(), context)?;

        // 4. 遍历列表项，提取字段
        let mut items = Vec::new();
        let mut raw_items = Vec::new();

        match list_result.as_ref() {
            ExtractValueData::Array(arr) => {
                for item_value in arr.iter() {
                    match self.extract_item(item_value, context) {
                        Ok(item) => {
                            raw_items.push(item.raw.clone());
                            items.push(item);
                        }
                        Err(e) => {
                            // 记录错误但继续处理
                            eprintln!("Warning: Failed to extract item: {}", e);
                        }
                    }
                }
            }
            ExtractValueData::Html(h) => {
                // 单个结果
                let item_value = Arc::new(ExtractValueData::Html(Arc::clone(h)));
                if let Ok(item) = self.extract_item(&item_value, context) {
                    raw_items.push(item.raw.clone());
                    items.push(item);
                }
            }
            _ => {}
        }

        // 5. 判断是否有下一页（简单实现：有结果就认为可能有下一页）
        let has_next = !items.is_empty();

        Ok(SearchResponse {
            items,
            has_next,
            raw_items,
        })
    }
}
