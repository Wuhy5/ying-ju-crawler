//! # 验证检测器
//!
//! 检测 HTTP 响应是否为人机验证页面

use crate::Result;
use crawler_schema::config::{
    ChallengeDetector,
    CloudflareDetector,
    CustomDetector,
    HcaptchaDetector,
    RecaptchaDetector,
    RecaptchaVersion,
};
use regex::Regex;
use std::collections::HashMap;

/// 检测结果
#[derive(Debug, Clone)]
pub struct DetectionResult {
    /// 是否检测到验证
    pub detected: bool,
    /// 检测到的验证类型
    pub challenge_type: Option<ChallengeType>,
    /// 额外信息（如 site_key 等）
    pub extra_info: HashMap<String, String>,
}

/// 验证类型
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChallengeType {
    /// Cloudflare JS Challenge
    CloudflareJs,
    /// Cloudflare Turnstile
    CloudflareTurnstile,
    /// Cloudflare Under Attack Mode
    CloudflareUnderAttack,
    /// reCAPTCHA v2
    RecaptchaV2,
    /// reCAPTCHA v3
    RecaptchaV3,
    /// hCaptcha
    Hcaptcha,
    /// 自定义验证
    Custom,
}

/// HTTP 响应上下文（用于检测）
#[derive(Debug)]
pub struct ResponseContext {
    /// HTTP 状态码
    pub status_code: u16,
    /// 响应头
    pub headers: HashMap<String, String>,
    /// 响应体
    pub body: String,
    /// 最终 URL（考虑重定向）
    pub final_url: String,
}

impl ResponseContext {
    /// 从 reqwest::Response 创建
    pub async fn from_response(response: reqwest::Response) -> Result<Self> {
        let status_code = response.status().as_u16();
        let final_url = response.url().to_string();
        let headers = response
            .headers()
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
            .collect();
        let body = response
            .text()
            .await
            .map_err(|e| crate::RuntimeError::HttpRequest(e.to_string()))?;

        Ok(Self {
            status_code,
            headers,
            body,
            final_url,
        })
    }

    /// 从已有数据创建
    pub fn new(
        status_code: u16,
        headers: HashMap<String, String>,
        body: String,
        final_url: String,
    ) -> Self {
        Self {
            status_code,
            headers,
            body,
            final_url,
        }
    }
}

impl DetectionResult {
    /// 未检测到验证
    pub fn not_detected() -> Self {
        Self {
            detected: false,
            challenge_type: None,
            extra_info: HashMap::new(),
        }
    }

    /// 检测到验证
    pub fn detected(challenge_type: ChallengeType) -> Self {
        Self {
            detected: true,
            challenge_type: Some(challenge_type),
            extra_info: HashMap::new(),
        }
    }

    /// 添加额外信息
    pub fn with_info(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.extra_info.insert(key.into(), value.into());
        self
    }
}

/// 验证检测器 trait
pub trait ChallengeDetectorExt {
    /// 检测响应是否为验证页面
    fn detect(&self, response: &ResponseContext) -> DetectionResult;
}

impl ChallengeDetectorExt for ChallengeDetector {
    fn detect(&self, response: &ResponseContext) -> DetectionResult {
        match self {
            ChallengeDetector::Cloudflare(config) => detect_cloudflare(config, response),
            ChallengeDetector::Recaptcha(config) => detect_recaptcha(config, response),
            ChallengeDetector::Hcaptcha(config) => detect_hcaptcha(config, response),
            ChallengeDetector::Custom(config) => detect_custom(config, response),
        }
    }
}

// ============================================================================
// Cloudflare 检测
// ============================================================================

/// Cloudflare 检测模式
const CLOUDFLARE_PATTERNS: &[&str] = &[
    // JS Challenge
    "Just a moment",
    "Checking your browser",
    "Please Wait... | Cloudflare",
    "_cf_chl_opt",
    "cf-please-wait",
    // Under Attack Mode
    "Attention Required! | Cloudflare",
    "cf-challenge-running",
    // Turnstile
    "challenges.cloudflare.com/turnstile",
    "cf-turnstile",
    // 通用
    "__cf_bm",
    "cf_clearance",
];

/// Cloudflare 响应头特征
const CLOUDFLARE_HEADERS: &[&str] = &["cf-ray", "cf-cache-status", "cf-mitigated"];

fn detect_cloudflare(config: &CloudflareDetector, response: &ResponseContext) -> DetectionResult {
    // 检查状态码
    if response.status_code != 403 && response.status_code != 503 && response.status_code != 429 {
        // 某些 Cloudflare 页面可能返回 200，继续检查内容
        if response.status_code != 200 {
            return DetectionResult::not_detected();
        }
    }

    // 检查响应头
    let has_cf_header = CLOUDFLARE_HEADERS
        .iter()
        .any(|h| response.headers.contains_key(*h));

    // 检查响应体
    let body_lower = response.body.to_lowercase();
    let mut challenge_type = None;

    for pattern in CLOUDFLARE_PATTERNS {
        if response.body.contains(pattern) || body_lower.contains(&pattern.to_lowercase()) {
            challenge_type = Some(if pattern.contains("turnstile") {
                ChallengeType::CloudflareTurnstile
            } else if pattern.contains("Attention Required") {
                ChallengeType::CloudflareUnderAttack
            } else {
                ChallengeType::CloudflareJs
            });
            break;
        }
    }

    // 检查额外模式
    if challenge_type.is_none()
        && let Some(extra) = &config.extra_patterns
    {
        for pattern in extra {
            if response.body.contains(pattern) {
                challenge_type = Some(ChallengeType::CloudflareJs);
                break;
            }
        }
    }

    // 综合判断
    if let Some(ct) = challenge_type {
        let mut result = DetectionResult::detected(ct);

        // 尝试提取 cf-ray
        if let Some(ray) = response.headers.get("cf-ray") {
            result = result.with_info("cf_ray", ray);
        }

        return result;
    }

    // 如果有 CF 头但没有验证内容，可能是正常的 CF CDN 响应
    if has_cf_header && (response.status_code == 403 || response.status_code == 503) {
        return DetectionResult::detected(ChallengeType::CloudflareJs);
    }

    DetectionResult::not_detected()
}

