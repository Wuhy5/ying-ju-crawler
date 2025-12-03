//! # HTTP 客户端
//!
//! 封装 reqwest，提供连接池和重试机制

use crate::{Result, error::RuntimeError};
use crawler_schema::HttpConfig;
use std::time::Duration;

/// HTTP 客户端
///
/// 封装 reqwest::Client，提供连接池复用
#[derive(Clone)]
pub struct HttpClient {
    client: reqwest::Client,
    config: HttpConfig,
}

impl HttpClient {
    /// 创建新的 HTTP 客户端
    pub fn new(config: HttpConfig) -> Result<Self> {
        let mut client_builder = reqwest::Client::builder();

        // 配置超时
        if let Some(timeout) = config.timeout {
            client_builder = client_builder.timeout(Duration::from_secs(timeout as u64));
        }

        // 配置连接超时
        if let Some(connect_timeout) = config.connect_timeout {
            client_builder =
                client_builder.connect_timeout(Duration::from_secs(connect_timeout as u64));
        }

        // 配置重定向
        if let Some(follow) = config.follow_redirects {
            if !follow {
                client_builder = client_builder.redirect(reqwest::redirect::Policy::none());
            } else if let Some(max) = config.max_redirects {
                client_builder =
                    client_builder.redirect(reqwest::redirect::Policy::limited(max as usize));
            }
        }

        // 配置 SSL 验证
        if let Some(verify) = config.verify_ssl {
            client_builder = client_builder.danger_accept_invalid_certs(!verify);
        }

        // 配置代理
        if let Some(proxy) = &config.proxy {
            let proxy = reqwest::Proxy::all(proxy)
                .map_err(|e| RuntimeError::HttpConfig(format!("Invalid proxy: {}", e)))?;
            client_builder = client_builder.proxy(proxy);
        }

        // 配置连接池
        client_builder = client_builder.pool_max_idle_per_host(10);

        let client = client_builder
            .build()
            .map_err(|e| RuntimeError::HttpConfig(format!("Failed to build client: {}", e)))?;

        Ok(Self { client, config })
    }

    /// 获取底层 reqwest::Client
    pub fn inner(&self) -> &reqwest::Client {
        &self.client
    }

    /// 获取配置
    pub fn config(&self) -> &HttpConfig {
        &self.config
    }

    /// 发起 GET 请求
    pub async fn get(&self, url: &str) -> Result<reqwest::Response> {
        let mut request = self.client.get(url);

        // 应用全局 headers
        if let Some(headers) = &self.config.headers {
            for (key, value) in headers {
                request = request.header(key, value);
            }
        }

        // 应用 User-Agent
        if let Some(ua) = &self.config.user_agent {
            request = request.header("User-Agent", ua);
        }

        self.execute_with_retry(request).await
    }

    /// 发起 POST 请求
    pub async fn post(&self, url: &str, body: String) -> Result<reqwest::Response> {
        let mut request = self.client.post(url).body(body);

        // 应用全局 headers
        if let Some(headers) = &self.config.headers {
            for (key, value) in headers {
                request = request.header(key, value);
            }
        }

        // 应用 User-Agent
        if let Some(ua) = &self.config.user_agent {
            request = request.header("User-Agent", ua);
        }

        self.execute_with_retry(request).await
    }

    /// 执行请求（带重试）
    async fn execute_with_retry(
        &self,
        request: reqwest::RequestBuilder,
    ) -> Result<reqwest::Response> {
        let retry_count = self.config.retry_count.unwrap_or(0);
        let retry_delay = self.config.retry_delay.unwrap_or(1000);

        let mut last_error = None;

        for attempt in 0..=retry_count {
            if attempt > 0 {
                tokio::time::sleep(Duration::from_millis(retry_delay as u64)).await;
            }

            match request.try_clone() {
                Some(req) => match req.send().await {
                    Ok(response) => return Ok(response),
                    Err(e) => {
                        last_error = Some(e);
                    }
                },
                None => {
                    return Err(RuntimeError::HttpRequest(
                        "Failed to clone request".to_string(),
                    ));
                }
            }
        }

        Err(RuntimeError::HttpRequest(format!(
            "Request failed after {} retries: {}",
            retry_count,
            last_error.unwrap()
        )))
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new(HttpConfig::default()).expect("Failed to create default HttpClient")
    }
}
