//! # 字符串过滤器

use crate::{
    Result,
    error::RuntimeError,
    extractor::{ExtractValue, filter::Filter},
};
use serde_json::Value;

/// Trim 过滤器
pub struct TrimFilter;

impl Filter for TrimFilter {
    fn apply(&self, input: &ExtractValue, _args: &[Value]) -> Result<ExtractValue> {
        let s = input.as_string().ok_or_else(|| {
            RuntimeError::Extraction("trim filter requires string input".to_string())
        })?;
        Ok(ExtractValue::String(s.trim().to_string()))
    }
}

/// Lower 过滤器
pub struct LowerFilter;

impl Filter for LowerFilter {
    fn apply(&self, input: &ExtractValue, _args: &[Value]) -> Result<ExtractValue> {
        let s = input.as_string().ok_or_else(|| {
            RuntimeError::Extraction("lower filter requires string input".to_string())
        })?;
        Ok(ExtractValue::String(s.to_lowercase()))
    }
}

/// Upper 过滤器
pub struct UpperFilter;

impl Filter for UpperFilter {
    fn apply(&self, input: &ExtractValue, _args: &[Value]) -> Result<ExtractValue> {
        let s = input.as_string().ok_or_else(|| {
            RuntimeError::Extraction("upper filter requires string input".to_string())
        })?;
        Ok(ExtractValue::String(s.to_uppercase()))
    }
}

// TODO: 实现更多字符串过滤器
// - replace
// - strip_html
// - split
// - join
// 等
