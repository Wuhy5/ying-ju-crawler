//! # 验证管理器
//!
//! 协调验证检测和处理的整体流程

use super::{
    ChallengeCredentials,
    ChallengeDetectorExt,
    ChallengeHandlerExt,
    CredentialsCache,
    DetectionResult,
    HandlerContext,
    ResponseContext,
};
use crate::{Result, RuntimeError, webview::SharedWebViewProvider};
use crawler_schema::{
    config::ChallengeConfig,
    script::{Script, ScriptSource},
};
use std::sync::Arc;
use url::Url;

/// 验证管理器
///
/// 负责检测和处理人机验证
pub struct ChallengeManager {
    /// 验证配置
    config: ChallengeConfig,
    /// WebView 提供者
    webview_provider: SharedWebViewProvider,
    /// 凭证缓存
    credentials_cache: Arc<CredentialsCache>,
    /// HTTP 客户端
    http_client: Option<reqwest::Client>,
}

impl ChallengeManager {
    /// 创建新的验证管理器
    pub fn new(config: ChallengeConfig, webview_provider: SharedWebViewProvider) -> Self {
        Self {
            config,
            webview_provider,
            credentials_cache: Arc::new(CredentialsCache::new()),
            http_client: None,
        }
    }

    /// 设置 HTTP 客户端（用于重试处理器）
    pub fn with_http_client(mut self, client: reqwest::Client) -> Self {
        self.http_client = Some(client);
        self
    }

    /// 设置凭证缓存
    pub fn with_credentials_cache(mut self, cache: Arc<CredentialsCache>) -> Self {
        self.credentials_cache = cache;
        self
    }

    /// 检测响应是否为验证页面
    pub fn detect(&self, response: &ResponseContext) -> DetectionResult {
        if !self.config.enabled {
            return DetectionResult::not_detected();
        }

        for detector in &self.config.detectors {
            let result = detector.detect(response);
            if result.detected {
                tracing::info!(
                    "检测到人机验证: {:?}, 额外信息: {:?}",
                    result.challenge_type,
                    result.extra_info
                );
                return result;
            }
        }

        DetectionResult::not_detected()
    }

    /// 处理验证
    ///
    /// 返回验证凭证，调用方需要将凭证应用到后续请求
    pub async fn handle(
        &self,
        url: &str,
        response: ResponseContext,
    ) -> Result<ChallengeCredentials> {
        // 提取域名用于缓存
        let domain = extract_domain(url).unwrap_or_else(|| url.to_string());

        // 检查缓存
        if let Some(cached) = self.credentials_cache.get(&domain).await
            && !cached.is_expired()
        {
            tracing::debug!("使用缓存的验证凭证: {}", domain);
            return Ok(cached);
        }

        // 检测验证类型
        let detection = self.detect(&response);
        if !detection.detected {
            return Ok(ChallengeCredentials::new());
        }

        // 构建处理上下文
        let ctx = HandlerContext {
            webview_provider: self.webview_provider.clone(),
            url: url.to_string(),
            detection,
            response,
            http_client: self.http_client.clone(),
        };

        // 尝试处理
        let mut last_error = None;
        for attempt in 1..=self.config.max_attempts {
            tracing::info!("验证处理尝试 {}/{}", attempt, self.config.max_attempts);

            match self.config.handler.handle(&ctx).await {
                Ok(credentials) => {
                    // 缓存凭证
                    let mut creds = credentials.clone();
                    if let Some(duration) = self.config.cache_duration {
                        creds = creds.with_ttl(duration);
                    }
                    self.credentials_cache.set(&domain, creds).await;

                    tracing::info!("验证处理成功");
                    return Ok(credentials);
                }
                Err(e) => {
                    tracing::warn!("验证处理失败 (尝试 {}): {}", attempt, e);
                    last_error = Some(e);
                }
            }
        }

        Err(last_error.unwrap_or(RuntimeError::ChallengeMaxAttempts {
            attempts: self.config.max_attempts,
        }))
    }

    /// 检测并处理验证（一体化接口）
    ///
    /// 如果检测到验证，自动处理并返回凭证
    pub async fn detect_and_handle(
        &self,
        url: &str,
        response: ResponseContext,
    ) -> Result<Option<ChallengeCredentials>> {
        let detection = self.detect(&response);
        if !detection.detected {
            return Ok(None);
        }

        let credentials = self.handle(url, response).await?;
        Ok(Some(credentials))
    }

    /// 获取域名的缓存凭证
    pub async fn get_cached_credentials(&self, url: &str) -> Option<ChallengeCredentials> {
        let domain = extract_domain(url)?;
        self.credentials_cache.get(&domain).await
    }

    /// 清除域名的缓存凭证
    pub async fn clear_cached_credentials(&self, url: &str) {
        if let Some(domain) = extract_domain(url) {
            self.credentials_cache.remove(&domain).await;
        }
    }
}

/// 提取 URL 的域名
fn extract_domain(url: &str) -> Option<String> {
    Url::parse(url)
        .ok()
        .and_then(|u| u.host_str().map(|s| s.to_string()))
}

/// 创建默认的 Cloudflare 验证配置
pub fn default_cloudflare_config() -> ChallengeConfig {
    use crawler_schema::config::{
        ChallengeDetector,
        ChallengeHandler,
        CloudflareDetector,
        WebviewHandler,
    };

    ChallengeConfig {
        enabled: true,
        detectors: vec![ChallengeDetector::Cloudflare(CloudflareDetector::default())],
        handler: ChallengeHandler::Webview(WebviewHandler {
            tip: Some("请完成人机验证".to_string()),
            timeout_seconds: 120,
            user_agent: None,
            success_check: Some(
                "return !document.body.innerHTML.includes('Just a moment')".to_string(),
            ),
            check_interval_ms: Some(500),
            finish_script: None,
            extract_cookies: Some(vec!["cf_clearance".to_string(), "__cf_bm".to_string()]),
        }),
        cache_duration: Some(3600), // 1 小时
        max_attempts: 3,
    }
}

/// 创建默认的 reCAPTCHA 验证配置
pub fn default_recaptcha_config() -> ChallengeConfig {
    use crawler_schema::config::{
        ChallengeDetector,
        ChallengeHandler,
        RecaptchaDetector,
        WebviewHandler,
    };

    ChallengeConfig {
        enabled: true,
        detectors: vec![ChallengeDetector::Recaptcha(RecaptchaDetector::default())],
        handler: ChallengeHandler::Webview(WebviewHandler {
            tip: Some("请完成 reCAPTCHA 验证".to_string()),
            timeout_seconds: 180,
            user_agent: None,
            success_check: Some(
                "return document.querySelector('.g-recaptcha-response')?.value?.length > 0"
                    .to_string(),
            ),
            check_interval_ms: Some(1000),
            finish_script: Some(Script {
                source: ScriptSource::Code(
                    "return { token: document.querySelector('.g-recaptcha-response')?.value }"
                        .to_string(),
                ),
                engine: None,
                function: None,
                params: None,
                security: None,
            }),
            extract_cookies: None,
        }),
        cache_duration: None,
        max_attempts: 3,
    }
}
