//! # JSON 选择器执行器

use crate::{
    Result,
    context::Context,
    error::RuntimeError,
    extractor::{ExtractValue, StepExecutor},
};
use crawler_schema::SelectorStep;

/// JSON 选择器执行器
pub struct JsonSelectorExecutor {
    selector: SelectorStep,
}

impl JsonSelectorExecutor {
    pub fn new(selector: SelectorStep) -> Self {
        Self { selector }
    }
}

impl StepExecutor for JsonSelectorExecutor {
    fn execute(&self, input: &ExtractValue, _context: &Context) -> Result<ExtractValue> {
        // 获取 JSON 值
        let json = match input {
            ExtractValue::Json(v) => v,
            ExtractValue::String(s) => {
                // 尝试解析 JSON
                &serde_json::from_str(s)
                    .map_err(|e| RuntimeError::Extraction(format!("Failed to parse JSON: {}", e)))?
            }
            _ => {
                return Err(RuntimeError::Extraction(
                    "JSON selector requires JSON input".to_string(),
                ));
            }
        };

        // TODO: 实现 JSONPath 逻辑
        // 使用 jsonpath_lib 或 serde_json::Value 手动遍历
        let _ = json;
        let _ = &self.selector;

        Ok(ExtractValue::String("TODO: JSON selector".to_string()))
    }
}
