//! # URL 处理过滤器

use crate::{
    Result,
    error::RuntimeError,
    extractor::{SharedValue, filter::Filter, value::ExtractValueData},
};
use serde_json::Value;
use std::sync::Arc;

/// AbsoluteUrl 过滤器
/// 将相对 URL 转换为绝对 URL
/// 参数: [base_url]
pub struct AbsoluteUrlFilter;

impl Filter for AbsoluteUrlFilter {
    fn apply(&self, input: &SharedValue, args: &[Value]) -> Result<SharedValue> {
        let url = input.as_str().ok_or_else(|| {
            RuntimeError::Extraction("absolute_url filter requires string input".to_string())
        })?;

        // 如果已经是绝对 URL，直接返回
        if url.starts_with("http://") || url.starts_with("https://") {
            return Ok(Arc::new(ExtractValueData::String(Arc::from(
                url.to_string().into_boxed_str(),
            ))));
        }

        // 需要 base_url 参数
        let base_url = args.first().and_then(|v| v.as_str()).ok_or_else(|| {
            RuntimeError::Extraction("absolute_url filter requires base_url argument".to_string())
        })?;

        // 拼接 URL
        let absolute = if url.starts_with('/') {
            // 绝对路径
            let base = base_url.trim_end_matches('/');
            // 提取 base 的 origin (scheme + host)
            if let Some(idx) = base.find("://") {
                if let Some(path_start) = base[idx + 3..].find('/') {
                    format!("{}{}", &base[..idx + 3 + path_start], url)
                } else {
                    format!("{}{}", base, url)
                }
            } else {
                format!("{}{}", base, url)
            }
        } else {
            // 相对路径
            let base = base_url.trim_end_matches('/');
            format!("{}/{}", base, url)
        };

        Ok(Arc::new(ExtractValueData::String(Arc::from(
            absolute.into_boxed_str(),
        ))))
    }
}

/// UrlEncode 过滤器
pub struct UrlEncodeFilter;

impl Filter for UrlEncodeFilter {
    fn apply(&self, input: &SharedValue, _args: &[Value]) -> Result<SharedValue> {
        let s = input.as_str().ok_or_else(|| {
            RuntimeError::Extraction("url_encode filter requires string input".to_string())
        })?;

        Ok(Arc::new(ExtractValueData::String(Arc::from(
            urlencoding::encode(s).to_string().into_boxed_str(),
        ))))
    }
}

/// UrlDecode 过滤器
pub struct UrlDecodeFilter;

impl Filter for UrlDecodeFilter {
    fn apply(&self, input: &SharedValue, _args: &[Value]) -> Result<SharedValue> {
        let s = input.as_str().ok_or_else(|| {
            RuntimeError::Extraction("url_decode filter requires string input".to_string())
        })?;

        let decoded = urlencoding::decode(s)
            .map_err(|e| RuntimeError::Extraction(format!("Failed to decode URL: {}", e)))?;

        Ok(Arc::new(ExtractValueData::String(Arc::from(
            decoded.to_string().into_boxed_str(),
        ))))
    }
}
