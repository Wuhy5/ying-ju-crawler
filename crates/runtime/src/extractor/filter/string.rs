//! # 字符串过滤器

use crate::{
    Result,
    error::RuntimeError,
    extractor::{SharedValue, filter::Filter, value::ExtractValueData},
};
use serde_json::Value;
use std::sync::Arc;

/// Trim 过滤器
pub struct TrimFilter;

impl Filter for TrimFilter {
    fn apply(&self, input: &SharedValue, _args: &[Value]) -> Result<SharedValue> {
        let s = input.as_str().ok_or_else(|| {
            RuntimeError::Extraction("trim filter requires string input".to_string())
        })?;
        Ok(Arc::new(ExtractValueData::String(Arc::from(
            s.trim().to_string().into_boxed_str(),
        ))))
    }
}

/// Lower 过滤器
pub struct LowerFilter;

impl Filter for LowerFilter {
    fn apply(&self, input: &SharedValue, _args: &[Value]) -> Result<SharedValue> {
        let s = input.as_str().ok_or_else(|| {
            RuntimeError::Extraction("lower filter requires string input".to_string())
        })?;
        Ok(Arc::new(ExtractValueData::String(Arc::from(
            s.to_lowercase().into_boxed_str(),
        ))))
    }
}

/// Upper 过滤器
pub struct UpperFilter;

impl Filter for UpperFilter {
    fn apply(&self, input: &SharedValue, _args: &[Value]) -> Result<SharedValue> {
        let s = input.as_str().ok_or_else(|| {
            RuntimeError::Extraction("upper filter requires string input".to_string())
        })?;
        Ok(Arc::new(ExtractValueData::String(Arc::from(
            s.to_uppercase().into_boxed_str(),
        ))))
    }
}

/// Replace 过滤器
/// 参数: [from, to]
pub struct ReplaceFilter;

impl Filter for ReplaceFilter {
    fn apply(&self, input: &SharedValue, args: &[Value]) -> Result<SharedValue> {
        let s = input.as_str().ok_or_else(|| {
            RuntimeError::Extraction("replace filter requires string input".to_string())
        })?;

        if args.len() < 2 {
            return Err(RuntimeError::Extraction(
                "replace filter requires 2 arguments: from, to".to_string(),
            ));
        }

        let from = args[0].as_str().ok_or_else(|| {
            RuntimeError::Extraction("replace: 'from' must be a string".to_string())
        })?;
        let to = args[1].as_str().ok_or_else(|| {
            RuntimeError::Extraction("replace: 'to' must be a string".to_string())
        })?;

        Ok(Arc::new(ExtractValueData::String(Arc::from(
            s.replace(from, to).into_boxed_str(),
        ))))
    }
}

/// RegexReplace 过滤器
/// 参数: [pattern, replacement]
pub struct RegexReplaceFilter;

impl Filter for RegexReplaceFilter {
    fn apply(&self, input: &SharedValue, args: &[Value]) -> Result<SharedValue> {
        let s = input.as_str().ok_or_else(|| {
            RuntimeError::Extraction("regex_replace filter requires string input".to_string())
        })?;

        if args.len() < 2 {
            return Err(RuntimeError::Extraction(
                "regex_replace filter requires 2 arguments: pattern, replacement".to_string(),
            ));
        }

        let pattern = args[0].as_str().ok_or_else(|| {
            RuntimeError::Extraction("regex_replace: 'pattern' must be a string".to_string())
        })?;
        let replacement = args[1].as_str().ok_or_else(|| {
            RuntimeError::Extraction("regex_replace: 'replacement' must be a string".to_string())
        })?;

        let re = regex::Regex::new(pattern)
            .map_err(|e| RuntimeError::Extraction(format!("Invalid regex pattern: {}", e)))?;

        Ok(Arc::new(ExtractValueData::String(Arc::from(
            re.replace_all(s, replacement).to_string().into_boxed_str(),
        ))))
    }
}

/// Split 过滤器
/// 参数: [separator]
pub struct SplitFilter;

impl Filter for SplitFilter {
    fn apply(&self, input: &SharedValue, args: &[Value]) -> Result<SharedValue> {
        let s = input.as_str().ok_or_else(|| {
            RuntimeError::Extraction("split filter requires string input".to_string())
        })?;

        let sep = args.first().and_then(|v| v.as_str()).unwrap_or(" ");

        let parts: Vec<SharedValue> = s
            .split(sep)
            .map(|p| {
                Arc::new(ExtractValueData::String(Arc::from(
                    p.to_string().into_boxed_str(),
                )))
            })
            .collect();

        Ok(Arc::new(ExtractValueData::Array(Arc::new(parts))))
    }
}

/// Join 过滤器
/// 参数: [separator]
pub struct JoinFilter;

impl Filter for JoinFilter {
    fn apply(&self, input: &SharedValue, args: &[Value]) -> Result<SharedValue> {
        let arr = input.as_array_slice().ok_or_else(|| {
            RuntimeError::Extraction("join filter requires array input".to_string())
        })?;

        let sep = args.first().and_then(|v| v.as_str()).unwrap_or("");

        let strings: Vec<String> = arr
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect();

        Ok(Arc::new(ExtractValueData::String(Arc::from(
            strings.join(sep).into_boxed_str(),
        ))))
    }
}

/// StripHtml 过滤器
/// 移除所有 HTML 标签
pub struct StripHtmlFilter;

impl Filter for StripHtmlFilter {
    fn apply(&self, input: &SharedValue, _args: &[Value]) -> Result<SharedValue> {
        let s = input.as_str().ok_or_else(|| {
            RuntimeError::Extraction("strip_html filter requires string input".to_string())
        })?;

        // 使用正则移除 HTML 标签
        let re = regex::Regex::new(r"<[^>]+>").unwrap();
        let result = re.replace_all(s, "").to_string();

        Ok(Arc::new(ExtractValueData::String(Arc::from(
            result.into_boxed_str(),
        ))))
    }
}

/// Substring 过滤器
/// 参数: [start, length?]
pub struct SubstringFilter;

impl Filter for SubstringFilter {
    fn apply(&self, input: &SharedValue, args: &[Value]) -> Result<SharedValue> {
        let s = input.as_str().ok_or_else(|| {
            RuntimeError::Extraction("substring filter requires string input".to_string())
        })?;

        let start = args.first().and_then(|v| v.as_i64()).unwrap_or(0) as usize;

        let len = args.get(1).and_then(|v| v.as_i64()).map(|l| l as usize);

        let chars: Vec<char> = s.chars().collect();
        let end = len
            .map(|l| (start + l).min(chars.len()))
            .unwrap_or(chars.len());
        let result: String = chars[start.min(chars.len())..end].iter().collect();

        Ok(Arc::new(ExtractValueData::String(Arc::from(
            result.into_boxed_str(),
        ))))
    }
}
