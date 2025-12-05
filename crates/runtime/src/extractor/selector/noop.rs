//! # 空操作执行器

use crate::{
    Result,
    context::Context,
    extractor::{SharedValue, value::ExtractValueData},
};
use std::sync::Arc;

/// 空操作执行器（占位符）
pub struct NoopExecutor;

impl NoopExecutor {
    /// 执行空操作，直接返回输入
    pub fn execute(input: &ExtractValueData, _context: &Context) -> Result<SharedValue> {
        Ok(Arc::new(input.clone()))
    }
}
