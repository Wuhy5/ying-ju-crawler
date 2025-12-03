//! # XPath 选择器执行器

use crate::{
    Result,
    context::Context,
    error::RuntimeError,
    extractor::{ExtractValue, StepExecutor},
};
use crawler_schema::SelectorStep;

/// XPath 选择器执行器
pub struct XpathSelectorExecutor {
    selector: SelectorStep,
}

impl XpathSelectorExecutor {
    pub fn new(selector: SelectorStep) -> Self {
        Self { selector }
    }
}

impl StepExecutor for XpathSelectorExecutor {
    fn execute(&self, input: &ExtractValue, _context: &Context) -> Result<ExtractValue> {
        // 获取 HTML/XML 字符串
        let html = match input {
            ExtractValue::String(s) | ExtractValue::Html(s) => s,
            _ => {
                return Err(RuntimeError::Extraction(
                    "XPath selector requires HTML/XML input".to_string(),
                ));
            }
        };

        // TODO: 实现 XPath 逻辑
        // 可能需要添加依赖：xpath_reader 或 sxd-xpath
        let _ = html;
        let _ = &self.selector;

        Ok(ExtractValue::String("TODO: XPath selector".to_string()))
    }
}
