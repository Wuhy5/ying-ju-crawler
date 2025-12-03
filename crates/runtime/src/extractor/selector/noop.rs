//! # 空操作执行器

use crate::{
    Result,
    context::Context,
    extractor::{ExtractValue, StepExecutor},
};

/// 空操作执行器（占位符）
pub struct NoopExecutor;

impl StepExecutor for NoopExecutor {
    fn execute(&self, input: &ExtractValue, _context: &Context) -> Result<ExtractValue> {
        Ok(input.clone())
    }
}
