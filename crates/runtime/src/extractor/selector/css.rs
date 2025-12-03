//! # CSS 选择器执行器

use crate::{
    Result,
    context::Context,
    error::RuntimeError,
    extractor::{ExtractValue, StepExecutor},
};
use crawler_schema::SelectorStep;

/// CSS 选择器执行器
pub struct CssSelectorExecutor {
    selector: SelectorStep,
}

impl CssSelectorExecutor {
    pub fn new(selector: SelectorStep) -> Self {
        Self { selector }
    }
}

impl StepExecutor for CssSelectorExecutor {
    fn execute(&self, input: &ExtractValue, _context: &Context) -> Result<ExtractValue> {
        // 获取 HTML 字符串
        let html = match input {
            ExtractValue::String(s) | ExtractValue::Html(s) => s,
            _ => {
                return Err(RuntimeError::Extraction(
                    "CSS selector requires HTML input".to_string(),
                ));
            }
        };

        // TODO: 实现 CSS 选择器逻辑
        // 使用 scraper 或 select.rs
        let _ = html;
        let _ = &self.selector;

        Ok(ExtractValue::String("TODO: CSS selector".to_string()))
    }
}
