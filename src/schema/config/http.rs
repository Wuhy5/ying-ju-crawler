//! 全局HTTP配置 (HttpConfig)

use super::traits::ConfigMerge;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 默认User-Agent
pub const DEFAULT_USER_AGENT: &str = "Mozilla/5.0 (compatible; YingJuCrawler/1.0)";
/// 默认超时时间（秒）
pub const DEFAULT_TIMEOUT: u32 = 30;
/// 默认是否跟随重定向
pub const DEFAULT_FOLLOW_REDIRECTS: bool = true;
/// 默认最大重定向次数
pub const DEFAULT_MAX_REDIRECTS: u32 = 10;

/// 全局HTTP配置 (HttpConfig)
/// 定义所有网络请求的默认行为。
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
#[serde(deny_unknown_fields)]
pub struct HttpConfig {
    /// 全局 User-Agent。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
    /// 全局请求超时时间（秒）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u32>,
    /// 全局代理地址。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy: Option<String>,
    /// 全局请求头。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, String>>,
    /// 是否允许重定向。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub follow_redirects: Option<bool>,
    /// 最大重定向次数。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_redirects: Option<u32>,
    /// 连接超时时间（秒）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect_timeout: Option<u32>,
    /// 是否验证SSL证书。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verify_ssl: Option<bool>,
    /// 请求间隔时间（毫秒），用于限流。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_delay: Option<u32>,
    /// 最大并发请求数。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrent: Option<u32>,
    /// 重试次数。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_count: Option<u32>,
    /// 重试间隔（毫秒）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_delay: Option<u32>,
}

impl HttpConfig {
    /// 创建带有默认值的配置
    pub fn with_defaults() -> Self {
        Self {
            user_agent: Some(DEFAULT_USER_AGENT.to_string()),
            timeout: Some(DEFAULT_TIMEOUT),
            follow_redirects: Some(DEFAULT_FOLLOW_REDIRECTS),
            max_redirects: Some(DEFAULT_MAX_REDIRECTS),
            verify_ssl: Some(true),
            ..Default::default()
        }
    }

    /// 获取user_agent，带默认值
    pub fn user_agent_or_default(&self) -> &str {
        self.user_agent.as_deref().unwrap_or(DEFAULT_USER_AGENT)
    }

    /// 获取timeout，带默认值
    pub fn timeout_or_default(&self) -> u32 {
        self.timeout.unwrap_or(DEFAULT_TIMEOUT)
    }

    /// 获取follow_redirects，带默认值
    pub fn follow_redirects_or_default(&self) -> bool {
        self.follow_redirects.unwrap_or(DEFAULT_FOLLOW_REDIRECTS)
    }
}

impl ConfigMerge for HttpConfig {
    fn merge(&self, other: &Self) -> Self {
        Self {
            user_agent: self.user_agent.merge(&other.user_agent),
            timeout: self.timeout.merge(&other.timeout),
            proxy: self.proxy.merge(&other.proxy),
            headers: match (&self.headers, &other.headers) {
                (Some(base), Some(overlay)) => Some(base.merge(overlay)),
                (None, Some(h)) => Some(h.clone()),
                (Some(h), None) => Some(h.clone()),
                (None, None) => None,
            },
            follow_redirects: self.follow_redirects.merge(&other.follow_redirects),
            max_redirects: self.max_redirects.merge(&other.max_redirects),
            connect_timeout: self.connect_timeout.merge(&other.connect_timeout),
            verify_ssl: self.verify_ssl.merge(&other.verify_ssl),
            request_delay: self.request_delay.merge(&other.request_delay),
            max_concurrent: self.max_concurrent.merge(&other.max_concurrent),
            retry_count: self.retry_count.merge(&other.retry_count),
            retry_delay: self.retry_delay.merge(&other.retry_delay),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_config_merge() {
        let global = HttpConfig {
            user_agent: Some("GlobalUA".to_string()),
            timeout: Some(60),
            proxy: Some("http://proxy:8080".to_string()),
            ..Default::default()
        };

        let local = HttpConfig {
            timeout: Some(30), // 覆盖全局
            verify_ssl: Some(false),
            ..Default::default()
        };

        let merged = global.merge(&local);

        assert_eq!(merged.user_agent, Some("GlobalUA".to_string())); // 保留全局
        assert_eq!(merged.timeout, Some(30)); // 被覆盖
        assert_eq!(merged.proxy, Some("http://proxy:8080".to_string())); // 保留全局
        assert_eq!(merged.verify_ssl, Some(false)); // 新增
    }

    #[test]
    fn test_with_defaults() {
        let config = HttpConfig::with_defaults();
        assert_eq!(config.user_agent_or_default(), DEFAULT_USER_AGENT);
        assert_eq!(config.timeout_or_default(), DEFAULT_TIMEOUT);
        assert!(config.follow_redirects_or_default());
    }
}
