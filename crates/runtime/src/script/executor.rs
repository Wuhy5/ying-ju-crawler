//! 脚本执行器 (StepExecutor 实现)

use crate::{
    Result,
    context::Context,
    extractor::{ExtractValue, StepExecutor},
    script::{ScriptContext, ScriptEngine},
};
use crawler_schema::script::{Script, ScriptSource};
use std::sync::Arc;

/// 脚本步骤执行器
pub struct ScriptExecutor {
    /// 脚本引擎实例
    engine: Arc<dyn ScriptEngine>,
}

impl ScriptExecutor {
    /// 创建新的脚本执行器
    pub fn new() -> Self {
        Self {
            engine: crate::script::ScriptEngineFactory::create_default(),
        }
    }

    /// 使用自定义引擎创建
    pub fn with_engine(engine: Arc<dyn ScriptEngine>) -> Self {
        Self { engine }
    }
}

impl Default for ScriptExecutor {
    fn default() -> Self {
        Self::new()
    }
}

impl StepExecutor for ScriptExecutor {
    fn execute(&self, input: &ExtractValue, _context: &Context) -> Result<ExtractValue> {
        // TODO: 将 ExtractValue 转换为脚本可用的格式
        // TODO: 从 ScriptStep 中提取脚本内容并执行
        // 目前简单返回输入的克隆
        Ok(input.clone())
    }
}

/// 执行单个脚本步骤
pub fn execute_script_step(
    step: &Script,
    input: ExtractValue,
    _context: &Context,
    engine: &Arc<dyn ScriptEngine>,
) -> Result<ExtractValue> {
    // 解析脚本步骤配置，获取脚本代码
    let script = match step.source() {
        ScriptSource::Code(code) => code,
        ScriptSource::File(path) => {
            // TODO: 从文件加载脚本
            return Err(crate::error::RuntimeError::ScriptRuntime(format!(
                "从文件加载脚本暂未实现: {}",
                path
            )));
        }
        ScriptSource::Url(url) => {
            // TODO: 从 URL 加载脚本
            return Err(crate::error::RuntimeError::ScriptRuntime(format!(
                "从 URL 加载脚本暂未实现: {}",
                url
            )));
        }
    };

    // TODO: 将输入转换为字符串(需要根据 ExtractValue 的实际定义实现)
    let input_str = match &input {
        ExtractValue::String(s) => s.clone(),
        ExtractValue::Json(v) => v.to_string(),
        ExtractValue::Html(h) => h.clone(),
        ExtractValue::Array(_) => {
            // TODO: 数组序列化
            String::new()
        }
        ExtractValue::Null => String::new(),
    };

    // 创建脚本上下文，包含参数
    let mut variables = std::collections::HashMap::new();
    if let Some(params) = step.params() {
        for (key, value) in params {
            variables.insert(key.clone(), value.clone());
        }
    }

    let script_context = ScriptContext::new(input_str, variables);

    // 执行脚本
    let result = engine.execute(&script, &script_context)?;

    // 返回字符串结果
    Ok(ExtractValue::String(result))
}
