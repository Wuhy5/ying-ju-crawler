//! # 提取引擎
//!
//! 核心提取逻辑（关键改动：使用引用避免克隆）

use crate::{
    Result,
    context::Context,
    error::RuntimeError,
    extractor::{
        StepExecutorFactory,
        value::{ExtractValueData, SharedValue},
    },
};
use crawler_schema::extract::{ExtractStep, FieldExtractor};
use std::sync::Arc;

/// 提取引擎
///
/// 负责执行字段提取流程
pub struct ExtractEngine {
    // 预留：缓存、优化配置等
}

impl ExtractEngine {
    /// 创建新的提取引擎
    pub fn new() -> Self {
        Self {}
    }

    /// 提取字段（关键改动：仅接收引用）
    ///
    /// 执行 FieldExtractor 定义的提取流程
    /// 所有回退尝试都使用同一个 input 引用，避免多次克隆
    pub fn extract_field(
        extractor: &FieldExtractor,
        input: &ExtractValueData,
        context: &Context,
    ) -> Result<SharedValue> {
        // 执行主步骤链
        match Self::execute_steps(&extractor.steps, input, context) {
            Ok(value) => {
                // 检查是否为空
                if value.is_empty() && !extractor.nullable {
                    // 尝试回退（仍然使用 input 的引用，无克隆）
                    if let Some(fallback) = &extractor.fallback {
                        for fallback_steps in fallback {
                            if let Ok(fallback_value) =
                                Self::execute_steps(fallback_steps, input, context)
                                && !fallback_value.is_empty()
                            {
                                return Ok(fallback_value);
                            }
                        }
                    }

                    // 使用默认值
                    if let Some(default) = &extractor.default {
                        return Ok(Arc::new(ExtractValueData::from_json(default)));
                    }

                    // 如果不允许空值，返回错误
                    return Err(RuntimeError::Extraction(
                        "Field extraction returned empty value".to_string(),
                    ));
                }

                Ok(value)
            }
            Err(e) => {
                // 尝试回退
                if let Some(fallback) = &extractor.fallback {
                    for fallback_steps in fallback {
                        if let Ok(fallback_value) =
                            Self::execute_steps(fallback_steps, input, context)
                            && !fallback_value.is_empty()
                        {
                            return Ok(fallback_value);
                        }
                    }
                }

                // 使用默认值
                if let Some(default) = &extractor.default {
                    return Ok(Arc::new(ExtractValueData::from_json(default)));
                }

                Err(e)
            }
        }
    }

    /// 执行步骤链
    pub(crate) fn execute_steps(
        steps: &[ExtractStep],
        input: &ExtractValueData,
        context: &Context,
    ) -> Result<SharedValue> {
        let mut current = Arc::new(input.clone());

        for step in steps {
            // 直接调用工厂的静态方法，避免创建执行器实例
            current = StepExecutorFactory::execute(step, &current, context)?;
        }

        Ok(current)
    }
}

impl Default for ExtractEngine {
    fn default() -> Self {
        Self::new()
    }
}
