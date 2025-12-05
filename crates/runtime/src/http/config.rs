//! # HTTP 配置扩展
//!
//! 为 HttpConfig 提供合并和转换功能

use crawler_schema::config::{HttpConfig, RequestConfig, ResponseConfig};

/// HTTP 配置扩展 trait
pub trait HttpConfigExt {
    /// 合并配置（other 优先级更高）
    fn merge(&self, other: &Self) -> Self;

    /// 合并请求配置
    fn merge_request(&self, request: &RequestConfig) -> Self;
}

impl HttpConfigExt for HttpConfig {
    fn merge(&self, other: &Self) -> Self {
        Self {
            user_agent: other.user_agent.clone().or_else(|| self.user_agent.clone()),
            timeout: other.timeout.or(self.timeout),
            proxy: other.proxy.clone().or_else(|| self.proxy.clone()),
            follow_redirects: other.follow_redirects.or(self.follow_redirects),
            max_redirects: other.max_redirects.or(self.max_redirects),
            connect_timeout: other.connect_timeout.or(self.connect_timeout),
            verify_ssl: other.verify_ssl.or(self.verify_ssl),
            request_delay: other.request_delay.or(self.request_delay),
            max_concurrent: other.max_concurrent.or(self.max_concurrent),
            retry_count: other.retry_count.or(self.retry_count),
            retry_delay: other.retry_delay.or(self.retry_delay),
            request: merge_request_config(&self.request, &other.request),
            response: merge_response_config(&self.response, &other.response),
        }
    }

    fn merge_request(&self, request: &RequestConfig) -> Self {
        let mut result = self.clone();
        result.request = merge_request_config(&result.request, &Some(request.clone()));
        result
    }
}

/// 合并请求配置
fn merge_request_config(
    base: &Option<RequestConfig>,
    override_config: &Option<RequestConfig>,
) -> Option<RequestConfig> {
    match (base, override_config) {
        (None, None) => None,
        (Some(b), None) => Some(b.clone()),
        (None, Some(o)) => Some(o.clone()),
        (Some(b), Some(o)) => {
            let mut merged = b.clone();
            if o.method.is_some() {
                merged.method = o.method;
            }
            if o.body.is_some() {
                merged.body = o.body.clone();
            }
            if o.content_type.is_some() {
                merged.content_type = o.content_type.clone();
            }
            // 合并 headers
            merged.headers = match (&b.headers, &o.headers) {
                (None, None) => None,
                (Some(h), None) => Some(h.clone()),
                (None, Some(h)) => Some(h.clone()),
                (Some(bh), Some(oh)) => {
                    let mut h = bh.clone();
                    h.extend(oh.clone());
                    Some(h)
                }
            };
            Some(merged)
        }
    }
}

/// 合并响应配置
fn merge_response_config(
    base: &Option<ResponseConfig>,
    override_config: &Option<ResponseConfig>,
) -> Option<ResponseConfig> {
    match (base, override_config) {
        (None, None) => None,
        (Some(b), None) => Some(b.clone()),
        (None, Some(o)) => Some(o.clone()),
        (Some(b), Some(o)) => {
            let mut merged = b.clone();
            if o.encoding.is_some() {
                merged.encoding = o.encoding.clone();
            }
            if o.content_type.is_some() {
                merged.content_type = o.content_type.clone();
            }
            if o.preprocess.is_some() {
                merged.preprocess = o.preprocess.clone();
            }
            Some(merged)
        }
    }
}