// ============================================================================
// reCAPTCHA 检测
// ============================================================================

const RECAPTCHA_PATTERNS: &[&str] = &[
    "www.google.com/recaptcha",
    "www.recaptcha.net",
    "g-recaptcha",
    "grecaptcha",
    "recaptcha/api.js",
    "recaptcha/enterprise.js",
];

fn detect_recaptcha(config: &RecaptchaDetector, response: &ResponseContext) -> DetectionResult {
    let body_lower = response.body.to_lowercase();

    for pattern in RECAPTCHA_PATTERNS {
        if body_lower.contains(&pattern.to_lowercase()) {
            let challenge_type = match config.version {
                Some(RecaptchaVersion::V3) => ChallengeType::RecaptchaV3,
                _ => ChallengeType::RecaptchaV2,
            };

            let mut result = DetectionResult::detected(challenge_type);

            // 尝试提取 site key
            if let Some(site_key) = extract_recaptcha_site_key(&response.body) {
                result = result.with_info("site_key", site_key);
            }

            return result;
        }
    }

    DetectionResult::not_detected()
}

/// 提取 reCAPTCHA site key
fn extract_recaptcha_site_key(body: &str) -> Option<String> {
    // 尝试匹配 data-sitekey
    let re = Regex::new(r#"data-sitekey=["']([^"']+)["']"#).ok()?;
    if let Some(caps) = re.captures(body) {
        return caps.get(1).map(|m| m.as_str().to_string());
    }

    // 尝试匹配 grecaptcha.render 调用
    let re =
        Regex::new(r#"grecaptcha\.render\([^,]+,\s*\{\s*["']?sitekey["']?\s*:\s*["']([^"']+)["']"#)
            .ok()?;
    if let Some(caps) = re.captures(body) {
        return caps.get(1).map(|m| m.as_str().to_string());
    }

    None
}

// ============================================================================
// hCaptcha 检测
// ============================================================================

const HCAPTCHA_PATTERNS: &[&str] = &[
    "hcaptcha.com",
    "h-captcha",
    "hcaptcha",
    "data-hcaptcha-widget-id",
];

fn detect_hcaptcha(_config: &HcaptchaDetector, response: &ResponseContext) -> DetectionResult {
    let body_lower = response.body.to_lowercase();

    for pattern in HCAPTCHA_PATTERNS {
        if body_lower.contains(&pattern.to_lowercase()) {
            let mut result = DetectionResult::detected(ChallengeType::Hcaptcha);

            // 尝试提取 site key
            if let Some(site_key) = extract_hcaptcha_site_key(&response.body) {
                result = result.with_info("site_key", site_key);
            }

            return result;
        }
    }

    DetectionResult::not_detected()
}

/// 提取 hCaptcha site key
fn extract_hcaptcha_site_key(body: &str) -> Option<String> {
    let re = Regex::new(r#"data-sitekey=["']([^"']+)["']"#).ok()?;
    if let Some(caps) = re.captures(body) {
        return caps.get(1).map(|m| m.as_str().to_string());
    }
    None
}

// ============================================================================
// 自定义检测
// ============================================================================

fn detect_custom(config: &CustomDetector, response: &ResponseContext) -> DetectionResult {
    // 检查状态码
    if let Some(codes) = &config.status_codes
        && !codes.contains(&response.status_code)
    {
        // 如果指定了状态码但不匹配，且没有其他条件，则不检测
        if config.body_patterns.is_none()
            && config.headers.is_none()
            && config.url_pattern.is_none()
        {
            return DetectionResult::not_detected();
        }
    }

    // 检查响应头
    if let Some(header_rules) = &config.headers {
        for (name, pattern) in header_rules {
            let header_value = response.headers.get(name).map(|s| s.as_str()).unwrap_or("");
            if let Ok(re) = Regex::new(pattern) {
                if !re.is_match(header_value) {
                    return DetectionResult::not_detected();
                }
            } else {
                // 不是正则，直接字符串匹配
                if !header_value.contains(pattern) {
                    return DetectionResult::not_detected();
                }
            }
        }
    }

    // 检查 URL 模式
    if let Some(url_pattern) = &config.url_pattern
        && let Ok(re) = Regex::new(url_pattern)
        && !re.is_match(&response.final_url)
    {
        return DetectionResult::not_detected();
    }

    // 检查响应体模式
    if let Some(patterns) = &config.body_patterns {
        let mut any_match = false;
        for pattern in patterns {
            if response.body.contains(pattern) {
                any_match = true;
                break;
            }
            // 尝试作为正则
            if let Ok(re) = Regex::new(pattern)
                && re.is_match(&response.body)
            {
                any_match = true;
                break;
            }
        }
        if !any_match {
            return DetectionResult::not_detected();
        }
    }

    // 如果所有条件都满足（或没有条件），检测为自定义验证
    DetectionResult::detected(ChallengeType::Custom)
}
