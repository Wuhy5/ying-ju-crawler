//! # 类型转换过滤器

use crate::{
    Result,
    error::RuntimeError,
    extractor::{ExtractValue, filter::Filter},
};
use serde_json::Value;

/// ToInt 过滤器
pub struct ToIntFilter;

impl Filter for ToIntFilter {
    fn apply(&self, input: &ExtractValue, _args: &[Value]) -> Result<ExtractValue> {
        let s = input.as_string().ok_or_else(|| {
            RuntimeError::Extraction("to_int filter requires string input".to_string())
        })?;

        let num = s
            .parse::<i64>()
            .map_err(|e| RuntimeError::Extraction(format!("Failed to parse int: {}", e)))?;

        Ok(ExtractValue::Json(Value::Number(num.into())))
    }
}

/// ToString 过滤器
pub struct ToStringFilter;

impl Filter for ToStringFilter {
    fn apply(&self, input: &ExtractValue, _args: &[Value]) -> Result<ExtractValue> {
        let s = match input {
            ExtractValue::String(s) => s.clone(),
            ExtractValue::Json(v) => v.to_string(),
            ExtractValue::Html(h) => h.clone(),
            ExtractValue::Array(_) => {
                return Err(RuntimeError::Extraction(
                    "Cannot convert array to string".to_string(),
                ));
            }
            ExtractValue::Null => String::new(),
        };

        Ok(ExtractValue::String(s))
    }
}

// TODO: 实现更多转换过滤器
// - to_float
// - to_bool
// - from_json
// - to_json
