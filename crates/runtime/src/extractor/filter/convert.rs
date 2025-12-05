//! # 类型转换过滤器

use crate::{
    Result,
    error::RuntimeError,
    extractor::{SharedValue, filter::Filter, value::ExtractValueData},
};
use serde_json::Value;
use std::sync::Arc;

/// ToInt 过滤器
pub struct ToIntFilter;

impl Filter for ToIntFilter {
    fn apply(&self, input: &SharedValue, _args: &[Value]) -> Result<SharedValue> {
        let s = input.as_str().ok_or_else(|| {
            RuntimeError::Extraction("to_int filter requires string input".to_string())
        })?;

        let num = s
            .parse::<i64>()
            .map_err(|e| RuntimeError::Extraction(format!("Failed to parse int: {}", e)))?;

        Ok(Arc::new(ExtractValueData::Json(Arc::new(Value::Number(
            num.into(),
        )))))
    }
}

/// ToString 过滤器
pub struct ToStringFilter;

impl Filter for ToStringFilter {
    fn apply(&self, input: &SharedValue, _args: &[Value]) -> Result<SharedValue> {
        let s = match input.as_ref() {
            ExtractValueData::String(s) => s.to_string(),
            ExtractValueData::Json(v) => v.to_string(),
            ExtractValueData::Html(h) => h.to_string(),
            ExtractValueData::Array(_) => {
                return Err(RuntimeError::Extraction(
                    "Cannot convert array to string".to_string(),
                ));
            }
            ExtractValueData::Null => String::new(),
        };

        Ok(Arc::new(ExtractValueData::String(Arc::from(
            s.into_boxed_str(),
        ))))
    }
}

// TODO: 实现更多转换过滤器
// - to_float
// - to_bool
// - from_json
// - to_json
