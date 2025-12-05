//! 脚本执行器
//!
//! 提供脚本步骤的执行能力，支持多种脚本引擎

use crate::{
    Result,
    context::Context,
    error::RuntimeError,
    extractor::{SharedValue, value::ExtractValueData},
    script::{ScriptContext, ScriptEngine, ScriptEngineFactory, ScriptLanguage},
};
use crawler_schema::script::{Script, ScriptEngine as SchemaScriptEngine, ScriptSource};
use std::{collections::HashMap, sync::Arc};

/// 脚本执行器
pub struct ScriptExecutor;

impl ScriptExecutor {
    /// 执行脚本步骤
    pub fn execute(
        script: &Script,
        input: &ExtractValueData,
        context: &Context,
    ) -> Result<SharedValue> {
        // 1. 加载脚本代码
        let code = Self::load_script_code(script)?;

        // 2. 获取脚本引擎
        let engine = Self::get_engine(script);

        // 3. 转换输入
        let input_str = Self::value_to_input(input);

        // 4. 构建变量上下文
        let mut variables: HashMap<String, serde_json::Value> = HashMap::new();

        // 添加脚本参数
        if let Some(params) = script.params() {
            for (key, value) in params {
                variables.insert(key.clone(), value.clone());
            }
        }

        // 添加上下文变量
        for (key, value) in context.variables() {
            variables.insert(key.clone(), value.clone());
        }

        // 5. 创建脚本上下文
        let script_context = ScriptContext::new(input_str, variables);

        // 6. 执行脚本
        let result = engine.execute(&code, &script_context)?;

        // 7. 解析输出
        Ok(Self::parse_output(result, input))
    }

    /// 获取脚本使用的引擎
    fn get_engine(script: &Script) -> Arc<dyn ScriptEngine> {
        match script.engine() {
            SchemaScriptEngine::Rhai => ScriptEngineFactory::create(ScriptLanguage::Rhai),
            SchemaScriptEngine::JavaScript => {
                ScriptEngineFactory::create(ScriptLanguage::JavaScript)
            }
            SchemaScriptEngine::Lua => ScriptEngineFactory::create(ScriptLanguage::Lua),
            SchemaScriptEngine::Python => ScriptEngineFactory::create(ScriptLanguage::Python),
        }
    }

    /// 加载脚本代码
    fn load_script_code(script: &Script) -> Result<String> {
        match script.source() {
            ScriptSource::Code(code) => Ok(code.to_string()),
            ScriptSource::Url(url) => {
                // URL 加载需要异步，暂时不支持
                Err(RuntimeError::ScriptRuntime(format!(
                    "从 URL 加载脚本暂未实现: {}",
                    url
                )))
            }
        }
    }

    /// 将 ExtractValueData 转换为脚本输入字符串
    fn value_to_input(value: &ExtractValueData) -> String {
        match value {
            ExtractValueData::String(s) => s.to_string(),
            ExtractValueData::Json(v) => {
                if let Some(s) = v.as_str() {
                    s.to_string()
                } else {
                    v.to_string()
                }
            }
            ExtractValueData::Html(h) => h.to_string(),
            ExtractValueData::Array(arr) => {
                let json_arr: Vec<serde_json::Value> =
                    arr.iter().map(|v| v.to_owned_json()).collect();
                serde_json::to_string(&json_arr).unwrap_or_default()
            }
            ExtractValueData::Null => String::new(),
        }
    }

    /// 解析脚本输出为 ExtractValueData
    fn parse_output(output: String, input: &ExtractValueData) -> SharedValue {
        // 尝试解析为 JSON
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&output) {
            match json {
                serde_json::Value::String(s) => {
                    Arc::new(ExtractValueData::String(Arc::from(s.into_boxed_str())))
                }
                serde_json::Value::Array(arr) => {
                    let items: Vec<SharedValue> = arr
                        .iter()
                        .map(|v| Arc::new(ExtractValueData::from_json(v)))
                        .collect();
                    Arc::new(ExtractValueData::Array(Arc::new(items)))
                }
                serde_json::Value::Null => Arc::new(ExtractValueData::Null),
                other => Arc::new(ExtractValueData::Json(Arc::new(other))),
            }
        } else {
            // 如果不是 JSON，根据输入类型决定输出类型
            match input {
                ExtractValueData::Html(_) => {
                    Arc::new(ExtractValueData::Html(Arc::from(output.into_boxed_str())))
                }
                _ => Arc::new(ExtractValueData::String(Arc::from(output.into_boxed_str()))),
            }
        }
    }
}
