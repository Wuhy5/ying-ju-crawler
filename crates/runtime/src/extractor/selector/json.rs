//! # JSON 选择器执行器

use crate::{
    Result,
    context::Context,
    error::RuntimeError,
    extractor::value::{ExtractValueData, SharedValue},
};
use crawler_schema::extract::SelectorStep;
use jsonpath_rust::JsonPath;
use serde_json::Value;
use std::sync::Arc;

/// JSON 选择器执行器
pub struct JsonSelectorExecutor;

impl JsonSelectorExecutor {
    /// 执行 JSON 选择器
    pub fn execute(
        selector: &SelectorStep,
        input: &ExtractValueData,
        _context: &Context,
    ) -> Result<SharedValue> {
        // 获取 JSON 值
        let json: Value = match input {
            ExtractValueData::Json(v) => (**v).clone(),
            ExtractValueData::String(s) => serde_json::from_str(s)
                .map_err(|e| RuntimeError::Extraction(format!("Failed to parse JSON: {}", e)))?,
            ExtractValueData::Array(arr) => {
                // 如果是数组，对每个元素应用选择器
                let results: Vec<SharedValue> = arr
                    .iter()
                    .filter_map(|item| Self::execute(selector, item, _context).ok())
                    .collect();
                return Ok(Arc::new(ExtractValueData::Array(Arc::new(results))));
            }
            _ => {
                return Err(RuntimeError::Extraction(
                    "JSON selector requires JSON input".to_string(),
                ));
            }
        };

        let (jsonpath_str, select_all) = match selector {
            SelectorStep::Simple(s) => (s.as_str(), false),
            SelectorStep::WithOptions { expr, all } => (expr.as_str(), *all),
        };

        // 使用 JsonPath trait 的 query 方法
        let results = json.query(jsonpath_str).map_err(|e| {
            RuntimeError::Extraction(format!("Invalid JSONPath '{}': {}", jsonpath_str, e))
        })?;

        // 处理结果
        if results.is_empty() {
            Ok(Arc::new(ExtractValueData::Null))
        } else if !select_all && results.len() == 1 {
            Ok(Arc::new(ExtractValueData::from_json(results[0])))
        } else {
            let items: Vec<SharedValue> = results
                .into_iter()
                .map(|v| Arc::new(ExtractValueData::from_json(v)))
                .collect();
            Ok(Arc::new(ExtractValueData::Array(Arc::new(items))))
        }
    }
}
