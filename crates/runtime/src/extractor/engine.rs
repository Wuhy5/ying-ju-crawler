//! # 提取引擎
//!
//! 核心提取逻辑

use crate::{
    Result,
    context::Context,
    error::RuntimeError,
    extractor::{ExtractValue, StepExecutorFactory},
};
use crawler_schema::FieldExtractor;

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

    /// 提取字段
    ///
    /// 执行 FieldExtractor 定义的提取流程
    pub fn extract_field(
        &self,
        extractor: &FieldExtractor,
        input: &ExtractValue,
        context: &Context,
    ) -> Result<ExtractValue> {
        // 执行主步骤链
        match self.execute_steps(&extractor.steps, input, context) {
            Ok(value) => {
                // 检查是否为空
                if value.is_empty() && !extractor.nullable {
                    // 尝试回退
                    if let Some(fallback) = &extractor.fallback {
                        for fallback_steps in fallback {
                            if let Ok(fallback_value) =
                                self.execute_steps(fallback_steps, input, context)
                            {
                                if !fallback_value.is_empty() {
                                    return Ok(fallback_value);
                                }
                            }
                        }
                    }

                    // 使用默认值
                    if let Some(default) = &extractor.default {
                        return Ok(ExtractValue::from_json(default));
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
                            self.execute_steps(fallback_steps, input, context)
                        {
                            if !fallback_value.is_empty() {
                                return Ok(fallback_value);
                            }
                        }
                    }
                }

                // 使用默认值
                if let Some(default) = &extractor.default {
                    return Ok(ExtractValue::from_json(default));
                }

                Err(e)
            }
        }
    }

    /// 执行步骤链
    fn execute_steps(
        &self,
        steps: &[crawler_schema::ExtractStep],
        input: &ExtractValue,
        context: &Context,
    ) -> Result<ExtractValue> {
        let mut current = input.clone();

        for step in steps {
            let executor = StepExecutorFactory::create(step);
            current = executor.execute(&current, context)?;
        }

        Ok(current)
    }
}

impl Default for ExtractEngine {
    fn default() -> Self {
        Self::new()
    }
}
