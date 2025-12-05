//! # 映射执行器
//!
//! 对数组每个元素应用步骤

use crate::{
    Result,
    context::Context,
    error::RuntimeError,
    extractor::{
        StepExecutorFactory,
        value::{ExtractValueData, SharedValue},
    },
};
use crawler_schema::extract::ExtractStep;
use std::sync::Arc;

/// 映射执行器
pub struct MapExecutor;

impl MapExecutor {
    /// 执行映射
    pub fn execute(
        steps: &[ExtractStep],
        input: &ExtractValueData,
        context: &Context,
    ) -> Result<SharedValue> {
        match input {
            ExtractValueData::Array(arr) => {
                let results: Vec<SharedValue> = arr
                    .iter()
                    .filter_map(|item| Self::execute_steps(steps, item, context).ok())
                    .collect();

                Ok(Arc::new(ExtractValueData::Array(Arc::new(results))))
            }
            _ => {
                // 非数组输入，直接应用步骤
                Err(RuntimeError::Extraction(
                    "Map step requires array input".to_string(),
                ))
            }
        }
    }

    /// 对单个值执行所有步骤
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
}
