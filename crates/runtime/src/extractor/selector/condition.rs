//! # 条件执行器
//!
//! 根据条件选择执行不同的提取逻辑

use crate::{
    Result,
    context::Context,
    extractor::{
        StepExecutorFactory,
        value::{ExtractValueData, SharedValue},
    },
};
use crawler_schema::extract::{ConditionStep, ExtractStep};
use std::sync::Arc;

/// 条件执行器
pub struct ConditionExecutor;

impl ConditionExecutor {
    /// 执行条件分支
    pub fn execute(
        condition: &ConditionStep,
        input: &ExtractValueData,
        context: &Context,
    ) -> Result<SharedValue> {
        if Self::evaluate_condition(&condition.when, input, context) {
            // 条件为真，执行 then 步骤
            Self::execute_steps(&condition.then, input, context)
        } else if let Some(otherwise) = &condition.otherwise {
            // 条件为假，执行 otherwise 步骤
            Self::execute_steps(otherwise, input, context)
        } else {
            // 没有 otherwise，返回原输入
            Ok(Arc::new(input.clone()))
        }
    }

    /// 执行一系列步骤
    fn execute_steps(
        steps: &[ExtractStep],
        input: &ExtractValueData,
        context: &Context,
    ) -> Result<SharedValue> {
        let mut current = Arc::new(input.clone());

        for step in steps {
            current = StepExecutorFactory::execute(step, &current, context)?;
        }

        Ok(current)
    }

    /// 判断条件是否为真
    ///
    /// 执行 `when` 步骤，如果结果非空/非 null/非 false，则为真
    fn evaluate_condition(
        steps: &[ExtractStep],
        input: &ExtractValueData,
        context: &Context,
    ) -> bool {
        match Self::execute_steps(steps, input, context) {
            Ok(result) => result.is_truthy(),
            Err(_) => false,
        }
    }
}
