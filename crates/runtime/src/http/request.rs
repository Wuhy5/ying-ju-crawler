//! # 请求构建器
//!
//! 提供便捷的请求构建接口

use crate::{Result, context::Context, http::HttpClient, template::TemplateRenderer};
use crawler_schema::{HttpMethod, RequestConfig, Template};

/// 请求构建器
pub struct RequestBuilder<'a> {
    client: &'a HttpClient,
    url: Template,
    method: HttpMethod,
    body: Option<Template>,
    headers: std::collections::HashMap<String, Template>,
}

impl<'a> RequestBuilder<'a> {
    /// 创建新的请求构建器
    pub fn new(client: &'a HttpClient, url: Template) -> Self {
        Self {
            client,
            url,
            method: HttpMethod::Get,
            body: None,
            headers: std::collections::HashMap::new(),
        }
    }

    /// 设置 HTTP 方法
    pub fn method(mut self, method: HttpMethod) -> Self {
        self.method = method;
        self
    }

    /// 设置请求体
    pub fn body(mut self, body: Template) -> Self {
        self.body = Some(body);
        self
    }

    /// 添加请求头
    pub fn header<K: Into<String>>(mut self, key: K, value: Template) -> Self {
        self.headers.insert(key.into(), value);
        self
    }

    /// 应用请求配置
    pub fn with_config(mut self, config: &RequestConfig) -> Self {
        if let Some(method) = &config.method {
            self.method = *method;
        }
        if let Some(body) = &config.body {
            self.body = Some(body.clone());
        }
        if let Some(headers) = &config.headers {
            self.headers.extend(headers.clone());
        }
        self
    }

    /// 执行请求
    pub async fn execute(self, context: &Context) -> Result<reqwest::Response> {
        // 渲染 URL
        let url = self.url.render(context)?;

        match self.method {
            HttpMethod::Get => self.client.get(&url).await,
            HttpMethod::Post => {
                let body = if let Some(body_template) = self.body {
                    body_template.render(context)?
                } else {
                    String::new()
                };
                self.client.post(&url, body).await
            }
            _ => todo!("Implement other HTTP methods"),
        }
    }
}
