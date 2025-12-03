//! # 属性提取器

use crate::{
    Result,
    context::Context,
    error::RuntimeError,
    extractor::{ExtractValue, StepExecutor},
};

/// 属性提取器
pub struct AttrExecutor {
    attr_name: String,
}

impl AttrExecutor {
    pub fn new(attr_name: String) -> Self {
        Self { attr_name }
    }
}

impl StepExecutor for AttrExecutor {
    fn execute(&self, input: &ExtractValue, _context: &Context) -> Result<ExtractValue> {
        // TODO: 实现属性提取逻辑
        // 需要从 HTML 元素中提取属性
        let _ = input;
        let _ = &self.attr_name;

        Err(RuntimeError::Extraction(
            "Attr executor not yet implemented".to_string(),
        ))
    }
}
