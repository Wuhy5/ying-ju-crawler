//! # 变量执行器

use crate::{
    Result,
    context::Context,
    error::RuntimeError,
    extractor::{ExtractValue, StepExecutor},
};

/// 变量执行器
pub struct VarExecutor {
    var_name: String,
}

impl VarExecutor {
    pub fn new(var_name: String) -> Self {
        Self { var_name }
    }
}

impl StepExecutor for VarExecutor {
    fn execute(&self, _input: &ExtractValue, context: &Context) -> Result<ExtractValue> {
        context
            .get(&self.var_name)
            .map(|v| ExtractValue::from_json(v))
            .ok_or_else(|| {
                RuntimeError::Extraction(format!("Variable not found: {}", self.var_name))
            })
    }
}
