//! # 变量执行器

use crate::{
    Result,
    context::Context,
    error::RuntimeError,
    extractor::value::{ExtractValueData, SharedValue},
};
use std::sync::Arc;

/// 变量执行器
pub struct VarExecutor;

impl VarExecutor {
    /// 从上下文获取变量
    pub fn execute(
        var_name: &str,
        _input: &ExtractValueData,
        context: &Context,
    ) -> Result<SharedValue> {
        context
            .get(var_name)
            .map(|v| Arc::new(ExtractValueData::from_json(v)))
            .ok_or_else(|| RuntimeError::Extraction(format!("Variable not found: {}", var_name)))
    }
}
