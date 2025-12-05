//! # 常量值执行器

use crate::{
    Result,
    context::Context,
    extractor::value::{ExtractValueData, SharedValue},
};
use serde_json::Value;
use std::sync::Arc;

/// 常量值执行器
pub struct ConstExecutor;

impl ConstExecutor {
    /// 返回常量值
    pub fn execute(
        value: &Value,
        _input: &ExtractValueData,
        _context: &Context,
    ) -> Result<SharedValue> {
        Ok(Arc::new(ExtractValueData::from_json(value)))
    }
}
